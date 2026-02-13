export type TimerPhase = "Working" | "Breaking" | "Paused" | "Suspended";

export interface TimerState {
  phase: TimerPhase;
  seconds_remaining: number;
  phase_duration: number;
  phase_started_at: number;
  breaks_completed_today: number;
}

export interface UserSettings {
  work_interval_minutes: number;
  break_duration_seconds: number;
  sound_enabled: boolean;
  sound_volume: number;
  notification_enabled: boolean;
  overlay_enabled: boolean;
  launch_at_login: boolean;
  daily_goal: number;
  idle_pause_minutes: number;
  theme: string;
  onboarding_completed: boolean;
  onboarding_completed_at: number | null;
  tooltips_seen: string;
  first_break_completed: boolean;
}

export interface BreakRecord {
  id: number;
  started_at: number;
  duration_seconds: number;
  completed: boolean;
  skipped: boolean;
  preceding_work_seconds: number;
}

export interface DailyStats {
  date: string;
  breaks_completed: number;
  breaks_skipped: number;
  total_rest_seconds: number;
  longest_streak: number;
  compliance_rate: number;
}

export interface OnboardingState {
  onboarding_completed: boolean;
  onboarding_completed_at: number | null;
  tooltips_seen: string[];
  first_break_completed: boolean;
  is_first_day: boolean;
}

export interface AnalyticsSummary {
  today: DailyStats;
  last_7_days: DailyStats[];
  last_30_days: DailyStats[];
  current_day_streak: number;
  best_day_streak: number;
  lifetime_breaks: number;
  lifetime_rest_seconds: number;
}
