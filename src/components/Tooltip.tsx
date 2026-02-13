import { useEffect, useRef, useState, type RefObject } from "react";

interface TooltipProps {
  id: string;
  title: string;
  description: string;
  position: "top" | "bottom" | "left" | "right";
  targetRef: RefObject<HTMLElement | null>;
  seen: string[];
  onDismiss: (id: string) => void;
}

export default function Tooltip({
  id,
  title,
  description,
  position,
  targetRef,
  seen,
  onDismiss,
}: TooltipProps) {
  const tooltipRef = useRef<HTMLDivElement>(null);
  const [coords, setCoords] = useState<{ top: number; left: number } | null>(
    null,
  );

  const alreadySeen = seen.includes(id);

  // Position the tooltip relative to the target element
  useEffect(() => {
    if (alreadySeen) return;
    const target = targetRef.current;
    const tooltip = tooltipRef.current;
    if (!target || !tooltip) return;

    const updatePosition = () => {
      const targetRect = target.getBoundingClientRect();
      const tooltipRect = tooltip.getBoundingClientRect();
      const gap = 10;

      let top = 0;
      let left = 0;

      switch (position) {
        case "top":
          top = targetRect.top - tooltipRect.height - gap;
          left =
            targetRect.left + targetRect.width / 2 - tooltipRect.width / 2;
          break;
        case "bottom":
          top = targetRect.bottom + gap;
          left =
            targetRect.left + targetRect.width / 2 - tooltipRect.width / 2;
          break;
        case "left":
          top =
            targetRect.top + targetRect.height / 2 - tooltipRect.height / 2;
          left = targetRect.left - tooltipRect.width - gap;
          break;
        case "right":
          top =
            targetRect.top + targetRect.height / 2 - tooltipRect.height / 2;
          left = targetRect.right + gap;
          break;
      }

      // Clamp to viewport
      const padding = 8;
      left = Math.max(
        padding,
        Math.min(left, window.innerWidth - tooltipRect.width - padding),
      );
      top = Math.max(
        padding,
        Math.min(top, window.innerHeight - tooltipRect.height - padding),
      );

      setCoords({ top, left });
    };

    updatePosition();

    window.addEventListener("resize", updatePosition);
    return () => window.removeEventListener("resize", updatePosition);
  }, [alreadySeen, position, targetRef]);

  // Close on click-outside
  useEffect(() => {
    if (alreadySeen) return;

    const handleClickOutside = (e: MouseEvent) => {
      if (
        tooltipRef.current &&
        !tooltipRef.current.contains(e.target as Node)
      ) {
        onDismiss(id);
      }
    };

    // Delay attaching to avoid immediately dismissing
    const timeout = setTimeout(() => {
      document.addEventListener("mousedown", handleClickOutside);
    }, 100);

    return () => {
      clearTimeout(timeout);
      document.removeEventListener("mousedown", handleClickOutside);
    };
  }, [alreadySeen, id, onDismiss]);

  if (alreadySeen) return null;

  const arrowClasses: Record<string, string> = {
    top: "bottom-[-6px] left-1/2 -translate-x-1/2 border-l-transparent border-r-transparent border-b-transparent border-t-white dark:border-t-gray-700",
    bottom:
      "top-[-6px] left-1/2 -translate-x-1/2 border-l-transparent border-r-transparent border-t-transparent border-b-white dark:border-b-gray-700",
    left: "right-[-6px] top-1/2 -translate-y-1/2 border-t-transparent border-b-transparent border-r-transparent border-l-white dark:border-l-gray-700",
    right:
      "left-[-6px] top-1/2 -translate-y-1/2 border-t-transparent border-b-transparent border-l-transparent border-r-white dark:border-r-gray-700",
  };

  return (
    <div
      ref={tooltipRef}
      className="fixed z-50 max-w-[250px] rounded-xl bg-white dark:bg-gray-700 shadow-lg shadow-black/10 dark:shadow-black/30 p-4 animate-tooltip-enter"
      style={
        coords
          ? { top: coords.top, left: coords.left }
          : { opacity: 0, pointerEvents: "none" }
      }
    >
      {/* Arrow */}
      <div
        className={`absolute w-0 h-0 border-[6px] ${arrowClasses[position]}`}
      />

      <p className="text-sm font-semibold text-gray-800 dark:text-gray-100 mb-1">
        {title}
      </p>
      <p className="text-xs text-gray-500 dark:text-gray-300 leading-relaxed mb-3">
        {description}
      </p>
      <div className="flex justify-end">
        <button
          onClick={() => onDismiss(id)}
          className="text-xs font-medium text-blue-500 hover:text-blue-600 dark:text-blue-400 dark:hover:text-blue-300 transition-colors"
        >
          Got it
        </button>
      </div>
    </div>
  );
}

export function PulsingDot({ visible }: { visible: boolean }) {
  if (!visible) return null;
  return (
    <span className="absolute -top-1 -right-1 flex h-3 w-3 z-10">
      <span className="animate-tooltip-pulse absolute inline-flex h-full w-full rounded-full bg-blue-400 opacity-75" />
      <span className="relative inline-flex rounded-full h-3 w-3 bg-blue-500" />
    </span>
  );
}
