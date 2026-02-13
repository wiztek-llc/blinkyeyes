import { useCallback, useEffect, useState } from "react";
import { listen } from "@tauri-apps/api/event";
import type { OnboardingState } from "../lib/types";
import {
  getOnboardingState,
  completeOnboarding as completeOnboardingCmd,
  markTooltipSeen as markTooltipSeenCmd,
  triggerDemoBreak as triggerDemoBreakCmd,
  resetOnboarding as resetOnboardingCmd,
} from "../lib/commands";

export function useOnboarding() {
  const [state, setState] = useState<OnboardingState | null>(null);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    getOnboardingState()
      .then((s) => {
        setState(s);
        setLoading(false);
      })
      .catch((err) => {
        console.error("Failed to get onboarding state:", err);
        setLoading(false);
      });

    const unlisten = listen<OnboardingState>("onboarding-completed", (event) => {
      setState(event.payload);
    });

    return () => {
      unlisten.then((fn) => fn());
    };
  }, []);

  const completeOnboarding = useCallback(async () => {
    const result = await completeOnboardingCmd();
    setState(result);
    return result;
  }, []);

  const markTooltipSeen = useCallback(async (tooltipId: string) => {
    const seen = await markTooltipSeenCmd(tooltipId);
    setState((prev) =>
      prev ? { ...prev, tooltips_seen: seen } : prev
    );
    return seen;
  }, []);

  const triggerDemoBreak = useCallback(async () => {
    return triggerDemoBreakCmd();
  }, []);

  const resetOnboarding = useCallback(async () => {
    const result = await resetOnboardingCmd();
    if (result) {
      const newState = await getOnboardingState();
      setState(newState);
    }
    return result;
  }, []);

  return {
    state,
    loading,
    completeOnboarding,
    markTooltipSeen,
    triggerDemoBreak,
    resetOnboarding,
  };
}
