use crate::state::{BreakRecord, DailyStats, UserSettings};
use chrono::{NaiveDate, Utc};
use rusqlite::{params, Connection, Result as SqlResult};
use std::path::PathBuf;
use std::sync::Mutex;

const MIGRATION_001_SQL: &str = include_str!("../migrations/001_initial.sql");
const MIGRATION_002_SQL: &str = include_str!("../migrations/002_onboarding.sql");

/// Returns the OS-specific path for the blinky database directory.
pub fn get_db_dir() -> PathBuf {
    let base = dirs::data_dir().unwrap_or_else(|| PathBuf::from("."));
    base.join("com.blinky.app")
}

/// Opens (or creates) the database, enables WAL mode, runs migrations.
/// Returns a Mutex-wrapped connection suitable for Tauri state management.
pub fn init_db(db_path: &str) -> SqlResult<Mutex<Connection>> {
    let conn = Connection::open(db_path)?;

    // Enable WAL for concurrent read/write without blocking
    conn.execute_batch("PRAGMA journal_mode=WAL;")?;
    conn.execute_batch("PRAGMA foreign_keys=ON;")?;

    run_migrations(&conn)?;

    Ok(Mutex::new(conn))
}

/// Same as init_db but takes a Connection directly (for testing with :memory:).
#[cfg(test)]
pub fn init_db_conn(conn: &Connection) -> SqlResult<()> {
    conn.execute_batch("PRAGMA journal_mode=WAL;")?;
    conn.execute_batch("PRAGMA foreign_keys=ON;")?;
    run_migrations(conn)?;
    Ok(())
}

fn run_migrations(conn: &Connection) -> SqlResult<()> {
    // Check if _migrations table exists
    let has_migrations_table: bool = conn
        .prepare("SELECT name FROM sqlite_master WHERE type='table' AND name='_migrations'")?
        .exists([])?;

    let has_001 = if has_migrations_table {
        conn.prepare("SELECT id FROM _migrations WHERE name = '001_initial'")?
            .exists([])?
    } else {
        false
    };

    if !has_001 {
        conn.execute_batch(MIGRATION_001_SQL)?;
        conn.execute(
            "INSERT INTO _migrations (name) VALUES (?1)",
            params!["001_initial"],
        )?;
    }

    // Migration 002: onboarding columns
    let has_002 = conn
        .prepare("SELECT id FROM _migrations WHERE name = '002_onboarding'")?
        .exists([])?;

    if !has_002 {
        conn.execute_batch(MIGRATION_002_SQL)?;
        conn.execute(
            "INSERT INTO _migrations (name) VALUES (?1)",
            params!["002_onboarding"],
        )?;

        // Auto-complete onboarding for existing users who already have break records.
        let has_breaks: bool = conn
            .prepare("SELECT id FROM break_records LIMIT 1")?
            .exists([])?;
        if has_breaks {
            let now_ms = chrono::Utc::now().timestamp_millis();
            conn.execute(
                "UPDATE settings SET onboarding_completed = 1, first_break_completed = 1, onboarding_completed_at = ?1 WHERE id = 1",
                params![now_ms],
            )?;
        }
    }

    Ok(())
}

/// Insert a new break record when a break starts. Returns the row ID.
pub fn insert_break_record(
    conn: &Connection,
    started_at: u64,
    preceding_work_seconds: u32,
) -> SqlResult<i64> {
    conn.execute(
        "INSERT INTO break_records (started_at, duration_seconds, completed, skipped, preceding_work_seconds)
         VALUES (?1, 0, 0, 0, ?2)",
        params![started_at as i64, preceding_work_seconds],
    )?;
    Ok(conn.last_insert_rowid())
}

/// Update a break record when the break ends (completed or skipped).
pub fn update_break_completion(
    conn: &Connection,
    id: i64,
    duration_seconds: u32,
    completed: bool,
    skipped: bool,
) -> SqlResult<()> {
    conn.execute(
        "UPDATE break_records SET duration_seconds = ?1, completed = ?2, skipped = ?3 WHERE id = ?4",
        params![duration_seconds, completed as i32, skipped as i32, id],
    )?;
    Ok(())
}

