import { useState } from "react";
import type { UserSettings } from "../../lib/types";
import { triggerDemoBreak } from "../../lib/commands";

interface PreviewStepProps {
  onNext: () => void;
  onBack: () => void;
  settings: Partial<UserSettings>;
  onUpdateSettings: (partial: Partial<UserSettings>) => void;
  onGoToStep?: (step: number) => void;
}

export default function PreviewStep({
  onNext,
  onBack,
}: PreviewStepProps) {
  const [demoActive, setDemoActive] = useState(false);

  async function handleTryIt() {
    if (demoActive) return;
    setDemoActive(true);
    try {
      await triggerDemoBreak();
    } catch {
      // Demo break may fail silently — don't block the wizard
    }
    setDemoActive(false);
  }

  // Static preview of the overlay pill (matches MiniOverlay appearance)
  const ringSize = 30;
  const strokeWidth = 2.5;
  const radius = (ringSize - strokeWidth) / 2;
  const circumference = 2 * Math.PI * radius;

  return (
    <div className="flex flex-col h-full px-6 py-5">
      <h2 className="text-lg font-semibold text-center mb-4">
        Here's what a break looks like
      </h2>

      <div className="flex-1 flex flex-col items-center">
        {/* Static preview of overlay */}
        <div className="w-full max-w-xs mb-4">
          <div className="mx-auto rounded-full bg-gray-900/80 backdrop-blur-md px-4 py-2.5 flex items-center gap-2.5 shadow-lg scale-90">
            {/* Mini progress ring */}
            <div
              className="relative flex items-center justify-center shrink-0"
              style={{ width: ringSize, height: ringSize }}
            >
              <svg
                width={ringSize}
                height={ringSize}
                className="absolute -rotate-90"
              >
                <circle
                  cx={ringSize / 2}
                  cy={ringSize / 2}
                  r={radius}
                  fill="none"
                  stroke="rgba(255,255,255,0.15)"
                  strokeWidth={strokeWidth}
                />
                <circle
                  cx={ringSize / 2}
                  cy={ringSize / 2}
                  r={radius}
                  fill="none"
                  stroke="#4ade80"
                  strokeWidth={strokeWidth}
                  strokeLinecap="round"
                  strokeDasharray={circumference}
                  strokeDashoffset={circumference * 0.35}
                />
              </svg>
              <span className="text-white text-[10px] font-bold tabular-nums">
                14s
              </span>
            </div>

            <span className="text-white/90 text-xs select-none">
              Look away — rest your eyes
            </span>

            <span className="text-white/30 text-[10px] ml-auto">skip</span>
          </div>
        </div>

        {/* Try it button */}
        <button
          onClick={handleTryIt}
          disabled={demoActive}
          className={`mb-5 px-5 py-2 rounded-xl text-sm font-medium transition-colors ${
            demoActive
              ? "bg-gray-300 dark:bg-gray-600 text-gray-500 dark:text-gray-400 cursor-not-allowed"
              : "bg-green-50 dark:bg-green-950/30 text-green-700 dark:text-green-300 hover:bg-green-100 dark:hover:bg-green-900/40 border border-green-200 dark:border-green-800"
          }`}
        >
          {demoActive ? "Break in progress..." : "Try a break now"}
        </button>

        {/* Explanatory text */}
        <div className="max-w-sm space-y-3 text-center">
          <p className="text-xs text-gray-500 dark:text-gray-400 leading-relaxed">
            When it's time for a break, this gentle reminder will appear at the
            top of your screen. It won't steal your focus or interrupt what
            you're doing — you'll see a 20-second countdown, and when it's done,
            a soft chime lets you know.
          </p>

          <p className="text-xs text-gray-500 dark:text-gray-400 leading-relaxed italic">
            Don't worry — you can always skip a break if you're in the middle of
            something important.
          </p>
        </div>
      </div>

      {/* Navigation */}
      <div className="flex justify-between items-center pt-4">
        <button
          onClick={onBack}
          className="px-4 py-2 text-sm text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-200 transition-colors"
        >
          Back
        </button>
        <button
          onClick={onNext}
          className="px-6 py-2.5 bg-blue-600 text-white rounded-xl text-sm font-medium hover:bg-blue-700 active:bg-blue-800 transition-colors"
        >
          Next
        </button>
      </div>
    </div>
  );
}
