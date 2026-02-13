import { useEffect, useRef } from "react";
import { listen } from "@tauri-apps/api/event";
import type { TimerState } from "../lib/types";
import { getSettings } from "../lib/commands";
import chimeUrl from "../assets/chime.wav";

/**
 * Plays a chime sound when a break completes.
 * Respects sound_enabled and sound_volume settings.
 * Audio is played via the Web Audio API through the WebView,
 * which uses the OS's native audio stack (PipeWire/PulseAudio/CoreAudio/WASAPI).
 */
export function useChime() {
  const audioRef = useRef<HTMLAudioElement | null>(null);

  useEffect(() => {
    // Pre-create the Audio element so it's ready to play instantly
    const audio = new Audio(chimeUrl);
    audio.preload = "auto";
    audioRef.current = audio;

    const unlisten = listen<TimerState>("break-completed", async () => {
      try {
        const settings = await getSettings();
        if (!settings.sound_enabled) return;

        const el = audioRef.current;
        if (!el) return;

        el.volume = Math.max(0, Math.min(1, settings.sound_volume));
        el.currentTime = 0;
        await el.play();
      } catch (e) {
        console.error("[chime] playback failed:", e);
      }
    });

    return () => {
      unlisten.then((fn) => fn());
      audioRef.current = null;
    };
  }, []);
}
