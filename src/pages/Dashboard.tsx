import { useCallback, useEffect, useRef, useState } from "react";
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
import Tooltip, { PulsingDot } from "../components/Tooltip";

function formatDuration(totalSeconds: number): string {
  const mins = Math.floor(totalSeconds / 60);
  if (mins < 60) return `${mins} minute${mins !== 1 ? "s" : ""}`;
  const hrs = Math.floor(mins / 60);
  const rem = mins % 60;
  return rem > 0 ? `${hrs}h ${rem}m` : `${hrs} hour${hrs !== 1 ? "s" : ""}`;
}

const TOOLTIP_SEQUENCE = [
  {
    id: "timer",
    title: "Your work timer",
    description:
      "When this reaches zero, you'll get a gentle reminder to look away for 20 seconds. You can pause or skip anytime.",
    position: "bottom" as const,
  },
  {
    id: "streak",
    title: "Build a streak",
    description: "", // filled dynamically with daily_goal
    position: "bottom" as const,
  },
  {
    id: "compliance",
    title: "Completion rate",
    description:
      "This shows what percentage of break reminders you completed instead of skipping. Don't stress about 100% — any breaks are good for your eyes!",
    position: "bottom" as const,
  },
  {
    id: "chart",
    title: "Your history",
    description:
      "This chart tracks your daily breaks over the past week. Green means completed, orange means skipped.",
    position: "top" as const,
  },
];

export default function Dashboard() {
  const timer = useTimer();
  const { data: analytics } = useAnalytics();
  const { settings } = useSettings();
  const { state: onboardingState, markTooltipSeen } = useOnboarding();

  const isFirstDay = onboardingState?.is_first_day ?? false;
  const tooltipsSeen = onboardingState?.tooltips_seen ?? [];

  const [bannerDismissed, setBannerDismissed] = useState(false);
  const [showCelebration, setShowCelebration] = useState(false);
  const celebrationTimer = useRef<ReturnType<typeof setTimeout> | null>(null);

  // Tooltip sequencing state
  const [activeTooltip, setActiveTooltip] = useState<string | null>(null);
  const tooltipInitialized = useRef(false);

  // Refs for tooltip targets
  const timerRef = useRef<HTMLDivElement>(null);
  const streakRef = useRef<HTMLDivElement>(null);
  const complianceRef = useRef<HTMLDivElement>(null);
  const chartRef = useRef<HTMLDivElement>(null);

  const refMap: Record<string, React.RefObject<HTMLDivElement | null>> = {
    timer: timerRef,
    streak: streakRef,
    compliance: complianceRef,
    chart: chartRef,
  };

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

  // Initialize tooltip sequence with 2-second delay on first dashboard visit
  useEffect(() => {
    if (tooltipInitialized.current || !onboardingState) return;
    if (!onboardingState.onboarding_completed) return;

    tooltipInitialized.current = true;

    const unseenTooltips = TOOLTIP_SEQUENCE.filter(
      (t) => !onboardingState.tooltips_seen.includes(t.id),
    );
    if (unseenTooltips.length === 0) return;

    const timeout = setTimeout(() => {
      setActiveTooltip(unseenTooltips[0].id);
    }, 2000);

    return () => clearTimeout(timeout);
  }, [onboardingState]);

  const handleTooltipDismiss = useCallback(
    (id: string) => {
      markTooltipSeen(id).then((updatedSeen) => {
        setActiveTooltip(null);

        // Find the next unseen tooltip
        const currentIndex = TOOLTIP_SEQUENCE.findIndex((t) => t.id === id);
        const nextTooltip = TOOLTIP_SEQUENCE.slice(currentIndex + 1).find(
          (t) => !updatedSeen.includes(t.id),
        );

        if (nextTooltip) {
          // Brief delay before showing the next tooltip
          setTimeout(() => {
            setActiveTooltip(nextTooltip.id);
          }, 300);
        }
      });
    },
    [markTooltipSeen],
  );

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

  // Build dynamic streak tooltip description
  const dailyGoal = settings?.daily_goal ?? 24;
  const tooltipDescriptions: Record<string, string> = {
    streak: `Complete your daily break goal every day to keep your streak going. Your goal is set to ${dailyGoal} breaks per day.`,
  };

  return (
    <div className="space-y-4 max-w-lg mx-auto">
      <div ref={timerRef} className="relative">
        <PulsingDot
          visible={
            !tooltipsSeen.includes("timer") && activeTooltip !== "timer"
          }
        />
        <TimerStatus timer={timer} isFirstDay={isFirstDay} />
      </div>

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
            <div ref={streakRef} className="relative">
              <PulsingDot
                visible={
                  !tooltipsSeen.includes("streak") &&
                  activeTooltip !== "streak"
                }
              />
              <StreakCard
                analytics={analytics}
                dailyGoal={dailyGoal}
                isFirstDay={isFirstDay}
              />
            </div>
            <div ref={complianceRef} className="relative">
              <PulsingDot
                visible={
                  !tooltipsSeen.includes("compliance") &&
                  activeTooltip !== "compliance"
                }
              />
              <ComplianceRate today={analytics.today} />
            </div>
          </div>

          <div ref={chartRef} className="relative">
            <PulsingDot
              visible={
                !tooltipsSeen.includes("chart") && activeTooltip !== "chart"
              }
            />
            <DailyChart days={analytics.last_7_days} />
          </div>

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

      {/* Active tooltip */}
      {activeTooltip &&
        TOOLTIP_SEQUENCE.map(
          (t) =>
            t.id === activeTooltip && (
              <Tooltip
                key={t.id}
                id={t.id}
                title={t.title}
                description={tooltipDescriptions[t.id] ?? t.description}
                position={t.position}
                targetRef={refMap[t.id]}
                seen={tooltipsSeen}
                onDismiss={handleTooltipDismiss}
              />
            ),
        )}
    </div>
  );
}
