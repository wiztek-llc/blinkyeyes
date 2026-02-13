mod analytics;
mod audio;
mod autostart;
mod commands;
mod db;
mod idle;
mod notifications;
mod onboarding;
mod overlay;
mod settings;
pub mod state;
mod timer;
mod tray;

use state::{AppState, DbConnection, TimerInternalState, TimerPhase, TimerState};
use std::sync::Mutex;
use tauri::{Manager, WindowEvent};

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_autostart::init(
            tauri_plugin_autostart::MacosLauncher::LaunchAgent,
            None,
        ))
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            // Initialize database
            let db_dir = db::get_db_dir();
            std::fs::create_dir_all(&db_dir).expect("Failed to create DB directory");
            let db_path = db_dir.join("blinky.db");
            let db_path_str = db_path.to_string_lossy().to_string();
            let db_mutex = db::init_db(&db_path_str).expect("Failed to initialize database");

            // Load settings from DB
            let settings = {
                let conn = db_mutex.lock().unwrap();
                db::load_settings(&conn).unwrap_or_default()
            };

            // Count today's completed breaks for initial state
            let breaks_today = {
                let conn = db_mutex.lock().unwrap();
                db::count_breaks_today(&conn).unwrap_or(0)
            };

            let now_ms = chrono::Utc::now().timestamp_millis() as u64;
            let work_duration = settings.work_interval_minutes as u64 * 60;

            // If onboarding hasn't been completed, start in Paused phase
            let initial_phase = if settings.onboarding_completed {
                TimerPhase::Working
            } else {
                TimerPhase::Paused
            };

            let timer_state = TimerState {
                phase: initial_phase,
                seconds_remaining: work_duration,
                phase_duration: work_duration,
                phase_started_at: now_ms,
                breaks_completed_today: breaks_today,
            };

            let timer_internal = TimerInternalState {
                phase_before_pause: TimerPhase::Working,
                current_break_record_id: None,
                work_started_at: now_ms,
            };

            let app_state = AppState {
                timer: Mutex::new(timer_state),
                settings: Mutex::new(settings),
                db_path: db_path_str,
                timer_internal: Mutex::new(timer_internal),
            };

            app.manage(app_state);
            app.manage(DbConnection(db_mutex));

            // Create the system tray
            let tray_state = tray::create_tray(app.handle()).expect("Failed to create system tray");
            app.manage(tray_state);

            // Start the background timer loop
            timer::start_timer_loop(app.handle());

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::get_timer_state,
            commands::pause_timer,
            commands::resume_timer,
            commands::skip_break,
            commands::reset_timer,
            commands::get_settings,
            commands::update_settings,
            commands::get_analytics_summary,
            commands::get_break_history,
            commands::get_daily_stats_range,
            commands::export_data_csv,
            commands::clear_all_data,
            commands::get_onboarding_state,
            commands::complete_onboarding,
            commands::mark_tooltip_seen,
            commands::trigger_demo_break,
            commands::reset_onboarding,
        ])
        .on_window_event(|window, event| {
            // Hide windows instead of closing — keep app running in tray
            if let WindowEvent::CloseRequested { api, .. } = event {
                let _ = window.hide();
                api.prevent_close();
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
