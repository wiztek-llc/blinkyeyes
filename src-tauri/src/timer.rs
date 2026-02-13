use crate::state::{AppState, DbConnection, TimerPhase, TimerState};
use chrono::Utc;
use std::time::Duration;
use tauri::{AppHandle, Emitter, Manager};

/// Spawn the timer loop as an async background task.
/// Call this from `.setup()` in lib.rs.
pub fn start_timer_loop(app: &AppHandle) {
    let handle = app.clone();
    tauri::async_runtime::spawn(async move {
        let mut idle_check_counter: u32 = 0;
        loop {
            tick(&handle, &mut idle_check_counter);
            tokio::time::sleep(Duration::from_secs(1)).await;
        }
    });
}

/// One tick of the timer state machine.
/// Reads current state, computes remaining time via wall clock,
/// handles phase transitions, emits events.
fn tick(app: &AppHandle, idle_check_counter: &mut u32) {
    // Check for idle every 30 seconds
    *idle_check_counter += 1;
    if *idle_check_counter >= 30 {
        *idle_check_counter = 0;
        check_idle(app);
    }

    let now_ms = Utc::now().timestamp_millis() as u64;

    // Determine what transition (if any) should happen.
    // We gather everything we need, then release locks before doing I/O and events.
    let transition = {
        let state = app.state::<AppState>();
        let settings = state.settings.lock().unwrap().clone();
        let mut timer = state.timer.lock().unwrap();
        let mut internal = state.timer_internal.lock().unwrap();

        match timer.phase {
            TimerPhase::Working => {
                let elapsed_secs = now_ms.saturating_sub(timer.phase_started_at) / 1000;
                let remaining = timer.phase_duration.saturating_sub(elapsed_secs);
                timer.seconds_remaining = remaining;

                if remaining == 0 {
                    // Working → Breaking
                    let break_duration = settings.break_duration_seconds as u64;
                    let preceding_work = timer.phase_duration as u32;

                    timer.phase = TimerPhase::Breaking;
                    timer.phase_duration = break_duration;
                    timer.seconds_remaining = break_duration;
                    timer.phase_started_at = now_ms;

                    internal.work_started_at = 0;

                    let snapshot = timer.clone();
                    Transition::StartBreak {
                        snapshot,
                        preceding_work,
                        notification_enabled: settings.notification_enabled,
                        overlay_enabled: settings.overlay_enabled,
                    }
                } else {
                    Transition::Tick(timer.clone())
                }
            }
            TimerPhase::Breaking => {
                let elapsed_secs = now_ms.saturating_sub(timer.phase_started_at) / 1000;
                let remaining = timer.phase_duration.saturating_sub(elapsed_secs);
                timer.seconds_remaining = remaining;

                if remaining == 0 {
                    // Breaking → Working (complete)
                    let break_record_id = internal.current_break_record_id.take();
                    let actual_duration = timer.phase_duration as u32;

                    let work_duration = settings.work_interval_minutes as u64 * 60;
                    timer.phase = TimerPhase::Working;
                    timer.phase_duration = work_duration;
                    timer.seconds_remaining = work_duration;
                    timer.phase_started_at = now_ms;
                    timer.breaks_completed_today += 1;

                    internal.work_started_at = now_ms;

                    let snapshot = timer.clone();
                    Transition::CompleteBreak {
                        snapshot,
                        break_record_id,
                        actual_duration,
                        overlay_enabled: settings.overlay_enabled,
                    }
                } else {
                    Transition::Tick(timer.clone())
                }
            }
            TimerPhase::Paused | TimerPhase::Suspended => {
                // Frozen — seconds_remaining unchanged
                Transition::Tick(timer.clone())
            }
        }
    };

    // Now handle side effects outside of locks
    match transition {
        Transition::StartBreak {
            snapshot,
            preceding_work,
            notification_enabled,
            overlay_enabled,
        } => {
            // Insert break record in DB
            if let Some(db_conn) = try_state::<DbConnection>(app) {
                let db = db_conn.0.lock().unwrap();
                match crate::db::insert_break_record(&db, now_ms, preceding_work) {
                    Ok(id) => {
                        let state = app.state::<AppState>();
                        let mut internal = state.timer_internal.lock().unwrap();
                        internal.current_break_record_id = Some(id);
                    }
                    Err(e) => {
                        eprintln!("[timer] Failed to insert break record: {}", e);
                    }
                }
            }

            if notification_enabled {
                crate::notifications::send_break_notification(app);
            }
            if overlay_enabled {
                crate::overlay::show_overlay(app);
            }
            crate::tray::update_tray_status(app, &snapshot.phase, snapshot.seconds_remaining);

            let _ = app.emit("break-started", &snapshot);
            let _ = app.emit("timer-tick", &snapshot);
        }
        Transition::CompleteBreak {
            snapshot,
            break_record_id,
            actual_duration,
            overlay_enabled,
        } => {
            // Finalize break record in DB
            if let Some(id) = break_record_id {
                if let Some(db_conn) = try_state::<DbConnection>(app) {
                    let db = db_conn.0.lock().unwrap();
                    let _ = crate::db::update_break_completion(&db, id, actual_duration, true, false);
                    let today = Utc::now().format("%Y-%m-%d").to_string();
                    let _ = crate::db::recompute_daily_stats(&db, &today);
                }
            }

            // Chime is played by the frontend on `break-completed` event
            if overlay_enabled {
                crate::overlay::hide_overlay(app);
            }
            crate::tray::update_tray_status(app, &snapshot.phase, snapshot.seconds_remaining);

            let _ = app.emit("break-completed", &snapshot);
            let _ = app.emit("timer-tick", &snapshot);
        }
        Transition::Tick(snapshot) => {
            crate::tray::update_tray_status(app, &snapshot.phase, snapshot.seconds_remaining);
            let _ = app.emit("timer-tick", &snapshot);
        }
    }
}

