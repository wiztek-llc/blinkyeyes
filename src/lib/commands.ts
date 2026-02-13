import { invoke } from "@tauri-apps/api/core";
import type {
  TimerState,
  UserSettings,
  BreakRecord,
  DailyStats,
  AnalyticsSummary,
  OnboardingState,
} from "./types";

export async function getTimerState(): Promise<TimerState> {
  return invoke("get_timer_state");
}

export async function pauseTimer(): Promise<TimerState> {
  return invoke("pause_timer");
}

export async function resumeTimer(): Promise<TimerState> {
  return invoke("resume_timer");
}

export async function skipBreak(): Promise<TimerState> {
  return invoke("skip_break");
}

export async function resetTimer(): Promise<TimerState> {
  return invoke("reset_timer");
}

export async function getSettings(): Promise<UserSettings> {
  return invoke("get_settings");
}

export async function updateSettings(
  settings: UserSettings
): Promise<UserSettings> {
  return invoke("update_settings", { settings });
}

export async function getAnalyticsSummary(): Promise<AnalyticsSummary> {
  return invoke("get_analytics_summary");
}

export async function getBreakHistory(
  limit: number,
  offset: number
): Promise<BreakRecord[]> {
  return invoke("get_break_history", { limit, offset });
}

export async function getDailyStatsRange(
  from: string,
  to: string
): Promise<DailyStats[]> {
  return invoke("get_daily_stats_range", { from, to });
}

export async function exportDataCsv(): Promise<string> {
  return invoke("export_data_csv");
}

export async function clearAllData(): Promise<boolean> {
  return invoke("clear_all_data");
}

// --- Onboarding commands ---

export async function getOnboardingState(): Promise<OnboardingState> {
  return invoke("get_onboarding_state");
}

export async function completeOnboarding(): Promise<OnboardingState> {
  return invoke("complete_onboarding");
}

export async function markTooltipSeen(
  tooltipId: string
): Promise<string[]> {
  return invoke("mark_tooltip_seen", { tooltipId });
}

export async function triggerDemoBreak(): Promise<boolean> {
  return invoke("trigger_demo_break");
}

export async function resetOnboarding(): Promise<boolean> {
  return invoke("reset_onboarding");
}
