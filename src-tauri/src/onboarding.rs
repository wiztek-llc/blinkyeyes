use crate::state::{OnboardingState, UserSettings};
use chrono::{DateTime, Utc};

/// Build an OnboardingState from the raw UserSettings fields.
/// Parses tooltips_seen JSON string into Vec<String> and computes is_first_day.
pub fn build_onboarding_state(settings: &UserSettings) -> OnboardingState {
    let tooltips_seen: Vec<String> =
        serde_json::from_str(&settings.tooltips_seen).unwrap_or_default();

    let is_first_day = match settings.onboarding_completed_at {
        Some(ts_ms) => {
            let completed_date =
                DateTime::from_timestamp_millis(ts_ms as i64).map(|dt| dt.date_naive());
            let today = Utc::now().date_naive();
            completed_date == Some(today)
        }
        None => false,
    };

    OnboardingState {
        onboarding_completed: settings.onboarding_completed,
        onboarding_completed_at: settings.onboarding_completed_at,
        tooltips_seen,
        first_break_completed: settings.first_break_completed,
        is_first_day,
    }
}

/// Mark onboarding as complete. Sets the flag and timestamp.
/// Does NOT start the timer — that's the caller's responsibility.
pub fn complete_onboarding(settings: &mut UserSettings) {
    let now_ms = Utc::now().timestamp_millis() as u64;
    settings.onboarding_completed = true;
    settings.onboarding_completed_at = Some(now_ms);
}

/// Add a tooltip_id to the seen list if not already present.
/// Returns the updated list.
pub fn mark_tooltip_seen(settings: &mut UserSettings, tooltip_id: &str) -> Vec<String> {
    let mut seen: Vec<String> = serde_json::from_str(&settings.tooltips_seen).unwrap_or_default();
    if !seen.contains(&tooltip_id.to_string()) {
        seen.push(tooltip_id.to_string());
    }
    settings.tooltips_seen = serde_json::to_string(&seen).unwrap_or_else(|_| "[]".to_string());
    seen
}

/// Reset all onboarding state to defaults.
pub fn reset_onboarding(settings: &mut UserSettings) {
    settings.onboarding_completed = false;
    settings.onboarding_completed_at = None;
    settings.tooltips_seen = "[]".to_string();
    settings.first_break_completed = false;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_onboarding_state_fresh() {
        let settings = UserSettings::default();
        let state = build_onboarding_state(&settings);
        assert!(!state.onboarding_completed);
        assert!(state.onboarding_completed_at.is_none());
        assert!(state.tooltips_seen.is_empty());
        assert!(!state.first_break_completed);
        assert!(!state.is_first_day);
    }

    #[test]
    fn test_build_onboarding_state_completed_today() {
        let mut settings = UserSettings::default();
        let now_ms = Utc::now().timestamp_millis() as u64;
        settings.onboarding_completed = true;
        settings.onboarding_completed_at = Some(now_ms);

        let state = build_onboarding_state(&settings);
        assert!(state.onboarding_completed);
        assert!(state.is_first_day);
    }

    #[test]
    fn test_complete_onboarding() {
        let mut settings = UserSettings::default();
        assert!(!settings.onboarding_completed);

        complete_onboarding(&mut settings);

        assert!(settings.onboarding_completed);
        assert!(settings.onboarding_completed_at.is_some());
    }

    #[test]
    fn test_mark_tooltip_seen() {
        let mut settings = UserSettings::default();

        let seen = mark_tooltip_seen(&mut settings, "streak");
        assert_eq!(seen, vec!["streak".to_string()]);

        // Adding the same ID again should not duplicate
        let seen = mark_tooltip_seen(&mut settings, "streak");
        assert_eq!(seen, vec!["streak".to_string()]);

        // Adding a different ID
        let seen = mark_tooltip_seen(&mut settings, "timer");
        assert_eq!(seen, vec!["streak".to_string(), "timer".to_string()]);
    }

    #[test]
    fn test_reset_onboarding() {
        let mut settings = UserSettings::default();
        complete_onboarding(&mut settings);
        mark_tooltip_seen(&mut settings, "streak");
        settings.first_break_completed = true;

        reset_onboarding(&mut settings);

        assert!(!settings.onboarding_completed);
        assert!(settings.onboarding_completed_at.is_none());
        assert_eq!(settings.tooltips_seen, "[]");
        assert!(!settings.first_break_completed);
    }
}
