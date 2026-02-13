use crate::state::UserSettings;

/// Validate user settings. Returns Ok(()) if valid, Err(message) if invalid.
pub fn validate_settings(settings: &UserSettings) -> Result<(), String> {
    if settings.work_interval_minutes < 1 || settings.work_interval_minutes > 120 {
        return Err("work_interval_minutes must be between 1 and 120".to_string());
    }
    if settings.break_duration_seconds < 5 || settings.break_duration_seconds > 300 {
        return Err("break_duration_seconds must be between 5 and 300".to_string());
    }
    if settings.sound_volume < 0.0 || settings.sound_volume > 1.0 {
        return Err("sound_volume must be between 0.0 and 1.0".to_string());
    }
    if settings.daily_goal < 1 || settings.daily_goal > 100 {
        return Err("daily_goal must be between 1 and 100".to_string());
    }
    if !["system", "light", "dark"].contains(&settings.theme.as_str()) {
        return Err("theme must be 'system', 'light', or 'dark'".to_string());
    }
    Ok(())
}
