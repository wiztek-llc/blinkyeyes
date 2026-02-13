import type { TimerState } from "../lib/types";
import {
  pauseTimer,
  resumeTimer,
  skipBreak,
  resetTimer,
} from "../lib/commands";

function formatTime(seconds: number): string {
  const m = Math.floor(seconds / 60);
  const s = seconds % 60;
  return `${String(m).padStart(2, "0")}:${String(s).padStart(2, "0")}`;
}

function phaseLabel(phase: string): string {
  switch (phase) {
    case "Working":
      return "Next break in";
    case "Breaking":
      return "Look away!";
    case "Paused":
      return "Paused";
    case "Suspended":
      return "Suspended (idle)";
    default:
      return "";
  }
}

export default function TimerStatus({
  timer,
  isFirstDay,
}: {
  timer: TimerState | null;
  isFirstDay?: boolean;
}) {
  if (!timer) {
    return (
      <div className="rounded-2xl bg-white dark:bg-gray-800 p-6 text-center">
        <p className="text-gray-400">Loading...</p>
      </div>
    );
  }

  const progress =
    timer.phase_duration > 0
      ? 1 - timer.seconds_remaining / timer.phase_duration
      : 0;
  const isBreaking = timer.phase === "Breaking";
  const isPaused = timer.phase === "Paused";

  return (
    <div className="rounded-2xl bg-white dark:bg-gray-800 p-6 text-center space-y-4">
      <p className="text-sm font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wide">
        {phaseLabel(timer.phase)}
      </p>

      <p className="text-5xl font-bold tabular-nums tracking-tight">
        {isBreaking
          ? `${timer.seconds_remaining}s`
          : formatTime(timer.seconds_remaining)}
      </p>

      <div className="h-1.5 w-full rounded-full bg-gray-100 dark:bg-gray-700 overflow-hidden">
        <div
          className={`h-full rounded-full transition-all duration-1000 ease-linear ${
            isBreaking ? "bg-green-400" : "bg-blue-400"
          }`}
          style={{ width: `${Math.min(progress * 100, 100)}%` }}
        />
      </div>

      <div className="flex items-center justify-center gap-2">
        {isPaused ? (
          <button
            onClick={() => resumeTimer().catch(console.error)}
            className="px-4 py-2 rounded-lg bg-blue-500 text-white text-sm font-medium hover:bg-blue-600 transition-colors"
          >
            Resume
          </button>
        ) : (
          <button
            onClick={() => pauseTimer().catch(console.error)}
            className="px-4 py-2 rounded-lg bg-gray-200 dark:bg-gray-700 text-sm font-medium hover:bg-gray-300 dark:hover:bg-gray-600 transition-colors"
          >
            Pause
          </button>
        )}

        {isBreaking && (
          <button
            onClick={() => skipBreak().catch(console.error)}
            className="px-4 py-2 rounded-lg bg-orange-100 text-orange-700 dark:bg-orange-900/30 dark:text-orange-300 text-sm font-medium hover:bg-orange-200 dark:hover:bg-orange-900/50 transition-colors"
          >
            Skip
          </button>
        )}

        <button
          onClick={() => resetTimer().catch(console.error)}
          className="px-4 py-2 rounded-lg bg-gray-200 dark:bg-gray-700 text-sm font-medium hover:bg-gray-300 dark:hover:bg-gray-600 transition-colors"
        >
          Reset
        </button>
      </div>

      <p className="text-sm text-gray-400 dark:text-gray-500">
        {timer.breaks_completed_today} break
        {timer.breaks_completed_today !== 1 ? "s" : ""} completed today
      </p>

      {isFirstDay && timer.breaks_completed_today === 0 && !isBreaking && (
        <p className="text-xs text-blue-400 dark:text-blue-400/70">
          Your first break is coming up — you'll see a gentle reminder when it's
          time.
        </p>
      )}
    </div>
  );
}
