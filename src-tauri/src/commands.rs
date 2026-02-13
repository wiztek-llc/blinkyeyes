use crate::state::{
    AnalyticsSummary, AppState, BreakRecord, DailyStats, DbConnection, TimerState, UserSettings,
};
use crate::{analytics, autostart, db, settings, timer};
use tauri::{AppHandle, Emitter, Manager, State};

#[tauri::command]
pub fn get_timer_state(state: State<AppState>) -> Result<TimerState, String> {
    let timer = state.timer.lock().map_err(|e| e.to_string())?;
    Ok(timer.clone())
}

#[tauri::command]
pub fn pause_timer(app: AppHandle) -> Result<TimerState, String> {
    Ok(timer::pause(&app))
}

#[tauri::command]
pub fn resume_timer(app: AppHandle) -> Result<TimerState, String> {
    Ok(timer::resume(&app))
}

#[tauri::command]
pub fn skip_break(app: AppHandle) -> Result<TimerState, String> {
    Ok(timer::skip_break(&app))
}

#[tauri::command]
pub fn reset_timer(app: AppHandle) -> Result<TimerState, String> {
    Ok(timer::reset(&app))
}

#[tauri::command]
pub fn get_settings(state: State<AppState>) -> Result<UserSettings, String> {
    let s = state.settings.lock().map_err(|e| e.to_string())?;
    Ok(s.clone())
}

#[tauri::command]
pub fn update_settings(
    app: AppHandle,
    settings: UserSettings,
) -> Result<UserSettings, String> {
    // Validate
    settings::validate_settings(&settings)?;

    let state = app.state::<AppState>();
    let db_conn = app.state::<DbConnection>();

    // Check what changed for side effects
    let old_settings = state.settings.lock().map_err(|e| e.to_string())?.clone();

    // Save to DB
    {
        let conn = db_conn.0.lock().map_err(|e| e.to_string())?;
        db::save_settings(&conn, &settings).map_err(|e| e.to_string())?;
    }

    // Update in-memory state
    {
        let mut s = state.settings.lock().map_err(|e| e.to_string())?;
        *s = settings.clone();
    }

    // Side effect: autostart
    if old_settings.launch_at_login != settings.launch_at_login {
        autostart::set_autostart(&app, settings.launch_at_login);
    }

    // Emit settings-changed event
    let _ = app.emit("settings-changed", &settings);

    Ok(settings)
}

#[tauri::command]
pub fn get_analytics_summary(
    state: State<AppState>,
    db_conn: State<DbConnection>,
) -> Result<AnalyticsSummary, String> {
    let daily_goal = state.settings.lock().map_err(|e| e.to_string())?.daily_goal;
    let conn = db_conn.0.lock().map_err(|e| e.to_string())?;
    analytics::build_analytics_summary(&conn, daily_goal).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_break_history(
    db_conn: State<DbConnection>,
    limit: u32,
    offset: u32,
) -> Result<Vec<BreakRecord>, String> {
    let conn = db_conn.0.lock().map_err(|e| e.to_string())?;
    db::get_break_records(&conn, limit, offset).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_daily_stats_range(
    db_conn: State<DbConnection>,
    from: String,
    to: String,
) -> Result<Vec<DailyStats>, String> {
    let conn = db_conn.0.lock().map_err(|e| e.to_string())?;
    db::get_daily_stats_range(&conn, &from, &to).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn export_data_csv(db_conn: State<DbConnection>) -> Result<String, String> {
    let conn = db_conn.0.lock().map_err(|e| e.to_string())?;
    db::export_to_csv(&conn).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn clear_all_data(
    state: State<AppState>,
    db_conn: State<DbConnection>,
) -> Result<bool, String> {
    let conn = db_conn.0.lock().map_err(|e| e.to_string())?;
    db::clear_all_data(&conn).map_err(|e| e.to_string())?;

    // Reset in-memory settings to defaults
    {
        let mut s = state.settings.lock().map_err(|e| e.to_string())?;
        *s = UserSettings::default();
    }

    // Reset breaks_completed_today since all data was cleared
    {
        let mut timer = state.timer.lock().map_err(|e| e.to_string())?;
        timer.breaks_completed_today = 0;
    }

    Ok(true)
}
