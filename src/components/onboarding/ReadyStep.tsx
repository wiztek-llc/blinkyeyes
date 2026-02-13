import type { UserSettings } from "../../lib/types";

interface ReadyStepProps {
  onNext: () => void;
  onBack: () => void;
  settings: Partial<UserSettings>;
  onUpdateSettings: (partial: Partial<UserSettings>) => void;
  onGoToStep?: (step: number) => void;
}

export default function ReadyStep({
  onNext,
  onBack,
  settings,
  onGoToStep,
}: ReadyStepProps) {
  const interval = settings.work_interval_minutes ?? 20;
  const overlayEnabled = settings.overlay_enabled ?? true;
  const notificationEnabled = settings.notification_enabled ?? true;
  const soundEnabled = settings.sound_enabled ?? true;
  const theme = settings.theme ?? "system";

  // Build notification summary
  const enabledNotifications: string[] = [];
  if (overlayEnabled) enabledNotifications.push("Overlay");
  if (notificationEnabled) enabledNotifications.push("Notification");
  if (soundEnabled) enabledNotifications.push("Sound");
  const notificationSummary =
    enabledNotifications.length > 0
      ? enabledNotifications.join(" + ")
      : "None";

  const themeLabel =
    theme === "dark" ? "Dark mode" : theme === "light" ? "Light mode" : "System default";

  function handleChange() {
    if (onGoToStep) {
      onGoToStep(1); // Navigate to Setup step (index 1)
    } else {
      onBack();
    }
  }

  return (
    <div className="flex flex-col h-full px-6 py-5">
      <h2 className="text-lg font-semibold text-center mb-5">
        You're all set
      </h2>

      <div className="flex-1 flex flex-col items-center">
        {/* Settings summary */}
        <div className="w-full max-w-xs rounded-2xl bg-white dark:bg-gray-800 p-4 mb-5">
          <div className="space-y-3">
            <SummaryRow
              label="Work interval"
              value={`Break every ${interval} minutes`}
            />
            <SummaryRow
              label="Notifications"
              value={notificationSummary}
            />
            <SummaryRow label="Theme" value={themeLabel} />
          </div>
          <button
            onClick={handleChange}
            className="mt-3 text-xs text-blue-600 dark:text-blue-400 hover:text-blue-700 dark:hover:text-blue-300 transition-colors"
          >
            Change settings
          </button>
        </div>

        {/* Encouragement */}
        <div className="max-w-sm text-center mb-6">
          <p className="text-sm text-gray-600 dark:text-gray-300 leading-relaxed">
            Blinky will quietly remind you to rest your eyes throughout the day.
            Small habits make a big difference.
          </p>
        </div>

        {/* Launch button */}
        <button
          onClick={onNext}
          className="px-8 py-3.5 bg-green-600 text-white rounded-2xl font-semibold text-sm hover:bg-green-700 active:bg-green-800 transition-colors shadow-md shadow-green-600/20"
        >
          Start Protecting Your Eyes
        </button>
      </div>

      {/* Back navigation */}
      <div className="flex justify-start pt-4">
        <button
          onClick={onBack}
          className="px-4 py-2 text-sm text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-200 transition-colors"
        >
          Back
        </button>
      </div>
    </div>
  );
}

function SummaryRow({ label, value }: { label: string; value: string }) {
  return (
    <div className="flex justify-between items-center">
      <span className="text-xs text-gray-500 dark:text-gray-400">{label}</span>
      <span className="text-sm font-medium">{value}</span>
    </div>
  );
}