/// Get break records, paginated, newest first.
pub fn get_break_records(
    conn: &Connection,
    limit: u32,
    offset: u32,
) -> SqlResult<Vec<BreakRecord>> {
    let mut stmt = conn.prepare(
        "SELECT id, started_at, duration_seconds, completed, skipped, preceding_work_seconds
         FROM break_records ORDER BY started_at DESC LIMIT ?1 OFFSET ?2",
    )?;
    let rows = stmt.query_map(params![limit, offset], |row| {
        Ok(BreakRecord {
            id: row.get(0)?,
            started_at: row.get::<_, i64>(1)? as u64,
            duration_seconds: row.get::<_, i32>(2)? as u32,
            completed: row.get::<_, i32>(3)? != 0,
            skipped: row.get::<_, i32>(4)? != 0,
            preceding_work_seconds: row.get::<_, i32>(5)? as u32,
        })
    })?;
    rows.collect()
}

/// Load settings from the single-row settings table.
pub fn load_settings(conn: &Connection) -> SqlResult<UserSettings> {
    conn.query_row(
        "SELECT work_interval_minutes, break_duration_seconds, sound_enabled, sound_volume,
                notification_enabled, overlay_enabled, launch_at_login, daily_goal,
                idle_pause_minutes, theme,
                onboarding_completed, onboarding_completed_at, tooltips_seen, first_break_completed
         FROM settings WHERE id = 1",
        [],
        |row| {
            Ok(UserSettings {
                work_interval_minutes: row.get::<_, i32>(0)? as u32,
                break_duration_seconds: row.get::<_, i32>(1)? as u32,
                sound_enabled: row.get::<_, i32>(2)? != 0,
                sound_volume: row.get::<_, f64>(3)? as f32,
                notification_enabled: row.get::<_, i32>(4)? != 0,
                overlay_enabled: row.get::<_, i32>(5)? != 0,
                launch_at_login: row.get::<_, i32>(6)? != 0,
                daily_goal: row.get::<_, i32>(7)? as u32,
                idle_pause_minutes: row.get::<_, i32>(8)? as u32,
                theme: row.get(9)?,
                onboarding_completed: row.get::<_, i32>(10)? != 0,
                onboarding_completed_at: row.get::<_, Option<i64>>(11)?.map(|v| v as u64),
                tooltips_seen: row.get(12)?,
                first_break_completed: row.get::<_, i32>(13)? != 0,
            })
        },
    )
}

/// Save settings to the single-row settings table.
pub fn save_settings(conn: &Connection, s: &UserSettings) -> SqlResult<()> {
    conn.execute(
        "UPDATE settings SET
            work_interval_minutes = ?1,
            break_duration_seconds = ?2,
            sound_enabled = ?3,
            sound_volume = ?4,
            notification_enabled = ?5,
            overlay_enabled = ?6,
            launch_at_login = ?7,
            daily_goal = ?8,
            idle_pause_minutes = ?9,
            theme = ?10,
            onboarding_completed = ?11,
            onboarding_completed_at = ?12,
            tooltips_seen = ?13,
            first_break_completed = ?14
         WHERE id = 1",
        params![
            s.work_interval_minutes as i32,
            s.break_duration_seconds as i32,
            s.sound_enabled as i32,
            s.sound_volume as f64,
            s.notification_enabled as i32,
            s.overlay_enabled as i32,
            s.launch_at_login as i32,
            s.daily_goal as i32,
            s.idle_pause_minutes as i32,
            s.theme,
            s.onboarding_completed as i32,
            s.onboarding_completed_at.map(|v| v as i64),
            s.tooltips_seen,
            s.first_break_completed as i32,
        ],
    )?;
    Ok(())
}

