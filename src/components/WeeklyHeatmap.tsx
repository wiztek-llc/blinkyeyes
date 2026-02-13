import type { DailyStats } from "../lib/types";

function dayName(dateStr: string): string {
  const d = new Date(dateStr + "T00:00:00");
  return d.toLocaleDateString(undefined, { weekday: "short" });
}

function intensityClass(breaks: number, max: number): string {
  if (breaks === 0) return "bg-gray-100 dark:bg-gray-700";
  const ratio = breaks / max;
  if (ratio <= 0.25) return "bg-green-200 dark:bg-green-900/50";
  if (ratio <= 0.5) return "bg-green-300 dark:bg-green-700/60";
  if (ratio <= 0.75) return "bg-green-400 dark:bg-green-600/70";
  return "bg-green-500 dark:bg-green-500/80";
}

export default function WeeklyHeatmap({ days }: { days: DailyStats[] }) {
  const maxBreaks = Math.max(...days.map((d) => d.breaks_completed), 1);

  const allEmpty = days.every((d) => d.breaks_completed === 0);

  return (
    <div className="rounded-2xl bg-white dark:bg-gray-800 p-5 space-y-3">
      <h3 className="text-sm font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wide">
        This week
      </h3>

      <div className="relative">
        <div className="grid grid-cols-7 gap-1.5">
          {days.map((day) => (
            <div key={day.date} className="flex flex-col items-center gap-1">
              <div
                className={`w-full aspect-square rounded-lg ${intensityClass(day.breaks_completed, maxBreaks)}`}
                title={`${day.date}: ${day.breaks_completed} breaks`}
              />
              <span className="text-[10px] text-gray-400 dark:text-gray-500">
                {dayName(day.date)}
              </span>
            </div>
          ))}
        </div>

        {allEmpty && (
          <div className="absolute inset-0 flex items-center justify-center">
            <p className="text-xs text-gray-400 dark:text-gray-500 bg-white/80 dark:bg-gray-800/80 px-3 py-1.5 rounded-lg text-center leading-relaxed">
              Each day you use Blinky, this grid fills in.
              <br />
              Aim for a solid wall of green!
            </p>
          </div>
        )}
      </div>
    </div>
  );
}
