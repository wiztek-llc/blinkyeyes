import { useState } from "react";
import type { UserSettings } from "../../lib/types";

interface SetupStepProps {
  onNext: () => void;
  onBack: () => void;
  settings: Partial<UserSettings>;
  onUpdateSettings: (partial: Partial<UserSettings>) => void;
  onGoToStep?: (step: number) => void;
}

const INTERVAL_PRESETS = [
  { value: 15, label: "15 min", description: "Frequent breaks" },
  { value: 20, label: "20 min", description: "Recommended" },
  { value: 30, label: "30 min", description: "Fewer interruptions" },
] as const;

function applyThemePreview(theme: string) {
  const root = document.documentElement;
  if (theme === "dark") {
    root.classList.add("dark");
  } else if (theme === "light") {
    root.classList.remove("dark");
  } else {
    if (window.matchMedia("(prefers-color-scheme: dark)").matches) {
      root.classList.add("dark");
    } else {
      root.classList.remove("dark");
    }
  }
}

export default function SetupStep({
  onNext,
  onBack,
  settings,
  onUpdateSettings,
}: SetupStepProps) {
  const [showCustom, setShowCustom] = useState(false);

  const interval = settings.work_interval_minutes ?? 20;
  const overlayEnabled = settings.overlay_enabled ?? true;
  const notificationEnabled = settings.notification_enabled ?? true;
  const soundEnabled = settings.sound_enabled ?? true;
  const theme = settings.theme ?? "system";

  const isPreset = INTERVAL_PRESETS.some((p) => p.value === interval) && !showCustom;
  const allNotificationsOff = !overlayEnabled && !notificationEnabled && !soundEnabled;

  function selectInterval(value: number) {
    setShowCustom(false);
    onUpdateSettings({ work_interval_minutes: value });
  }

  function selectCustom() {
    setShowCustom(true);
    if (INTERVAL_PRESETS.some((p) => p.value === interval)) {
      onUpdateSettings({ work_interval_minutes: interval });
    }
  }

  function setTheme(newTheme: string) {
    onUpdateSettings({ theme: newTheme });
    applyThemePreview(newTheme);
  }

  return (
    <div className="flex flex-col h-full px-6 py-5">
      <h2 className="text-lg font-semibold text-center mb-4">Quick setup</h2>

      <div className="flex-1 space-y-4 overflow-y-auto">
        {/* Work interval */}
        <div className="rounded-2xl bg-white dark:bg-gray-800 p-4">
          <h3 className="text-sm font-medium text-gray-500 dark:text-gray-400 mb-3">
            How often should Blinky remind you?
          </h3>
          <div className="grid grid-cols-4 gap-2">
            {INTERVAL_PRESETS.map((preset) => (
              <button
                key={preset.value}
                onClick={() => selectInterval(preset.value)}
                className={`rounded-xl px-2 py-2.5 text-center transition-colors ${
                  isPreset && interval === preset.value
                    ? "bg-blue-600 text-white"
                    : "bg-gray-100 dark:bg-gray-700 text-gray-700 dark:text-gray-300 hover:bg-gray-200 dark:hover:bg-gray-600"
                }`}
              >
                <div className="text-sm font-semibold">{preset.label}</div>
                <div
                  className={`text-xs mt-0.5 ${
                    isPreset && interval === preset.value
                      ? "text-blue-100"
                      : "text-gray-500 dark:text-gray-400"
                  }`}
                >
                  {preset.description}
                </div>
              </button>
            ))}
            <button
              onClick={selectCustom}
              className={`rounded-xl px-2 py-2.5 text-center transition-colors ${
                showCustom
                  ? "bg-blue-600 text-white"
                  : "bg-gray-100 dark:bg-gray-700 text-gray-700 dark:text-gray-300 hover:bg-gray-200 dark:hover:bg-gray-600"
              }`}
            >
              <div className="text-sm font-semibold">Custom</div>
              <div
                className={`text-xs mt-0.5 ${
                  showCustom
                    ? "text-blue-100"
                    : "text-gray-500 dark:text-gray-400"
                }`}
              >
                Set your own
              </div>
            </button>
          </div>
          {showCustom && (
            <div className="mt-3 flex items-center gap-2">
              <input
                type="number"
                min={1}
                max={120}
                value={interval}
                onChange={(e) => {
                  const v = Math.max(1, Math.min(120, Number(e.target.value)));
                  onUpdateSettings({ work_interval_minutes: v });
                }}
                className="w-16 text-center text-sm font-medium bg-gray-100 dark:bg-gray-700 rounded-lg px-2 py-1.5 border-0 outline-none focus:ring-2 focus:ring-blue-500"
              />
              <span className="text-sm text-gray-500 dark:text-gray-400">
                minutes
              </span>
            </div>
          )}
        </div>

        {/* Notifications */}
        <div className="rounded-2xl bg-white dark:bg-gray-800 p-4">
          <h3 className="text-sm font-medium text-gray-500 dark:text-gray-400 mb-3">
            How should Blinky get your attention?
          </h3>
          <div className="space-y-2.5">
            <ToggleRow
              label="Screen overlay"
              description="A gentle reminder floats at the top of your screen"
              checked={overlayEnabled}
              onChange={(v) => onUpdateSettings({ overlay_enabled: v })}
            />
            <ToggleRow
              label="System notification"
              description="A notification appears in your system tray"
              checked={notificationEnabled}
              onChange={(v) => onUpdateSettings({ notification_enabled: v })}
            />
            <ToggleRow
              label="Completion sound"
              description="A soft chime plays when the break is over"
              checked={soundEnabled}
              onChange={(v) => onUpdateSettings({ sound_enabled: v })}
            />
          </div>
          {allNotificationsOff && (
            <p className="text-xs text-amber-600 dark:text-amber-400 mt-2.5">
              Blinky won't be able to remind you with all notifications
              disabled.
            </p>
          )}
        </div>

        {/* Theme */}
        <div className="rounded-2xl bg-white dark:bg-gray-800 p-4">
          <h3 className="text-sm font-medium text-gray-500 dark:text-gray-400 mb-3">
            Theme
          </h3>
          <div className="grid grid-cols-3 gap-2">
            <ThemeButton
              label="System"
              icon={"\u{1F310}"}
              active={theme === "system"}
              onClick={() => setTheme("system")}
            />
            <ThemeButton
              label="Light"
              icon={"\u2600\uFE0F"}
              active={theme === "light"}
              onClick={() => setTheme("light")}
            />
            <ThemeButton
              label="Dark"
              icon={"\u{1F319}"}
              active={theme === "dark"}
              onClick={() => setTheme("dark")}
            />
          </div>
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

function ToggleRow({
  label,
  description,
  checked,
  onChange,
}: {
  label: string;
  description: string;
  checked: boolean;
  onChange: (v: boolean) => void;
}) {
  return (
    <label className="flex items-center gap-3 cursor-pointer">
      <button
        type="button"
        role="switch"
        aria-checked={checked}
        onClick={() => onChange(!checked)}
        className={`relative shrink-0 w-9 h-5.5 rounded-full transition-colors ${
          checked ? "bg-blue-500" : "bg-gray-300 dark:bg-gray-600"
        }`}
      >
        <span
          className={`absolute top-0.5 left-0.5 w-4.5 h-4.5 rounded-full bg-white shadow transition-transform ${
            checked ? "translate-x-3.5" : ""
          }`}
        />
      </button>
      <div className="min-w-0">
        <div className="text-sm font-medium leading-tight">{label}</div>
        <div className="text-xs text-gray-500 dark:text-gray-400 leading-tight">
          {description}
        </div>
      </div>
    </label>
  );
}

function ThemeButton({
  label,
  icon,
  active,
  onClick,
}: {
  label: string;
  icon: string;
  active: boolean;
  onClick: () => void;
}) {
  return (
    <button
      onClick={onClick}
      className={`rounded-xl py-3 text-center transition-colors ${
        active
          ? "bg-blue-600 text-white"
          : "bg-gray-100 dark:bg-gray-700 text-gray-700 dark:text-gray-300 hover:bg-gray-200 dark:hover:bg-gray-600"
      }`}
    >
      <div className="text-lg">{icon}</div>
      <div className="text-xs font-medium mt-1">{label}</div>
    </button>
  );
}
