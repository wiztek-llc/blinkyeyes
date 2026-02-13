import type { UserSettings } from "../../lib/types";

interface PreviewStepProps {
  onNext: () => void;
  onBack: () => void;
  settings: Partial<UserSettings>;
  onUpdateSettings: (partial: Partial<UserSettings>) => void;
}

export default function PreviewStep({ onNext, onBack }: PreviewStepProps) {
  return (
    <div className="flex flex-col items-center justify-center h-full text-center px-6">
      <p className="text-lg text-gray-600 dark:text-gray-300 mb-8">
        Preview your break reminder
      </p>
      <div className="flex gap-3">
        <button
          onClick={onBack}
          className="px-4 py-2 text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-200 transition-colors"
        >
          Back
        </button>
        <button
          onClick={onNext}
          className="px-6 py-3 bg-blue-600 text-white rounded-xl font-medium hover:bg-blue-700 transition-colors"
        >
          Next
        </button>
      </div>
    </div>
  );
}
