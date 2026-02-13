import { useTimer } from "../hooks/useTimer";
import { useAnalytics } from "../hooks/useAnalytics";
import { useSettings } from "../hooks/useSettings";
import TimerStatus from "../components/TimerStatus";
import StreakCard from "../components/StreakCard";
import DailyChart from "../components/DailyChart";
import WeeklyHeatmap from "../components/WeeklyHeatmap";
import ComplianceRate from "../components/ComplianceRate";

function formatDuration(totalSeconds: number): string {
  const mins = Math.floor(totalSeconds / 60);
  if (mins < 60) return `${mins} minute${mins !== 1 ? "s" : ""}`;
  const hrs = Math.floor(mins / 60);
  const rem = mins % 60;
  return rem > 0 ? `${hrs}h ${rem}m` : `${hrs} hour${hrs !== 1 ? "s" : ""}`;
}

export default function Dashboard() {
  const timer = useTimer();
  const { data: analytics } = useAnalytics();
  const { settings } = useSettings();

  return (
    <div className="space-y-4 max-w-lg mx-auto">
      <TimerStatus timer={timer} />

      {analytics && (
        <>
          <div className="grid grid-cols-2 gap-4">
            <StreakCard
              analytics={analytics}
              dailyGoal={settings?.daily_goal ?? 24}
            />
            <ComplianceRate today={analytics.today} />
          </div>

          <DailyChart days={analytics.last_7_days} />

          <WeeklyHeatmap days={analytics.last_7_days} />

          {(analytics.lifetime_breaks > 0 ||
            analytics.lifetime_rest_seconds > 0) && (
            <p className="text-center text-sm text-gray-400 dark:text-gray-500 pt-2">
              {analytics.lifetime_breaks.toLocaleString()} lifetime break
              {analytics.lifetime_breaks !== 1 ? "s" : ""} &middot;{" "}
              {formatDuration(analytics.lifetime_rest_seconds)} of rest
            </p>
          )}
        </>
      )}
    </div>
  );
}
