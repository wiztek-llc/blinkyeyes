import { useCallback, useEffect, useState } from "react";
import { listen } from "@tauri-apps/api/event";
import type { AnalyticsSummary } from "../lib/types";
import { getAnalyticsSummary } from "../lib/commands";

export function useAnalytics(): {
  data: AnalyticsSummary | null;
  loading: boolean;
  error: string | null;
  refresh: () => void;
} {
  const [data, setData] = useState<AnalyticsSummary | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  const refresh = useCallback(() => {
    setLoading(true);
    getAnalyticsSummary()
      .then((summary) => {
        setData(summary);
        setError(null);
      })
      .catch((err) => {
        setError(String(err));
      })
      .finally(() => {
        setLoading(false);
      });
  }, []);

  useEffect(() => {
    refresh();

    const unlisten = listen("break-completed", () => {
      refresh();
    });

    return () => {
      unlisten.then((fn) => fn());
    };
  }, [refresh]);

  return { data, loading, error, refresh };
}
