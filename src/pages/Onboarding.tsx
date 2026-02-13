import { useCallback, useRef, useState } from "react";
import type { UserSettings } from "../lib/types";
import { getSettings, updateSettings as updateSettingsCmd } from "../lib/commands";
import WelcomeStep from "../components/onboarding/WelcomeStep";
import SetupStep from "../components/onboarding/SetupStep";
import PreviewStep from "../components/onboarding/PreviewStep";
import ReadyStep from "../components/onboarding/ReadyStep";

const STEPS = [
  { label: "Welcome", Component: WelcomeStep },
  { label: "Setup", Component: SetupStep },
  { label: "Preview", Component: PreviewStep },
  { label: "Ready", Component: ReadyStep },
] as const;

interface OnboardingProps {
  onComplete: () => void;
}

export default function Onboarding({ onComplete }: OnboardingProps) {
  const [currentStep, setCurrentStep] = useState(0);
  const [accumulatedSettings, setAccumulatedSettings] = useState<Partial<UserSettings>>({});
  const [direction, setDirection] = useState<"forward" | "back">("forward");
  const [animating, setAnimating] = useState(false);
  const [saving, setSaving] = useState(false);
  const containerRef = useRef<HTMLDivElement>(null);

  const goNext = useCallback(() => {
    if (animating || currentStep >= STEPS.length - 1) return;
    setDirection("forward");
    setAnimating(true);
    setTimeout(() => {
      setCurrentStep((s) => s + 1);
      setAnimating(false);
    }, 300);
  }, [animating, currentStep]);

  const goBack = useCallback(() => {
    if (animating || currentStep <= 0) return;
    setDirection("back");
    setAnimating(true);
    setTimeout(() => {
      setCurrentStep((s) => s - 1);
      setAnimating(false);
    }, 300);
  }, [animating, currentStep]);

  const goToStep = useCallback(
    (step: number) => {
      if (animating || step < 0 || step >= STEPS.length || step === currentStep) return;
      setDirection(step < currentStep ? "back" : "forward");
      setAnimating(true);
      setTimeout(() => {
        setCurrentStep(step);
        setAnimating(false);
      }, 300);
    },
    [animating, currentStep],
  );

  const handleUpdateSettings = useCallback((partial: Partial<UserSettings>) => {
    setAccumulatedSettings((prev) => ({ ...prev, ...partial }));
  }, []);

  const handleComplete = useCallback(async () => {
    if (saving) return;
    setSaving(true);
    try {
      // Merge accumulated settings onto current settings and save
      const current = await getSettings();
      const merged = { ...current, ...accumulatedSettings };
      await updateSettingsCmd(merged);
      // Then complete onboarding (starts timer, transitions to dashboard)
      await onComplete();
    } catch (err) {
      console.error("Failed to save settings:", err);
    } finally {
      setSaving(false);
    }
  }, [saving, accumulatedSettings, onComplete]);

  const { Component: StepComponent } = STEPS[currentStep];

  const slideClass = animating
    ? direction === "forward"
      ? "animate-slide-out-left"
      : "animate-slide-out-right"
    : "animate-slide-in";

  return (
    <div className="min-h-screen bg-gray-50 dark:bg-gray-900 text-gray-900 dark:text-gray-100 flex flex-col animate-fade-in">
      {/* Step indicator */}
      <div className="flex justify-center pt-8 pb-4">
        <div className="flex items-center gap-3">
          {STEPS.map((step, i) => (
            <div key={step.label} className="flex flex-col items-center gap-1.5">
              <div
                className={`w-2.5 h-2.5 rounded-full transition-all duration-300 ${
                  i < currentStep
                    ? "bg-blue-500 dark:bg-blue-400"
                    : i === currentStep
                      ? "bg-blue-600 dark:bg-blue-300 scale-125"
                      : "bg-gray-300 dark:bg-gray-600"
                }`}
              />
              <span
                className={`text-xs transition-colors duration-300 ${
                  i === currentStep
                    ? "text-blue-600 dark:text-blue-300 font-medium"
                    : "text-gray-400 dark:text-gray-500"
                }`}
              >
                {step.label}
              </span>
            </div>
          ))}
        </div>
      </div>

      {/* Step content */}
      <div ref={containerRef} className="flex-1 overflow-hidden relative">
        <div className={`h-full ${slideClass}`}>
          <StepComponent
            onNext={currentStep === STEPS.length - 1 ? handleComplete : goNext}
            onBack={goBack}
            settings={accumulatedSettings}
            onUpdateSettings={handleUpdateSettings}
            onGoToStep={goToStep}
          />
        </div>
      </div>
    </div>
  );
}