/// Recompute daily stats for a given date (YYYY-MM-DD) from break_records.
/// Upserts the result into daily_stats_cache and returns the computed stats.
pub fn recompute_daily_stats(conn: &Connection, date: &str) -> SqlResult<DailyStats> {
    let parsed = NaiveDate::parse_from_str(date, "%Y-%m-%d")
        .map_err(|e| rusqlite::Error::InvalidParameterName(format!("bad date: {}", e)))?;

    let start_of_day = parsed
        .and_hms_opt(0, 0, 0)
        .unwrap()
        .and_utc()
        .timestamp_millis() as u64;
    let end_of_day = parsed
        .and_hms_opt(23, 59, 59)
        .unwrap()
        .and_utc()
        .timestamp_millis() as u64
        + 999; // include the last millisecond

    // Fetch all break records for this day
    let mut stmt = conn.prepare(
        "SELECT completed, skipped, duration_seconds
         FROM break_records
         WHERE started_at >= ?1 AND started_at <= ?2
         ORDER BY started_at ASC",
    )?;

    let records: Vec<(bool, bool, u32)> = stmt
        .query_map(params![start_of_day as i64, end_of_day as i64], |row| {
            Ok((
                row.get::<_, i32>(0)? != 0,
                row.get::<_, i32>(1)? != 0,
                row.get::<_, i32>(2)? as u32,
            ))
        })?
        .collect::<SqlResult<Vec<_>>>()?;

    let mut breaks_completed: u32 = 0;
    let mut breaks_skipped: u32 = 0;
    let mut total_rest_seconds: u32 = 0;
    let mut longest_streak: u32 = 0;
    let mut current_streak: u32 = 0;

    for (completed, skipped, duration) in &records {
        if *completed && !*skipped {
            breaks_completed += 1;
            total_rest_seconds += duration;
            current_streak += 1;
            if current_streak > longest_streak {
                longest_streak = current_streak;
            }
        } else if *skipped {
            breaks_skipped += 1;
            current_streak = 0;
        } else {
            // in-progress or incomplete — don't count either way
            current_streak = 0;
        }
    }

    let total = breaks_completed + breaks_skipped;
    let compliance_rate = if total == 0 {
        0.0
    } else {
        breaks_completed as f64 / total as f64
    };

    let stats = DailyStats {
        date: date.to_string(),
        breaks_completed,
        breaks_skipped,
        total_rest_seconds,
        longest_streak,
        compliance_rate,
    };

    // Upsert into cache
    conn.execute(
        "INSERT INTO daily_stats_cache (date, breaks_completed, breaks_skipped, total_rest_seconds, longest_streak, compliance_rate)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6)
         ON CONFLICT(date) DO UPDATE SET
            breaks_completed = excluded.breaks_completed,
            breaks_skipped = excluded.breaks_skipped,
            total_rest_seconds = excluded.total_rest_seconds,
            longest_streak = excluded.longest_streak,
            compliance_rate = excluded.compliance_rate",
        params![
            stats.date,
            stats.breaks_completed as i32,
            stats.breaks_skipped as i32,
            stats.total_rest_seconds as i32,
            stats.longest_streak as i32,
            stats.compliance_rate,
        ],
    )?;

    Ok(stats)
}

/// Get daily stats for a date range. Always recomputes today's entry.
/// Returns entries for each day in the range, zero-filled for missing days.
pub fn get_daily_stats_range(
    conn: &Connection,
    from: &str,
    to: &str,
) -> SqlResult<Vec<DailyStats>> {
    let from_date = NaiveDate::parse_from_str(from, "%Y-%m-%d")
        .map_err(|e| rusqlite::Error::InvalidParameterName(format!("bad from date: {}", e)))?;
    let to_date = NaiveDate::parse_from_str(to, "%Y-%m-%d")
        .map_err(|e| rusqlite::Error::InvalidParameterName(format!("bad to date: {}", e)))?;

    let today = Utc::now().format("%Y-%m-%d").to_string();

    // Recompute today if it falls within the range
    let today_date = NaiveDate::parse_from_str(&today, "%Y-%m-%d").unwrap();
    if today_date >= from_date && today_date <= to_date {
        let _ = recompute_daily_stats(conn, &today);
    }

    // Load all cached entries in range
    let mut stmt = conn.prepare(
        "SELECT date, breaks_completed, breaks_skipped, total_rest_seconds, longest_streak, compliance_rate
         FROM daily_stats_cache
         WHERE date >= ?1 AND date <= ?2
         ORDER BY date ASC",
    )?;

    let cached: Vec<DailyStats> = stmt
        .query_map(params![from, to], |row| {
            Ok(DailyStats {
                date: row.get(0)?,
                breaks_completed: row.get::<_, i32>(1)? as u32,
                breaks_skipped: row.get::<_, i32>(2)? as u32,
                total_rest_seconds: row.get::<_, i32>(3)? as u32,
                longest_streak: row.get::<_, i32>(4)? as u32,
                compliance_rate: row.get(5)?,
            })
        })?
        .collect::<SqlResult<Vec<_>>>()?;

    // Build a map for O(1) lookup
    let cache_map: std::collections::HashMap<String, DailyStats> =
        cached.into_iter().map(|s| (s.date.clone(), s)).collect();

    // Fill in every day in the range
    let mut result = Vec::new();
    let mut current = from_date;
    while current <= to_date {
        let date_str = current.format("%Y-%m-%d").to_string();
        if let Some(stats) = cache_map.get(&date_str) {
            result.push(stats.clone());
        } else {
            result.push(DailyStats::zero(&date_str));
        }
        current += chrono::Duration::days(1);
    }

    Ok(result)
}

