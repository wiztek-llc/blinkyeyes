use crate::db;
use crate::state::{AnalyticsSummary, DailyStats};
use chrono::{NaiveDate, Utc};
use rusqlite::{Connection, Result as SqlResult};

/// Build the full analytics summary for the dashboard.
///
/// `daily_goal` is the user's configured breaks-per-day target, used for streak calculations.
pub fn build_analytics_summary(conn: &Connection, daily_goal: u32) -> SqlResult<AnalyticsSummary> {
    let today = Utc::now().date_naive();
    let today_str = today.format("%Y-%m-%d").to_string();

    // 1. Always recompute today's stats fresh (the day is still in progress)
    let today_stats = db::recompute_daily_stats(conn, &today_str)?;

    // 2. Last 7 days (today - 6 days through today), zero-filled
    let seven_days_ago = today - chrono::Duration::days(6);
    let last_7_days = db::get_daily_stats_range(
        conn,
        &seven_days_ago.format("%Y-%m-%d").to_string(),
        &today_str,
    )?;

    // 3. Last 30 days (today - 29 days through today), zero-filled
    let thirty_days_ago = today - chrono::Duration::days(29);
    let last_30_days = db::get_daily_stats_range(
        conn,
        &thirty_days_ago.format("%Y-%m-%d").to_string(),
        &today_str,
    )?;

    // 4. Current day streak: consecutive days (ending at today or yesterday) meeting the goal
    let current_day_streak = compute_current_streak(conn, daily_goal, &today, &today_stats)?;

    // 5. Best day streak: all-time longest consecutive run meeting the goal
    let best_day_streak = compute_best_streak(conn, daily_goal)?;

    // 6. Lifetime totals: only count completed breaks
    let (lifetime_breaks, lifetime_rest_seconds) = compute_lifetime_totals(conn)?;

    Ok(AnalyticsSummary {
        today: today_stats,
        last_7_days,
        last_30_days,
        current_day_streak,
        best_day_streak,
        lifetime_breaks,
        lifetime_rest_seconds,
    })
}

/// Walk backwards from today counting consecutive days where daily_goal was met.
///
/// Special case: if today hasn't met the goal *yet*, don't break the streak —
/// the day isn't over. Start counting from yesterday instead, and if today
/// has met the goal, include it.
fn compute_current_streak(
    conn: &Connection,
    daily_goal: u32,
    today: &NaiveDate,
    today_stats: &DailyStats,
) -> SqlResult<u32> {
    let today_met_goal = today_stats.breaks_completed >= daily_goal;

    // Load all cached daily stats, ordered by date descending
    let mut stmt = conn.prepare(
        "SELECT date, breaks_completed FROM daily_stats_cache ORDER BY date DESC",
    )?;

    let entries: Vec<(String, u32)> = stmt
        .query_map([], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, i32>(1)? as u32))
        })?
        .collect::<SqlResult<Vec<_>>>()?;

    // Build a set for O(1) lookup
    let stats_map: std::collections::HashMap<String, u32> = entries.into_iter().collect();

    let mut streak: u32 = 0;

    if today_met_goal {
        // Today counts, start walking from today
        streak = 1;
        let mut day = *today - chrono::Duration::days(1);
        loop {
            let day_str = day.format("%Y-%m-%d").to_string();
            match stats_map.get(&day_str) {
                Some(&completed) if completed >= daily_goal => {
                    streak += 1;
                    day -= chrono::Duration::days(1);
                }
                _ => break,
            }
        }
    } else {
        // Today hasn't met goal yet — don't break streak, start from yesterday
        let mut day = *today - chrono::Duration::days(1);
        loop {
            let day_str = day.format("%Y-%m-%d").to_string();
            match stats_map.get(&day_str) {
                Some(&completed) if completed >= daily_goal => {
                    streak += 1;
                    day -= chrono::Duration::days(1);
                }
                _ => break,
            }
        }
    }

    Ok(streak)
}

