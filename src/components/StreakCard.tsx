import type { AnalyticsSummary } from "../lib/types";
import EmptyState from "./EmptyState";

export default function StreakCard({
  analytics,
  dailyGoal,
  isFirstDay,
}: {
  analytics: AnalyticsSummary;
  dailyGoal: number;
  isFirstDay?: boolean;
}) {
  const todayBreaks = analytics.today.breaks_completed;
  const goalProgress = Math.min(todayBreaks / dailyGoal, 1);
  const hasNoStreak = analytics.current_day_streak === 0;

  if (hasNoStreak && isFirstDay) {
    return (
      <div className="rounded-2xl bg-white dark:bg-gray-800 p-5">
        <h3 className="text-sm font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wide mb-1">
          Streak
        </h3>
        <EmptyState
          icon="🔥"
          title="Your streak starts today!"
          description="Complete breaks every day to build a streak. Day one begins now."
          compact
        />
      </div>
    );
  }

  return (
    <div className="rounded-2xl bg-white dark:bg-gray-800 p-5 space-y-3">
      <h3 className="text-sm font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wide">
        Streak
      </h3>

      <div className="flex items-baseline gap-2">
        <span className="text-3xl font-bold">
          {analytics.current_day_streak}
        </span>
        <span className="text-sm text-gray-500 dark:text-gray-400">
          day{analytics.current_day_streak !== 1 ? "s" : ""}
        </span>
      </div>

      <p className="text-xs text-gray-400 dark:text-gray-500">
        Best: {analytics.best_day_streak} day
        {analytics.best_day_streak !== 1 ? "s" : ""}
      </p>

      <div className="pt-2 border-t border-gray-100 dark:border-gray-700">
        <div className="flex items-center justify-between text-sm mb-1.5">
          <span className="text-gray-500 dark:text-gray-400">
            Today&apos;s goal
          </span>
          <span className="font-medium">
            {todayBreaks} / {dailyGoal}
          </span>
        </div>
        <div className="h-2 w-full rounded-full bg-gray-100 dark:bg-gray-700 overflow-hidden">
          <div
            className="h-full rounded-full bg-green-400 transition-all"
            style={{ width: `${goalProgress * 100}%` }}
          />
        </div>
      </div>
    </div>
  );
}