/// Try to get managed state. Returns None if not yet managed (shouldn't happen in practice).
fn try_state<T: Send + Sync + 'static>(app: &AppHandle) -> Option<tauri::State<'_, T>> {
    app.try_state::<T>()
}

// --- Public API (called by commands.rs) ---

/// Pause the timer. Freezes seconds_remaining at current wall-clock value.
pub fn pause(app: &AppHandle) -> TimerState {
    let state = app.state::<AppState>();
    let mut timer = state.timer.lock().unwrap();
    let mut internal = state.timer_internal.lock().unwrap();

    if timer.phase == TimerPhase::Paused || timer.phase == TimerPhase::Suspended {
        return timer.clone();
    }

    // Snapshot remaining time using wall clock
    let now_ms = Utc::now().timestamp_millis() as u64;
    let elapsed = now_ms.saturating_sub(timer.phase_started_at) / 1000;
    timer.seconds_remaining = timer.phase_duration.saturating_sub(elapsed);

    internal.phase_before_pause = timer.phase.clone();
    timer.phase = TimerPhase::Paused;

    let result = timer.clone();
    drop(internal);
    drop(timer);

    crate::tray::update_tray_status(app, &result.phase, result.seconds_remaining);
    let _ = app.emit("timer-paused", &result);
    let _ = app.emit("timer-tick", &result);

    result
}

/// Resume from Paused state. Adjusts phase_started_at so wall-clock math is correct.
pub fn resume(app: &AppHandle) -> TimerState {
    let state = app.state::<AppState>();
    let mut timer = state.timer.lock().unwrap();
    let internal = state.timer_internal.lock().unwrap();

    if timer.phase != TimerPhase::Paused {
        return timer.clone();
    }

    let now_ms = Utc::now().timestamp_millis() as u64;

    // Restore the phase we were in before pausing
    timer.phase = internal.phase_before_pause.clone();

    // Adjust phase_started_at so that wall-clock math yields the frozen seconds_remaining
    let elapsed_before = timer.phase_duration.saturating_sub(timer.seconds_remaining);
    timer.phase_started_at = now_ms - (elapsed_before * 1000);

    let result = timer.clone();
    drop(internal);
    drop(timer);

    crate::tray::update_tray_status(app, &result.phase, result.seconds_remaining);
    let _ = app.emit("timer-resumed", &result);
    let _ = app.emit("timer-tick", &result);

    result
}

/// Skip the current break. Logs it as skipped, returns to Working.
pub fn skip_break(app: &AppHandle) -> TimerState {
    let state = app.state::<AppState>();
    let mut timer = state.timer.lock().unwrap();
    let mut internal = state.timer_internal.lock().unwrap();

    // No-op if not breaking (debounce)
    if timer.phase != TimerPhase::Breaking {
        return timer.clone();
    }

    let now_ms = Utc::now().timestamp_millis() as u64;
    let elapsed = ((now_ms.saturating_sub(timer.phase_started_at)) / 1000) as u32;
    let break_record_id = internal.current_break_record_id.take();

    // Reset to working
    let settings = state.settings.lock().unwrap().clone();
    let work_duration = settings.work_interval_minutes as u64 * 60;

    timer.phase = TimerPhase::Working;
    timer.phase_duration = work_duration;
    timer.seconds_remaining = work_duration;
    timer.phase_started_at = now_ms;

    internal.work_started_at = now_ms;

    let result = timer.clone();
    drop(internal);
    drop(timer);

    // Update break record as skipped
    if let Some(id) = break_record_id {
        if let Some(db_conn) = try_state::<DbConnection>(app) {
            let db = db_conn.0.lock().unwrap();
            let _ = crate::db::update_break_completion(&db, id, elapsed, false, true);
            let today = Utc::now().format("%Y-%m-%d").to_string();
            let _ = crate::db::recompute_daily_stats(&db, &today);
        }
    }

    crate::overlay::hide_overlay(app);
    crate::tray::update_tray_status(app, &result.phase, result.seconds_remaining);
    let _ = app.emit("break-skipped", &result);
    let _ = app.emit("timer-tick", &result);

    result
}