/// Scan all daily_stats_cache entries to find the longest run of consecutive days
/// where breaks_completed >= daily_goal.
fn compute_best_streak(conn: &Connection, daily_goal: u32) -> SqlResult<u32> {
    let mut stmt = conn.prepare(
        "SELECT date, breaks_completed FROM daily_stats_cache ORDER BY date ASC",
    )?;

    let entries: Vec<(NaiveDate, u32)> = stmt
        .query_map([], |row| {
            let date_str: String = row.get(0)?;
            let completed = row.get::<_, i32>(1)? as u32;
            Ok((date_str, completed))
        })?
        .filter_map(|r| {
            r.ok().and_then(|(date_str, completed)| {
                NaiveDate::parse_from_str(&date_str, "%Y-%m-%d")
                    .ok()
                    .map(|d| (d, completed))
            })
        })
        .collect();

    if entries.is_empty() {
        return Ok(0);
    }

    let mut best: u32 = 0;
    let mut current: u32 = 0;
    let mut prev_date: Option<NaiveDate> = None;

    for (date, completed) in &entries {
        if *completed >= daily_goal {
            // Check if this day is consecutive with previous
            let is_consecutive = match prev_date {
                Some(prev) => *date == prev + chrono::Duration::days(1),
                None => true, // first entry starts a streak
            };

            if is_consecutive {
                current += 1;
            } else {
                current = 1;
            }

            if current > best {
                best = current;
            }
            prev_date = Some(*date);
        } else {
            current = 0;
            prev_date = Some(*date);
        }
    }

    Ok(best)
}

