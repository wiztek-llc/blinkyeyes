import type { UserSettings } from "../../lib/types";

interface WelcomeStepProps {
  onNext: () => void;
  onBack: () => void;
  settings: Partial<UserSettings>;
  onUpdateSettings: (partial: Partial<UserSettings>) => void;
  onGoToStep?: (step: number) => void;
}

const RULE_CARDS = [
  {
    number: "20",
    label: "minutes",
    description: "Work timer",
    icon: "\u{1F4BB}",
    animClass: "animate-stagger-in-1",
  },
  {
    number: "20",
    label: "feet away",
    description: "Distance vision",
    icon: "\u{1F440}",
    animClass: "animate-stagger-in-2",
  },
  {
    number: "20",
    label: "seconds",
    description: "Quick rest",
    icon: "\u{2728}",
    animClass: "animate-stagger-in-3",
  },
] as const;

export default function WelcomeStep({ onNext }: WelcomeStepProps) {
  return (
    <div className="flex flex-col items-center justify-center h-full px-8 py-6">
      {/* Hero / Branding */}
      <div className="text-center mb-6">
        <div className="text-4xl mb-3">{"\u{1F441}\uFE0F"}</div>
        <h1 className="text-2xl font-bold tracking-tight mb-1">Blinky</h1>
        <p className="text-sm text-gray-500 dark:text-gray-400">
          Gentle reminders to rest your eyes
        </p>
      </div>

      {/* 20-20-20 Rule Cards */}
      <div className="flex gap-3 mb-5 w-full max-w-sm">
        {RULE_CARDS.map((card) => (
          <div
            key={card.icon}
            className={`flex-1 rounded-2xl bg-blue-50 dark:bg-blue-950/30 p-3 text-center ${card.animClass}`}
          >
            <div className="text-xl mb-1">{card.icon}</div>
            <div className="text-3xl font-bold text-blue-600 dark:text-blue-400 leading-tight">
              {card.number}
            </div>
            <div className="text-xs font-medium text-gray-700 dark:text-gray-300 mt-0.5">
              {card.label}
            </div>
            <div className="text-xs text-gray-500 dark:text-gray-400 mt-1">
              {card.description}
            </div>
          </div>
        ))}
      </div>

      {/* Context */}
      <p className="text-xs text-gray-500 dark:text-gray-400 text-center max-w-sm leading-relaxed mb-6">
        Eye strain from screens is one of the most common health issues for
        people who work at computers. The 20-20-20 rule is a simple, proven way
        to reduce it.
      </p>

      {/* CTA */}
      <div className="text-center">
        <p className="text-sm text-gray-600 dark:text-gray-300 mb-4">
          Let's set up Blinky to work the way you want.
        </p>
        <button
          onClick={onNext}
          className="px-8 py-3 bg-blue-600 text-white rounded-xl font-medium hover:bg-blue-700 active:bg-blue-800 transition-colors text-sm"
        >
          Get started
        </button>
      </div>
    </div>
  );
}
