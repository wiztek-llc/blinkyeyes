import { useEffect, useRef, useState } from "react";
import { listen } from "@tauri-apps/api/event";
import { useTimer } from "../hooks/useTimer";
import { useAnalytics } from "../hooks/useAnalytics";
import { useSettings } from "../hooks/useSettings";
import { useOnboarding } from "../hooks/useOnboarding";
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
  const { state: onboardingState } = useOnboarding();

  const isFirstDay = onboardingState?.is_first_day ?? false;

  const [bannerDismissed, setBannerDismissed] = useState(false);
  const [showCelebration, setShowCelebration] = useState(false);
  const celebrationTimer = useRef<ReturnType<typeof setTimeout> | null>(null);

  // Restore banner dismissed state from sessionStorage
  useEffect(() => {
    if (sessionStorage.getItem("blinky-first-day-banner-dismissed") === "1") {
      setBannerDismissed(true);
    }
  }, []);

  // Listen for first-break-celebrated event
  useEffect(() => {
    const unlisten = listen("first-break-celebrated", () => {
      setShowCelebration(true);
      // Auto-dismiss after 10 seconds
      celebrationTimer.current = setTimeout(() => {
        setShowCelebration(false);
      }, 10000);
    });

    return () => {
      unlisten.then((fn) => fn());
      if (celebrationTimer.current) {
        clearTimeout(celebrationTimer.current);
      }
    };
  }, []);

  const dismissCelebration = () => {
    setShowCelebration(false);
    if (celebrationTimer.current) {
      clearTimeout(celebrationTimer.current);
      celebrationTimer.current = null;
    }
  };

  const dismissBanner = () => {
    setBannerDismissed(true);
    sessionStorage.setItem("blinky-first-day-banner-dismissed", "1");
  };

  return (
    <div className="space-y-4 max-w-lg mx-auto">
      <TimerStatus timer={timer} isFirstDay={isFirstDay} />

      {showCelebration && (
        <div className="relative rounded-xl bg-green-50 dark:bg-green-950/30 px-4 py-4 animate-celebration-enter">
          <button
            onClick={dismissCelebration}
            className="absolute top-2 right-3 text-green-400 hover:text-green-600 dark:text-green-400/70 dark:hover:text-green-300 transition-colors"
            aria-label="Dismiss celebration"
          >
            &times;
          </button>
          <div className="flex items-start gap-3">
            <span className="text-2xl shrink-0" aria-hidden="true">
              &#10024;
            </span>
            <div>
              <p className="font-semibold text-green-800 dark:text-green-200">
                You took your first break!
              </p>
              <p className="text-sm text-green-600 dark:text-green-400 mt-1">
                That's 20 seconds of rest your eyes needed. Keep it up — each
                break makes a difference.
              </p>
            </div>
          </div>
        </div>
      )}

      {isFirstDay && !bannerDismissed && (
        <div className="flex items-center justify-between rounded-xl bg-blue-50 dark:bg-blue-950/30 px-4 py-3">
          <p className="text-sm text-blue-700 dark:text-blue-300">
            Welcome to your first day with Blinky! Each break you take gets
            tracked here.
          </p>
          <button
            onClick={dismissBanner}
            className="ml-3 shrink-0 text-blue-400 hover:text-blue-600 dark:text-blue-400/70 dark:hover:text-blue-300 transition-colors"
            aria-label="Dismiss banner"
          >
            &times;
          </button>
        </div>
      )}

      {analytics && (
        <>
          <div className="grid grid-cols-2 gap-4">
            <StreakCard
              analytics={analytics}
              dailyGoal={settings?.daily_goal ?? 24}
              isFirstDay={isFirstDay}
            />
            <ComplianceRate today={analytics.today} />
          </div>

          <DailyChart days={analytics.last_7_days} />

          <WeeklyHeatmap days={analytics.last_7_days} />

          {analytics.lifetime_breaks === 0 &&
          analytics.lifetime_rest_seconds === 0 ? (
            <p className="text-center text-sm text-gray-400 dark:text-gray-500 pt-2">
              Start your lifetime stats with your first eye break.
            </p>
          ) : (
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