/// Get lifetime totals: count and total rest seconds for completed breaks only.
fn compute_lifetime_totals(conn: &Connection) -> SqlResult<(u64, u64)> {
    let result = conn.query_row(
        "SELECT COALESCE(COUNT(*), 0), COALESCE(SUM(duration_seconds), 0)
         FROM break_records WHERE completed = 1",
        [],
        |row| {
            let count = row.get::<_, i64>(0)? as u64;
            let seconds = row.get::<_, i64>(1)? as u64;
            Ok((count, seconds))
        },
    )?;
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::{init_db_conn, insert_break_record, update_break_completion};

    fn setup_test_db() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        init_db_conn(&conn).unwrap();
        conn
    }

    #[test]
    fn test_zero_data_returns_valid_struct() {
        let conn = setup_test_db();
        let summary = build_analytics_summary(&conn, 24).unwrap();

        assert_eq!(summary.today.breaks_completed, 0);
        assert_eq!(summary.today.breaks_skipped, 0);
        assert_eq!(summary.today.compliance_rate, 0.0);
        assert_eq!(summary.last_7_days.len(), 7);
        assert_eq!(summary.last_30_days.len(), 30);
        assert_eq!(summary.current_day_streak, 0);
        assert_eq!(summary.best_day_streak, 0);
        assert_eq!(summary.lifetime_breaks, 0);
        assert_eq!(summary.lifetime_rest_seconds, 0);
    }

    #[test]
    fn test_last_7_days_always_7_elements() {
        let conn = setup_test_db();
        // Insert a single break today
        let today = Utc::now().date_naive();
        let base = today
            .and_hms_opt(10, 0, 0)
            .unwrap()
            .and_utc()
            .timestamp_millis() as u64;

        let id = insert_break_record(&conn, base, 1200).unwrap();
        update_break_completion(&conn, id, 20, true, false).unwrap();

        let summary = build_analytics_summary(&conn, 24).unwrap();
        assert_eq!(summary.last_7_days.len(), 7);

        // Only today should have data
        let last = &summary.last_7_days[6]; // today is the last entry
        assert_eq!(last.breaks_completed, 1);

        // Earlier days should be zero
        for day in &summary.last_7_days[..6] {
            assert_eq!(day.breaks_completed, 0);
        }
    }

    #[test]
    fn test_last_30_days_always_30_elements() {
        let conn = setup_test_db();
        let summary = build_analytics_summary(&conn, 24).unwrap();
        assert_eq!(summary.last_30_days.len(), 30);
    }

    #[test]
    fn test_compliance_rate_with_mixed_breaks() {
        let conn = setup_test_db();
        let today = Utc::now().date_naive();
        let base = today
            .and_hms_opt(10, 0, 0)
            .unwrap()
            .and_utc()
            .timestamp_millis() as u64;

        // 3 completed, 1 skipped → compliance ≈ 0.75
        for i in 0..3 {
            let id = insert_break_record(&conn, base + i * 1_200_000, 1200).unwrap();
            update_break_completion(&conn, id, 20, true, false).unwrap();
        }
        let id = insert_break_record(&conn, base + 3_600_000, 1200).unwrap();
        update_break_completion(&conn, id, 5, false, true).unwrap();

        let summary = build_analytics_summary(&conn, 24).unwrap();
        assert_eq!(summary.today.breaks_completed, 3);
        assert_eq!(summary.today.breaks_skipped, 1);
        assert!((summary.today.compliance_rate - 0.75).abs() < 0.001);
    }

    #[test]
    fn test_current_streak_with_consecutive_days() {
        let conn = setup_test_db();
        let today = Utc::now().date_naive();
        let daily_goal = 2;

        // Seed 3 consecutive days (yesterday, day before, day before that) each meeting the goal
        for days_ago in 1..=3 {
            let date = today - chrono::Duration::days(days_ago);
            let base = date
                .and_hms_opt(10, 0, 0)
                .unwrap()
                .and_utc()
                .timestamp_millis() as u64;

            for j in 0..daily_goal {
                let id = insert_break_record(&conn, base + j as u64 * 1_200_000, 1200).unwrap();
                update_break_completion(&conn, id, 20, true, false).unwrap();
            }
            // Force cache computation for that day
            db::recompute_daily_stats(&conn, &date.format("%Y-%m-%d").to_string()).unwrap();
        }

        // Today has 0 breaks (hasn't met goal yet) — streak should still be 3
        let summary = build_analytics_summary(&conn, daily_goal).unwrap();
        assert_eq!(summary.current_day_streak, 3);
    }

    #[test]
    fn test_current_streak_includes_today_when_goal_met() {
        let conn = setup_test_db();
        let today = Utc::now().date_naive();
        let daily_goal = 2;

        // Today meets the goal
        let base = today
            .and_hms_opt(10, 0, 0)
            .unwrap()
            .and_utc()
            .timestamp_millis() as u64;
        for j in 0..daily_goal {
            let id = insert_break_record(&conn, base + j as u64 * 1_200_000, 1200).unwrap();
            update_break_completion(&conn, id, 20, true, false).unwrap();
        }

        // Yesterday also met the goal
        let yesterday = today - chrono::Duration::days(1);
        let ybase = yesterday
            .and_hms_opt(10, 0, 0)
            .unwrap()
            .and_utc()
            .timestamp_millis() as u64;
        for j in 0..daily_goal {
            let id = insert_break_record(&conn, ybase + j as u64 * 1_200_000, 1200).unwrap();
            update_break_completion(&conn, id, 20, true, false).unwrap();
        }
        db::recompute_daily_stats(&conn, &yesterday.format("%Y-%m-%d").to_string()).unwrap();

        let summary = build_analytics_summary(&conn, daily_goal).unwrap();
        assert_eq!(summary.current_day_streak, 2); // today + yesterday
    }

    #[test]
    fn test_streak_resets_on_gap() {
        let conn = setup_test_db();
        let today = Utc::now().date_naive();
        let daily_goal = 1;

        // 3 days ago: met goal
        let d3 = today - chrono::Duration::days(3);
        let base3 = d3.and_hms_opt(10, 0, 0).unwrap().and_utc().timestamp_millis() as u64;
        let id = insert_break_record(&conn, base3, 1200).unwrap();
        update_break_completion(&conn, id, 20, true, false).unwrap();
        db::recompute_daily_stats(&conn, &d3.format("%Y-%m-%d").to_string()).unwrap();

        // 2 days ago: zero breaks (gap!)
        let d2 = today - chrono::Duration::days(2);
        db::recompute_daily_stats(&conn, &d2.format("%Y-%m-%d").to_string()).unwrap();

        // Yesterday: met goal
        let d1 = today - chrono::Duration::days(1);
        let base1 = d1.and_hms_opt(10, 0, 0).unwrap().and_utc().timestamp_millis() as u64;
        let id = insert_break_record(&conn, base1, 1200).unwrap();
        update_break_completion(&conn, id, 20, true, false).unwrap();
        db::recompute_daily_stats(&conn, &d1.format("%Y-%m-%d").to_string()).unwrap();

        // Today: 0 breaks
        let summary = build_analytics_summary(&conn, daily_goal).unwrap();
        // Current streak should be 1 (only yesterday), not 2 (gap breaks it)
        assert_eq!(summary.current_day_streak, 1);
    }

    #[test]
    fn test_best_streak_across_history() {
        let conn = setup_test_db();
        let today = Utc::now().date_naive();
        let daily_goal = 1;

        // Create a 5-day streak ending 10 days ago
        for days_ago in 10..=14 {
            let date = today - chrono::Duration::days(days_ago);
            let base = date.and_hms_opt(10, 0, 0).unwrap().and_utc().timestamp_millis() as u64;
            let id = insert_break_record(&conn, base, 1200).unwrap();
            update_break_completion(&conn, id, 20, true, false).unwrap();
            db::recompute_daily_stats(&conn, &date.format("%Y-%m-%d").to_string()).unwrap();
        }

        // Create a 2-day streak ending yesterday
        for days_ago in 1..=2 {
            let date = today - chrono::Duration::days(days_ago);
            let base = date.and_hms_opt(10, 0, 0).unwrap().and_utc().timestamp_millis() as u64;
            let id = insert_break_record(&conn, base, 1200).unwrap();
            update_break_completion(&conn, id, 20, true, false).unwrap();
            db::recompute_daily_stats(&conn, &date.format("%Y-%m-%d").to_string()).unwrap();
        }

        let summary = build_analytics_summary(&conn, daily_goal).unwrap();
        assert_eq!(summary.best_day_streak, 5);
        assert_eq!(summary.current_day_streak, 2);
    }

    #[test]
    fn test_lifetime_totals_only_count_completed() {
        let conn = setup_test_db();
        let today = Utc::now().date_naive();
        let base = today
            .and_hms_opt(10, 0, 0)
            .unwrap()
            .and_utc()
            .timestamp_millis() as u64;

        // 3 completed (20s each), 2 skipped (5s each)
        for i in 0..3 {
            let id = insert_break_record(&conn, base + i * 1_200_000, 1200).unwrap();
            update_break_completion(&conn, id, 20, true, false).unwrap();
        }
        for i in 3..5 {
            let id = insert_break_record(&conn, base + i * 1_200_000, 1200).unwrap();
            update_break_completion(&conn, id, 5, false, true).unwrap();
        }

        let summary = build_analytics_summary(&conn, 24).unwrap();
        assert_eq!(summary.lifetime_breaks, 3);
        assert_eq!(summary.lifetime_rest_seconds, 60); // 3 * 20
    }

    #[test]
    fn test_performance_with_1000_records() {
        let conn = setup_test_db();
        let today = Utc::now().date_naive();

        // Insert 1000+ records spread across 30 days
        for days_ago in 0..30 {
            let date = today - chrono::Duration::days(days_ago);
            let base = date.and_hms_opt(8, 0, 0).unwrap().and_utc().timestamp_millis() as u64;

            for j in 0..35 {
                let id = insert_break_record(&conn, base + j * 1_200_000, 1200).unwrap();
                if j % 4 != 0 {
                    update_break_completion(&conn, id, 20, true, false).unwrap();
                } else {
                    update_break_completion(&conn, id, 5, false, true).unwrap();
                }
            }
            // Pre-cache non-today stats
            if days_ago > 0 {
                db::recompute_daily_stats(&conn, &date.format("%Y-%m-%d").to_string()).unwrap();
            }
        }

        // Total: 30 * 35 = 1050 records
        let start = std::time::Instant::now();
        let summary = build_analytics_summary(&conn, 24).unwrap();
        let elapsed = start.elapsed();

        assert!(elapsed.as_millis() < 50, "Took {}ms, expected <50ms", elapsed.as_millis());
        assert_eq!(summary.last_7_days.len(), 7);
        assert_eq!(summary.last_30_days.len(), 30);
        assert!(summary.lifetime_breaks > 0);
    }

    #[test]
    fn test_best_streak_with_no_data() {
        let conn = setup_test_db();
        let best = compute_best_streak(&conn, 24).unwrap();
        assert_eq!(best, 0);
    }

    #[test]
    fn test_best_streak_handles_non_consecutive_cache_entries() {
        let conn = setup_test_db();
        let today = Utc::now().date_naive();
        let daily_goal = 1;

        // Day 1 and Day 3 have data, Day 2 is missing from cache
        // This tests that compute_best_streak handles gaps (non-consecutive dates)
        let d1 = today - chrono::Duration::days(5);
        let d3 = today - chrono::Duration::days(3);

        let base1 = d1.and_hms_opt(10, 0, 0).unwrap().and_utc().timestamp_millis() as u64;
        let id = insert_break_record(&conn, base1, 1200).unwrap();
        update_break_completion(&conn, id, 20, true, false).unwrap();
        db::recompute_daily_stats(&conn, &d1.format("%Y-%m-%d").to_string()).unwrap();

        let base3 = d3.and_hms_opt(10, 0, 0).unwrap().and_utc().timestamp_millis() as u64;
        let id = insert_break_record(&conn, base3, 1200).unwrap();
        update_break_completion(&conn, id, 20, true, false).unwrap();
        db::recompute_daily_stats(&conn, &d3.format("%Y-%m-%d").to_string()).unwrap();

        let best = compute_best_streak(&conn, daily_goal).unwrap();
        // Two entries but not consecutive → best is 1
        assert_eq!(best, 1);
    }
}
