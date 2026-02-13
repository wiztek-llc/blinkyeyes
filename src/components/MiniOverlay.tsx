import { useEffect, useState } from "react";
import { listen } from "@tauri-apps/api/event";
import { skipBreak, getSettings } from "../lib/commands";
import type { TimerState } from "../lib/types";

function MiniOverlay() {
  const [timer, setTimer] = useState<TimerState | null>(null);
  const [visible, setVisible] = useState(false);
  const [isFirstBreak, setIsFirstBreak] = useState(false);

  useEffect(() => {
    // Check if this is the user's first-ever break
    getSettings()
      .then((s) => {
        setIsFirstBreak(s.onboarding_completed && !s.first_break_completed);
      })
      .catch(() => {});

    const unlistenTick = listen<TimerState>("timer-tick", (event) => {
      setTimer(event.payload);
    });

    const unlistenCompleted = listen<TimerState>("break-completed", () => {
      setVisible(false);
      // After first break completes, it's no longer the first break
      setIsFirstBreak(false);
    });

    const unlistenSkipped = listen<TimerState>("break-skipped", () => {
      setVisible(false);
    });

    const unlistenStarted = listen<TimerState>("break-started", () => {
      setVisible(true);
      // Re-check first break status when a new break starts
      getSettings()
        .then((s) => {
          setIsFirstBreak(s.onboarding_completed && !s.first_break_completed);
        })
        .catch(() => {});
    });

    return () => {
      unlistenTick.then((f) => f());
      unlistenCompleted.then((f) => f());
      unlistenSkipped.then((f) => f());
      unlistenStarted.then((f) => f());
    };
  }, []);

  // Show when we're in breaking phase
  useEffect(() => {
    if (timer?.phase === "Breaking") {
      setVisible(true);
    } else if (timer?.phase === "Working") {
      setVisible(false);
    }
  }, [timer?.phase]);

  if (!visible || !timer || timer.phase !== "Breaking") {
    return <div className="w-full h-full" />;
  }

  const remaining = timer.seconds_remaining;
  const total = timer.phase_duration;
  const progress = total > 0 ? (total - remaining) / total : 0;

  // SVG progress ring parameters
  const ringSize = 36;
  const strokeWidth = 3;
  const radius = (ringSize - strokeWidth) / 2;
  const circumference = 2 * Math.PI * radius;
  const strokeDashoffset = circumference * (1 - progress);

  const handleSkip = () => {
    skipBreak();
  };

  return (
    <div className="w-full h-full flex items-start justify-center pointer-events-none">
      <div
        className="mt-1 flex items-center gap-3 px-5 py-3 rounded-full
                    bg-gray-900/80 backdrop-blur-md
                    pointer-events-auto
                    animate-overlay-enter
                    shadow-lg"
      >
        {/* Progress ring with countdown */}
        <div className="relative flex items-center justify-center" style={{ width: ringSize, height: ringSize }}>
          <svg
            width={ringSize}
            height={ringSize}
            className="absolute -rotate-90"
          >
            {/* Background ring */}
            <circle
              cx={ringSize / 2}
              cy={ringSize / 2}
              r={radius}
              fill="none"
              stroke="rgba(255,255,255,0.15)"
              strokeWidth={strokeWidth}
            />
            {/* Progress ring */}
            <circle
              cx={ringSize / 2}
              cy={ringSize / 2}
              r={radius}
              fill="none"
              stroke="#4ade80"
              strokeWidth={strokeWidth}
              strokeLinecap="round"
              strokeDasharray={circumference}
              strokeDashoffset={strokeDashoffset}
              className="transition-[stroke-dashoffset] duration-1000 ease-linear"
            />
          </svg>
          <span className="text-white text-xs font-bold tabular-nums">
            {remaining}s
          </span>
        </div>

        {/* Eye icon + message */}
        <span className="text-white/90 text-sm select-none">
          {isFirstBreak
            ? "Your first break! Look at something far away..."
            : "Look away — rest your eyes"}
        </span>

        {/* Skip button — de-emphasized */}
        <button
          onClick={handleSkip}
          className="text-white/30 hover:text-white/60 text-xs ml-1 transition-colors cursor-pointer select-none"
        >
          skip
        </button>
      </div>
    </div>
  );
}

export default MiniOverlay;
