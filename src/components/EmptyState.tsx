interface EmptyStateProps {
  icon: string;
  title: string;
  description: string;
  compact?: boolean;
}

export default function EmptyState({
  icon,
  title,
  description,
  compact,
}: EmptyStateProps) {
  return (
    <div
      className={`flex flex-col items-center justify-center text-center ${
        compact ? "py-3 px-2 gap-1.5" : "py-5 px-4 gap-2"
      }`}
    >
      <span className={compact ? "text-xl" : "text-2xl"}>{icon}</span>
      <p
        className={`font-medium text-gray-600 dark:text-gray-300 ${
          compact ? "text-xs" : "text-sm"
        }`}
      >
        {title}
      </p>
      <p
        className={`text-gray-400 dark:text-gray-500 ${
          compact ? "text-[10px] leading-tight" : "text-xs leading-relaxed"
        }`}
      >
        {description}
      </p>
    </div>
  );
}
