import { useEffect, useState } from "react";
import { listen } from "@tauri-apps/api/event";
import type { TimerState } from "../lib/types";
import { getTimerState } from "../lib/commands";

export function useTimer(): TimerState | null {
  const [state, setState] = useState<TimerState | null>(null);

  useEffect(() => {
    getTimerState().then(setState).catch(console.error);

    const unlisten = listen<TimerState>("timer-tick", (event) => {
      setState(event.payload);
    });

    return () => {
      unlisten.then((fn) => fn());
    };
  }, []);

  return state;
}