/// Reset the timer to a fresh work interval.
pub fn reset(app: &AppHandle) -> TimerState {
    let state = app.state::<AppState>();
    let mut timer = state.timer.lock().unwrap();
    let mut internal = state.timer_internal.lock().unwrap();

    let now_ms = Utc::now().timestamp_millis() as u64;

    // If in a break, mark it as skipped
    let break_record_id = if timer.phase == TimerPhase::Breaking {
        let elapsed = ((now_ms.saturating_sub(timer.phase_started_at)) / 1000) as u32;
        let id = internal.current_break_record_id.take();
        if let Some(id) = id {
            // Store elapsed for DB update after dropping locks
            Some((id, elapsed))
        } else {
            None
        }
    } else {
        internal.current_break_record_id.take();
        None
    };

    let settings = state.settings.lock().unwrap().clone();
    let work_duration = settings.work_interval_minutes as u64 * 60;

    timer.phase = TimerPhase::Working;
    timer.phase_duration = work_duration;
    timer.seconds_remaining = work_duration;
    timer.phase_started_at = now_ms;

    internal.work_started_at = now_ms;

    let result = timer.clone();
    drop(internal);
    drop(timer);

    // Finalize any in-progress break record
    if let Some((id, elapsed)) = break_record_id {
        if let Some(db_conn) = try_state::<DbConnection>(app) {
            let db = db_conn.0.lock().unwrap();
            let _ = crate::db::update_break_completion(&db, id, elapsed, false, true);
            let today = Utc::now().format("%Y-%m-%d").to_string();
            let _ = crate::db::recompute_daily_stats(&db, &today);
        }
    }

    crate::overlay::hide_overlay(app);
    crate::tray::update_tray_status(app, &result.phase, result.seconds_remaining);
    let _ = app.emit("timer-tick", &result);

    result
}

/// Check system idle time and transition to/from Suspended as needed.
/// Called every ~30 seconds from the timer loop.
fn check_idle(app: &AppHandle) {
    let state = app.state::<AppState>();
    let settings = state.settings.lock().unwrap().clone();

    // 0 = idle detection disabled
    if settings.idle_pause_minutes == 0 {
        return;
    }

    let threshold_secs = settings.idle_pause_minutes as u64 * 60;
    let idle_secs = match crate::idle::get_idle_seconds() {
        Some(s) => s,
        None => return, // Detection not available on this platform
    };

    if idle_secs >= threshold_secs {
        // Suspend if currently Working
        let mut timer = state.timer.lock().unwrap();
        if timer.phase != TimerPhase::Working {
            return;
        }

        let mut internal = state.timer_internal.lock().unwrap();
        let now_ms = Utc::now().timestamp_millis() as u64;

        // Freeze remaining time using wall clock
        let elapsed = now_ms.saturating_sub(timer.phase_started_at) / 1000;
        timer.seconds_remaining = timer.phase_duration.saturating_sub(elapsed);
        internal.phase_before_pause = TimerPhase::Working;
        timer.phase = TimerPhase::Suspended;

        let snapshot = timer.clone();
        drop(internal);
        drop(timer);

        crate::tray::update_tray_status(app, &snapshot.phase, snapshot.seconds_remaining);
        let _ = app.emit("timer-paused", &snapshot);
        let _ = app.emit("timer-tick", &snapshot);
    } else if idle_secs < threshold_secs {
        // Resume with fresh work interval if currently Suspended
        let mut timer = state.timer.lock().unwrap();
        if timer.phase != TimerPhase::Suspended {
            return;
        }

        let now_ms = Utc::now().timestamp_millis() as u64;
        let work_duration = settings.work_interval_minutes as u64 * 60;

        timer.phase = TimerPhase::Working;
        timer.phase_duration = work_duration;
        timer.seconds_remaining = work_duration;
        timer.phase_started_at = now_ms;

        let mut internal = state.timer_internal.lock().unwrap();
        internal.work_started_at = now_ms;

        let snapshot = timer.clone();
        drop(internal);
        drop(timer);

        crate::tray::update_tray_status(app, &snapshot.phase, snapshot.seconds_remaining);
        let _ = app.emit("timer-resumed", &snapshot);
        let _ = app.emit("timer-tick", &snapshot);
    }
}

/// Internal enum to describe what the tick determined should happen,
/// so we can release locks before performing side effects.
enum Transition {
    Tick(TimerState),
    StartBreak {
        snapshot: TimerState,
        preceding_work: u32,
        notification_enabled: bool,
        overlay_enabled: bool,
    },
    CompleteBreak {
        snapshot: TimerState,
        break_record_id: Option<i64>,
        actual_duration: u32,
        overlay_enabled: bool,
    },
}
