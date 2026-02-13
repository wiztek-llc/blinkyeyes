import type { DailyStats } from "../lib/types";

function dayName(dateStr: string): string {
  const d = new Date(dateStr + "T00:00:00");
  return d.toLocaleDateString(undefined, { weekday: "short" });
}

export default function DailyChart({ days }: { days: DailyStats[] }) {
  const maxBreaks = Math.max(
    ...days.map((d) => d.breaks_completed + d.breaks_skipped),
    1,
  );

  const allEmpty = days.every(
    (d) => d.breaks_completed === 0 && d.breaks_skipped === 0,
  );

  return (
    <div className="rounded-2xl bg-white dark:bg-gray-800 p-5 space-y-3">
      <h3 className="text-sm font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wide">
        Last 7 days
      </h3>

      <div className="relative">
        <div className="flex items-end gap-2 h-32">
          {days.map((day) => {
            const total = day.breaks_completed + day.breaks_skipped;
            const completedH = (day.breaks_completed / maxBreaks) * 100;
            const skippedH = (day.breaks_skipped / maxBreaks) * 100;

            return (
              <div
                key={day.date}
                className="flex-1 flex flex-col items-center gap-1"
              >
                <div className="w-full flex flex-col justify-end h-24">
                  {total > 0 ? (
                    <>
                      {day.breaks_skipped > 0 && (
                        <div
                          className="w-full rounded-t bg-orange-300 dark:bg-orange-400/60"
                          style={{ height: `${skippedH}%` }}
                        />
                      )}
                      {day.breaks_completed > 0 && (
                        <div
                          className={`w-full bg-green-400 dark:bg-green-400/70 ${day.breaks_skipped > 0 ? "" : "rounded-t"} rounded-b`}
                          style={{ height: `${completedH}%` }}
                        />
                      )}
                    </>
                  ) : (
                    <div className="w-full rounded bg-gray-100 dark:bg-gray-700 h-1" />
                  )}
                </div>
                <span className="text-xs text-gray-400 dark:text-gray-500">
                  {dayName(day.date)}
                </span>
              </div>
            );
          })}
        </div>

        {allEmpty && (
          <div className="absolute inset-0 flex items-center justify-center">
            <p className="text-xs text-gray-400 dark:text-gray-500 bg-white/80 dark:bg-gray-800/80 px-3 py-1.5 rounded-lg">
              Your 7-day history will appear here as you use Blinky.
            </p>
          </div>
        )}
      </div>
    </div>
  );
}
