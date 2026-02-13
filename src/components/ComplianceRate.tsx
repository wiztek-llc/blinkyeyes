import type { DailyStats } from "../lib/types";
import EmptyState from "./EmptyState";

export default function ComplianceRate({ today }: { today: DailyStats }) {
  const rate = Math.round(today.compliance_rate * 100);
  const noBreaks =
    today.breaks_completed === 0 && today.breaks_skipped === 0;

  if (noBreaks) {
    return (
      <div className="rounded-2xl bg-white dark:bg-gray-800 p-5">
        <h3 className="text-sm font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wide mb-1">
          Compliance
        </h3>
        <EmptyState
          icon="📊"
          title="0%"
          description="Take your first break to see your completion rate here."
          compact
        />
      </div>
    );
  }

  return (
    <div className="rounded-2xl bg-white dark:bg-gray-800 p-5 space-y-3">
      <h3 className="text-sm font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wide">
        Compliance
      </h3>

      <div className="flex items-baseline gap-1">
        <span className="text-3xl font-bold">{rate}</span>
        <span className="text-lg text-gray-400">%</span>
      </div>

      <div className="flex gap-4 text-sm">
        <span className="text-green-600 dark:text-green-400">
          {today.breaks_completed} completed
        </span>
        <span className="text-orange-500 dark:text-orange-400">
          {today.breaks_skipped} skipped
        </span>
      </div>
    </div>
  );
}