/// Count breaks completed today (for initializing timer state on startup).
pub fn count_breaks_today(conn: &Connection) -> SqlResult<u32> {
    let today = Utc::now().format("%Y-%m-%d").to_string();
    let parsed = NaiveDate::parse_from_str(&today, "%Y-%m-%d").unwrap();
    let start_of_day = parsed
        .and_hms_opt(0, 0, 0)
        .unwrap()
        .and_utc()
        .timestamp_millis();
    let end_of_day = parsed
        .and_hms_opt(23, 59, 59)
        .unwrap()
        .and_utc()
        .timestamp_millis()
        + 999;

    let count: i32 = conn.query_row(
        "SELECT COUNT(*) FROM break_records WHERE completed = 1 AND started_at >= ?1 AND started_at <= ?2",
        params![start_of_day, end_of_day],
        |row| row.get(0),
    )?;

    Ok(count as u32)
}

/// Delete all break records, clear cache, reset settings to defaults.
pub fn clear_all_data(conn: &Connection) -> SqlResult<()> {
    conn.execute_batch(
        "DELETE FROM break_records;
         DELETE FROM daily_stats_cache;
         DELETE FROM settings;
         INSERT OR IGNORE INTO settings (id) VALUES (1);",
    )?;
    Ok(())
}

/// Export all break records as CSV to the user's Downloads folder.
/// Returns the file path of the created CSV.
pub fn export_to_csv(conn: &Connection) -> SqlResult<String> {
    let downloads = dirs::download_dir().unwrap_or_else(|| {
        dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("Downloads")
    });

    let timestamp = Utc::now().format("%Y%m%d_%H%M%S");
    let filename = format!("blinky_export_{}.csv", timestamp);
    let path = downloads.join(&filename);

    let mut stmt = conn.prepare(
        "SELECT id, started_at, duration_seconds, completed, skipped, preceding_work_seconds
         FROM break_records ORDER BY started_at ASC",
    )?;

    let records: Vec<BreakRecord> = stmt
        .query_map([], |row| {
            Ok(BreakRecord {
                id: row.get(0)?,
                started_at: row.get::<_, i64>(1)? as u64,
                duration_seconds: row.get::<_, i32>(2)? as u32,
                completed: row.get::<_, i32>(3)? != 0,
                skipped: row.get::<_, i32>(4)? != 0,
                preceding_work_seconds: row.get::<_, i32>(5)? as u32,
            })
        })?
        .collect::<SqlResult<Vec<_>>>()?;

    let mut csv =
        String::from("id,started_at,duration_seconds,completed,skipped,preceding_work_seconds\n");
    for r in &records {
        csv.push_str(&format!(
            "{},{},{},{},{},{}\n",
            r.id,
            r.started_at,
            r.duration_seconds,
            r.completed,
            r.skipped,
            r.preceding_work_seconds
        ));
    }

    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| rusqlite::Error::InvalidParameterName(format!("dir error: {}", e)))?;
    }
    std::fs::write(&path, csv)
        .map_err(|e| rusqlite::Error::InvalidParameterName(format!("write error: {}", e)))?;

    Ok(path.to_string_lossy().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_test_db() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        init_db_conn(&conn).unwrap();
        conn
    }

    #[test]
    fn test_default_settings() {
        let conn = setup_test_db();
        let settings = load_settings(&conn).unwrap();
        let defaults = UserSettings::default();

        assert_eq!(
            settings.work_interval_minutes,
            defaults.work_interval_minutes
        );
        assert_eq!(
            settings.break_duration_seconds,
            defaults.break_duration_seconds
        );
        assert_eq!(settings.sound_enabled, defaults.sound_enabled);
        assert!((settings.sound_volume - defaults.sound_volume).abs() < 0.01);
        assert_eq!(settings.notification_enabled, defaults.notification_enabled);
        assert_eq!(settings.overlay_enabled, defaults.overlay_enabled);
        assert_eq!(settings.launch_at_login, defaults.launch_at_login);
        assert_eq!(settings.daily_goal, defaults.daily_goal);
        assert_eq!(settings.idle_pause_minutes, defaults.idle_pause_minutes);
        assert_eq!(settings.theme, defaults.theme);
    }

    #[test]
    fn test_save_load_settings_roundtrip() {
        let conn = setup_test_db();
        let mut settings = UserSettings::default();
        settings.work_interval_minutes = 15;
        settings.break_duration_seconds = 30;
        settings.sound_enabled = false;
        settings.sound_volume = 0.5;
        settings.notification_enabled = false;
        settings.overlay_enabled = false;
        settings.launch_at_login = true;
        settings.daily_goal = 12;
        settings.idle_pause_minutes = 10;
        settings.theme = "dark".to_string();

        save_settings(&conn, &settings).unwrap();
        let loaded = load_settings(&conn).unwrap();

        assert_eq!(loaded.work_interval_minutes, 15);
        assert_eq!(loaded.break_duration_seconds, 30);
        assert!(!loaded.sound_enabled);
        assert!((loaded.sound_volume - 0.5).abs() < 0.01);
        assert!(!loaded.notification_enabled);
        assert!(!loaded.overlay_enabled);
        assert!(loaded.launch_at_login);
        assert_eq!(loaded.daily_goal, 12);
        assert_eq!(loaded.idle_pause_minutes, 10);
        assert_eq!(loaded.theme, "dark");
    }

    #[test]
    fn test_insert_and_query_break_records() {
        let conn = setup_test_db();
        let now = Utc::now().timestamp_millis() as u64;

        // Insert 10 records with increasing timestamps
        for i in 0..10 {
            let id = insert_break_record(&conn, now + i * 1000, 1200).unwrap();
            if i % 3 == 0 {
                update_break_completion(&conn, id, 20, true, false).unwrap();
            } else if i % 3 == 1 {
                update_break_completion(&conn, id, 5, false, true).unwrap();
            }
            // i % 3 == 2: leave as in-progress (default)
        }

        // Query first 5 (newest first)
        let records = get_break_records(&conn, 5, 0).unwrap();
        assert_eq!(records.len(), 5);
        // Newest first means descending started_at
        assert!(records[0].started_at > records[1].started_at);

        // Query next 5
        let records2 = get_break_records(&conn, 5, 5).unwrap();
        assert_eq!(records2.len(), 5);

        // All 10
        let all = get_break_records(&conn, 100, 0).unwrap();
        assert_eq!(all.len(), 10);
    }

    #[test]
    fn test_daily_stats_computation() {
        let conn = setup_test_db();
        let today = Utc::now().format("%Y-%m-%d").to_string();
        let parsed = NaiveDate::parse_from_str(&today, "%Y-%m-%d").unwrap();
        let base = parsed
            .and_hms_opt(10, 0, 0)
            .unwrap()
            .and_utc()
            .timestamp_millis() as u64;

        // 3 completed, 1 skipped
        let id1 = insert_break_record(&conn, base, 1200).unwrap();
        update_break_completion(&conn, id1, 20, true, false).unwrap();

        let id2 = insert_break_record(&conn, base + 1200_000, 1200).unwrap();
        update_break_completion(&conn, id2, 20, true, false).unwrap();

        let id3 = insert_break_record(&conn, base + 2400_000, 1200).unwrap();
        update_break_completion(&conn, id3, 20, true, false).unwrap();

        let id4 = insert_break_record(&conn, base + 3600_000, 1200).unwrap();
        update_break_completion(&conn, id4, 5, false, true).unwrap();

        let stats = recompute_daily_stats(&conn, &today).unwrap();
        assert_eq!(stats.breaks_completed, 3);
        assert_eq!(stats.breaks_skipped, 1);
        assert_eq!(stats.total_rest_seconds, 60); // 3 * 20
        assert_eq!(stats.longest_streak, 3); // first 3 were consecutive completed
        assert!((stats.compliance_rate - 0.75).abs() < 0.001);
    }

    #[test]
    fn test_longest_streak_edge_cases() {
        let conn = setup_test_db();
        let today = Utc::now().format("%Y-%m-%d").to_string();
        let parsed = NaiveDate::parse_from_str(&today, "%Y-%m-%d").unwrap();
        let base = parsed
            .and_hms_opt(10, 0, 0)
            .unwrap()
            .and_utc()
            .timestamp_millis() as u64;

        // All completed: streak = 5
        for i in 0..5 {
            let id = insert_break_record(&conn, base + i * 60_000, 1200).unwrap();
            update_break_completion(&conn, id, 20, true, false).unwrap();
        }
        let stats = recompute_daily_stats(&conn, &today).unwrap();
        assert_eq!(stats.longest_streak, 5);

        // Clear and test all skipped: streak = 0
        conn.execute_batch("DELETE FROM break_records; DELETE FROM daily_stats_cache;")
            .unwrap();
        for i in 0..3 {
            let id = insert_break_record(&conn, base + i * 60_000, 1200).unwrap();
            update_break_completion(&conn, id, 5, false, true).unwrap();
        }
        let stats = recompute_daily_stats(&conn, &today).unwrap();
        assert_eq!(stats.longest_streak, 0);

        // Clear and test alternating: streak = 1
        conn.execute_batch("DELETE FROM break_records; DELETE FROM daily_stats_cache;")
            .unwrap();
        for i in 0..6 {
            let id = insert_break_record(&conn, base + i * 60_000, 1200).unwrap();
            if i % 2 == 0 {
                update_break_completion(&conn, id, 20, true, false).unwrap();
            } else {
                update_break_completion(&conn, id, 5, false, true).unwrap();
            }
        }
        let stats = recompute_daily_stats(&conn, &today).unwrap();
        assert_eq!(stats.longest_streak, 1);
    }

    #[test]
    fn test_compliance_rate_zero_division() {
        let conn = setup_test_db();
        let today = Utc::now().format("%Y-%m-%d").to_string();
        let stats = recompute_daily_stats(&conn, &today).unwrap();
        assert_eq!(stats.compliance_rate, 0.0);
        assert_eq!(stats.breaks_completed, 0);
        assert_eq!(stats.breaks_skipped, 0);
    }

    #[test]
    fn test_clear_all_data_resets_everything() {
        let conn = setup_test_db();
        let now = Utc::now().timestamp_millis() as u64;

        // Insert some data
        insert_break_record(&conn, now, 1200).unwrap();
        let mut settings = UserSettings::default();
        settings.work_interval_minutes = 10;
        save_settings(&conn, &settings).unwrap();

        // Clear
        clear_all_data(&conn).unwrap();

        // Verify break records are gone
        let records = get_break_records(&conn, 100, 0).unwrap();
        assert_eq!(records.len(), 0);

        // Verify settings reset to defaults
        let loaded = load_settings(&conn).unwrap();
        assert_eq!(loaded.work_interval_minutes, 20); // back to default
    }

    #[test]
    fn test_daily_stats_range_zero_fills() {
        let conn = setup_test_db();
        let today = Utc::now().date_naive();
        let from = (today - chrono::Duration::days(6))
            .format("%Y-%m-%d")
            .to_string();
        let to = today.format("%Y-%m-%d").to_string();

        let range = get_daily_stats_range(&conn, &from, &to).unwrap();
        assert_eq!(range.len(), 7);

        // All should be zero-filled
        for entry in &range {
            assert_eq!(entry.breaks_completed, 0);
            assert_eq!(entry.breaks_skipped, 0);
        }
    }

    #[test]
    fn test_count_breaks_today() {
        let conn = setup_test_db();
        let today = Utc::now().format("%Y-%m-%d").to_string();
        let parsed = NaiveDate::parse_from_str(&today, "%Y-%m-%d").unwrap();
        let base = parsed
            .and_hms_opt(10, 0, 0)
            .unwrap()
            .and_utc()
            .timestamp_millis() as u64;

        assert_eq!(count_breaks_today(&conn).unwrap(), 0);

        let id1 = insert_break_record(&conn, base, 1200).unwrap();
        update_break_completion(&conn, id1, 20, true, false).unwrap();
        let id2 = insert_break_record(&conn, base + 60_000, 1200).unwrap();
        update_break_completion(&conn, id2, 5, false, true).unwrap(); // skipped, not counted
        let id3 = insert_break_record(&conn, base + 120_000, 1200).unwrap();
        update_break_completion(&conn, id3, 20, true, false).unwrap();

        assert_eq!(count_breaks_today(&conn).unwrap(), 2);
    }

    #[test]
    fn test_export_to_csv() {
        let conn = setup_test_db();
        let now = Utc::now().timestamp_millis() as u64;

        let id1 = insert_break_record(&conn, now, 1200).unwrap();
        update_break_completion(&conn, id1, 20, true, false).unwrap();
        let id2 = insert_break_record(&conn, now + 60_000, 1200).unwrap();
        update_break_completion(&conn, id2, 5, false, true).unwrap();

        let path = export_to_csv(&conn).unwrap();
        assert!(std::path::Path::new(&path).exists());

        let contents = std::fs::read_to_string(&path).unwrap();
        let lines: Vec<&str> = contents.lines().collect();
        assert_eq!(lines.len(), 3); // header + 2 records
        assert!(lines[0].starts_with("id,started_at,"));
        assert!(lines[1].contains(",true,false,")); // completed
        assert!(lines[2].contains(",false,true,")); // skipped

        // Clean up
        std::fs::remove_file(&path).unwrap();
    }

    #[test]
    fn test_migration_is_idempotent() {
        let conn = setup_test_db();
        // Running init again should not fail
        init_db_conn(&conn).unwrap();
        // Settings should still be there
        let settings = load_settings(&conn).unwrap();
        assert_eq!(settings.work_interval_minutes, 20);
    }

    #[test]
    fn test_onboarding_defaults_on_fresh_db() {
        let conn = setup_test_db();
        let settings = load_settings(&conn).unwrap();
        assert!(!settings.onboarding_completed);
        assert!(settings.onboarding_completed_at.is_none());
        assert_eq!(settings.tooltips_seen, "[]");
        assert!(!settings.first_break_completed);
    }

    #[test]
    fn test_onboarding_auto_complete_for_existing_users() {
        // Simulate an existing database that only has migration 001 applied
        let conn = Connection::open_in_memory().unwrap();
        conn.execute_batch("PRAGMA journal_mode=WAL;").unwrap();
        conn.execute_batch("PRAGMA foreign_keys=ON;").unwrap();

        // Apply only migration 001 manually
        conn.execute_batch(MIGRATION_001_SQL).unwrap();
        conn.execute(
            "INSERT INTO _migrations (name) VALUES (?1)",
            params!["001_initial"],
        )
        .unwrap();

        // Insert a break record (simulating an existing user)
        let now = chrono::Utc::now().timestamp_millis();
        conn.execute(
            "INSERT INTO break_records (started_at, duration_seconds, completed, skipped, preceding_work_seconds) VALUES (?1, 20, 1, 0, 1200)",
            params![now],
        )
        .unwrap();

        // Now run all migrations (should apply 002 and auto-complete)
        run_migrations(&conn).unwrap();

        let settings = load_settings(&conn).unwrap();
        assert!(
            settings.onboarding_completed,
            "onboarding should be auto-completed for existing users"
        );
        assert!(
            settings.first_break_completed,
            "first_break should be auto-completed for existing users"
        );
        assert!(
            settings.onboarding_completed_at.is_some(),
            "onboarding_completed_at should be set"
        );
    }

    #[test]
    fn test_onboarding_not_auto_completed_for_fresh_install() {
        // Fresh database — no break records
        let conn = setup_test_db();
        let settings = load_settings(&conn).unwrap();
        assert!(
            !settings.onboarding_completed,
            "onboarding should NOT be auto-completed on fresh install"
        );
        assert!(!settings.first_break_completed);
    }

    #[test]
    fn test_save_load_onboarding_fields_roundtrip() {
        let conn = setup_test_db();
        let mut settings = load_settings(&conn).unwrap();

        settings.onboarding_completed = true;
        settings.onboarding_completed_at = Some(1700000000000);
        settings.tooltips_seen = r#"["streak","timer"]"#.to_string();
        settings.first_break_completed = true;

        save_settings(&conn, &settings).unwrap();
        let loaded = load_settings(&conn).unwrap();

        assert!(loaded.onboarding_completed);
        assert_eq!(loaded.onboarding_completed_at, Some(1700000000000));
        assert_eq!(loaded.tooltips_seen, r#"["streak","timer"]"#);
        assert!(loaded.first_break_completed);
    }
}
