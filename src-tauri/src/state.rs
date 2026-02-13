use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TimerPhase {
    Working,
    Breaking,
    Paused,
    Suspended,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimerState {
    pub phase: TimerPhase,
    pub seconds_remaining: u64,
    pub phase_duration: u64,
    pub phase_started_at: u64,
    pub breaks_completed_today: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSettings {
    pub work_interval_minutes: u32,
    pub break_duration_seconds: u32,
    pub sound_enabled: bool,
    pub sound_volume: f32,
    pub notification_enabled: bool,
    pub overlay_enabled: bool,
    pub launch_at_login: bool,
    pub daily_goal: u32,
    pub idle_pause_minutes: u32,
    pub theme: String,
    pub onboarding_completed: bool,
    pub onboarding_completed_at: Option<u64>,
    pub tooltips_seen: String,
    pub first_break_completed: bool,
}

impl Default for UserSettings {
    fn default() -> Self {
        Self {
            work_interval_minutes: 20,
            break_duration_seconds: 20,
            sound_enabled: true,
            sound_volume: 0.7,
            notification_enabled: true,
            overlay_enabled: true,
            launch_at_login: false,
            daily_goal: 24,
            idle_pause_minutes: 5,
            theme: "system".to_string(),
            onboarding_completed: false,
            onboarding_completed_at: None,
            tooltips_seen: "[]".to_string(),
            first_break_completed: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BreakRecord {
    pub id: i64,
    pub started_at: u64,
    pub duration_seconds: u32,
    pub completed: bool,
    pub skipped: bool,
    pub preceding_work_seconds: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyStats {
    pub date: String,
    pub breaks_completed: u32,
    pub breaks_skipped: u32,
    pub total_rest_seconds: u32,
    pub longest_streak: u32,
    pub compliance_rate: f64,
}

impl DailyStats {
    pub fn zero(date: &str) -> Self {
        Self {
            date: date.to_string(),
            breaks_completed: 0,
            breaks_skipped: 0,
            total_rest_seconds: 0,
            longest_streak: 0,
            compliance_rate: 0.0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyticsSummary {
    pub today: DailyStats,
    pub last_7_days: Vec<DailyStats>,
    pub last_30_days: Vec<DailyStats>,
    pub current_day_streak: u32,
    pub best_day_streak: u32,
    pub lifetime_breaks: u64,
    pub lifetime_rest_seconds: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OnboardingState {
    pub onboarding_completed: bool,
    pub onboarding_completed_at: Option<u64>,
    pub tooltips_seen: Vec<String>,
    pub first_break_completed: bool,
    pub is_first_day: bool,
}

/// Internal timer bookkeeping — not exposed via IPC.
pub struct TimerInternalState {
    pub phase_before_pause: TimerPhase,
    pub current_break_record_id: Option<i64>,
    pub work_started_at: u64,
}

pub struct AppState {
    pub timer: Mutex<TimerState>,
    pub settings: Mutex<UserSettings>,
    pub db_path: String,
    pub timer_internal: Mutex<TimerInternalState>,
}

pub struct DbConnection(pub Mutex<Connection>);
