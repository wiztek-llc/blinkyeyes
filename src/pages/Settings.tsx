import { useState } from "react";
import { useSettings } from "../hooks/useSettings";
import { exportDataCsv, clearAllData } from "../lib/commands";
import type { UserSettings } from "../lib/types";

function Toggle({
  label,
  checked,
  onChange,
}: {
  label: string;
  checked: boolean;
  onChange: (v: boolean) => void;
}) {
  return (
    <label className="flex items-center justify-between py-2">
      <span className="text-sm">{label}</span>
      <button
        type="button"
        role="switch"
        aria-checked={checked}
        onClick={() => onChange(!checked)}
        className={`relative w-10 h-6 rounded-full transition-colors ${
          checked
            ? "bg-blue-500"
            : "bg-gray-300 dark:bg-gray-600"
        }`}
      >
        <span
          className={`absolute top-0.5 left-0.5 w-5 h-5 rounded-full bg-white shadow transition-transform ${
            checked ? "translate-x-4" : ""
          }`}
        />
      </button>
    </label>
  );
}

function SliderField({
  label,
  value,
  min,
  max,
  step,
  unit,
  onChange,
}: {
  label: string;
  value: number;
  min: number;
  max: number;
  step: number;
  unit: string;
  onChange: (v: number) => void;
}) {
  return (
    <div className="py-2 space-y-1">
      <div className="flex items-center justify-between">
        <span className="text-sm">{label}</span>
        <span className="text-sm font-medium tabular-nums">
          {value} {unit}
        </span>
      </div>
      <input
        type="range"
        min={min}
        max={max}
        step={step}
        value={value}
        onChange={(e) => onChange(Number(e.target.value))}
        className="w-full accent-blue-500"
      />
    </div>
  );
}

function Section({
  title,
  children,
}: {
  title: string;
  children: React.ReactNode;
}) {
  return (
    <div className="rounded-2xl bg-white dark:bg-gray-800 p-5 space-y-1">
      <h3 className="text-sm font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wide mb-2">
        {title}
      </h3>
      {children}
    </div>
  );
}

export default function Settings() {
  const { settings, saving, error, save } = useSettings();
  const [confirmClear, setConfirmClear] = useState(false);
  const [exportPath, setExportPath] = useState<string | null>(null);

  if (!settings) {
    return (
      <div className="text-center text-gray-400 py-12">Loading settings...</div>
    );
  }

  function update(partial: Partial<UserSettings>) {
    save({ ...settings!, ...partial });
  }

  return (
    <div className="space-y-4 max-w-lg mx-auto">
      {saving && (
        <p className="text-xs text-blue-500 text-center">Saving...</p>
      )}
      {error && (
        <p className="text-xs text-red-500 text-center">{error}</p>
      )}

      <Section title="Timer">
        <SliderField
          label="Work interval"
          value={settings.work_interval_minutes}
          min={1}
          max={60}
          step={1}
          unit="min"
          onChange={(v) => update({ work_interval_minutes: v })}
        />
        <SliderField
          label="Break duration"
          value={settings.break_duration_seconds}
          min={5}
          max={120}
          step={5}
          unit="sec"
          onChange={(v) => update({ break_duration_seconds: v })}
        />
        <div className="py-2">
          <div className="flex items-center justify-between">
            <span className="text-sm">Daily break goal</span>
            <input
              type="number"
              min={1}
              max={100}
              value={settings.daily_goal}
              onChange={(e) =>
                update({ daily_goal: Math.max(1, Number(e.target.value)) })
              }
              className="w-16 text-right text-sm font-medium bg-gray-100 dark:bg-gray-700 rounded-lg px-2 py-1 border-0 outline-none focus:ring-2 focus:ring-blue-500"
            />
          </div>
        </div>
      </Section>

      <Section title="Notifications">
        <Toggle
          label="System notifications"
          checked={settings.notification_enabled}
          onChange={(v) => update({ notification_enabled: v })}
        />
        <Toggle
          label="Overlay reminder"
          checked={settings.overlay_enabled}
          onChange={(v) => update({ overlay_enabled: v })}
        />
        <Toggle
          label="Completion sound"
          checked={settings.sound_enabled}
          onChange={(v) => update({ sound_enabled: v })}
        />
        {settings.sound_enabled && (
          <SliderField
            label="Volume"
            value={Math.round(settings.sound_volume * 100)}
            min={0}
            max={100}
            step={5}
            unit="%"
            onChange={(v) => update({ sound_volume: v / 100 })}
          />
        )}
      </Section>

      <Section title="System">
        <Toggle
          label="Launch at login"
          checked={settings.launch_at_login}
          onChange={(v) => update({ launch_at_login: v })}
        />
        <SliderField
          label="Auto-pause when idle"
          value={settings.idle_pause_minutes}
          min={0}
          max={15}
          step={1}
          unit={settings.idle_pause_minutes === 0 ? "(off)" : "min"}
          onChange={(v) => update({ idle_pause_minutes: v })}
        />
        <div className="py-2">
          <div className="flex items-center justify-between">
            <span className="text-sm">Theme</span>
            <select
              value={settings.theme}
              onChange={(e) => update({ theme: e.target.value })}
              className="text-sm bg-gray-100 dark:bg-gray-700 rounded-lg px-2 py-1 border-0 outline-none focus:ring-2 focus:ring-blue-500"
            >
              <option value="system">System</option>
              <option value="light">Light</option>
              <option value="dark">Dark</option>
            </select>
          </div>
        </div>
      </Section>

      <div className="rounded-2xl border-2 border-red-200 dark:border-red-900/40 p-5 space-y-3">
        <h3 className="text-sm font-medium text-red-500 dark:text-red-400 uppercase tracking-wide">
          Danger Zone
        </h3>

        <div className="flex flex-col gap-2">
          <button
            onClick={() => {
              exportDataCsv()
                .then((path) => setExportPath(path))
                .catch(console.error);
            }}
            className="px-4 py-2 rounded-lg bg-gray-200 dark:bg-gray-700 text-sm font-medium hover:bg-gray-300 dark:hover:bg-gray-600 transition-colors text-left"
          >
            Export data as CSV
          </button>
          {exportPath && (
            <p className="text-xs text-green-600 dark:text-green-400">
              Saved to {exportPath}
            </p>
          )}

          {confirmClear ? (
            <div className="flex items-center gap-2">
              <span className="text-sm text-red-500">Are you sure?</span>
              <button
                onClick={() => {
                  clearAllData()
                    .then(() => {
                      setConfirmClear(false);
                      window.location.reload();
                    })
                    .catch(console.error);
                }}
                className="px-3 py-1.5 rounded-lg bg-red-500 text-white text-sm font-medium hover:bg-red-600 transition-colors"
              >
                Yes, clear everything
              </button>
              <button
                onClick={() => setConfirmClear(false)}
                className="px-3 py-1.5 rounded-lg bg-gray-200 dark:bg-gray-700 text-sm font-medium hover:bg-gray-300 dark:hover:bg-gray-600 transition-colors"
              >
                Cancel
              </button>
            </div>
          ) : (
            <button
              onClick={() => setConfirmClear(true)}
              className="px-4 py-2 rounded-lg bg-red-100 text-red-700 dark:bg-red-900/30 dark:text-red-300 text-sm font-medium hover:bg-red-200 dark:hover:bg-red-900/50 transition-colors text-left"
            >
              Clear all data
            </button>
          )}
        </div>
      </div>
    </div>
  );
}
