import { useCallback, useEffect, useRef, useState } from "react";
import type { UserSettings } from "../lib/types";
import { getSettings, updateSettings } from "../lib/commands";

export function useSettings(): {
  settings: UserSettings | null;
  saving: boolean;
  error: string | null;
  save: (updated: UserSettings) => void;
} {
  const [settings, setSettings] = useState<UserSettings | null>(null);
  const [saving, setSaving] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const debounceRef = useRef<ReturnType<typeof setTimeout> | null>(null);

  useEffect(() => {
    getSettings().then(setSettings).catch((err) => setError(String(err)));
  }, []);

  const save = useCallback((updated: UserSettings) => {
    setSettings(updated);
    setError(null);

    if (debounceRef.current) {
      clearTimeout(debounceRef.current);
    }

    debounceRef.current = setTimeout(() => {
      setSaving(true);
      updateSettings(updated)
        .then((result) => {
          setSettings(result);
          setError(null);
        })
        .catch((err) => {
          setError(String(err));
        })
        .finally(() => {
          setSaving(false);
        });
    }, 300);
  }, []);

  return { settings, saving, error, save };
}
