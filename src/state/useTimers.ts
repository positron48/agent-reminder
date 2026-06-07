import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { useCallback, useEffect, useMemo, useState } from "react";
import type {
  AddTimerPayload,
  AppSettings,
  Timer,
  TraySummary,
} from "../types";

export function useTimers() {
  const [timers, setTimers] = useState<Timer[]>([]);
  const [summary, setSummary] = useState<TraySummary | null>(null);
  const [settings, setSettings] = useState<AppSettings>({ soundEnabled: true });
  const [loading, setLoading] = useState(true);
  const [now, setNow] = useState(Date.now());

  const refresh = useCallback(async () => {
    const [nextTimers, nextSummary, nextSettings] = await Promise.all([
      invoke<Timer[]>("get_timers"),
      invoke<TraySummary>("get_tray_summary"),
      invoke<AppSettings>("get_settings"),
    ]);
    setTimers(nextTimers);
    setSummary(nextSummary);
    setSettings(nextSettings);
    setLoading(false);
  }, []);

  useEffect(() => {
    refresh().catch(console.error);

    const unsubs: Array<() => void> = [];
    listen("timers-updated", () => refresh().catch(console.error)).then((un) =>
      unsubs.push(un),
    );
    listen("timer-completed", () => refresh().catch(console.error)).then((un) =>
      unsubs.push(un),
    );

    const tick = window.setInterval(() => setNow(Date.now()), 1000);
    const poll = window.setInterval(() => refresh().catch(console.error), 5000);

    return () => {
      unsubs.forEach((un) => un());
      window.clearInterval(tick);
      window.clearInterval(poll);
    };
  }, [refresh]);

  const activeTimers = useMemo(
    () =>
      timers
        .filter((t) => t.status === "active" && t.endsAt > now)
        .sort((a, b) => a.endsAt - b.endsAt),
    [timers, now],
  );

  const completedTimers = useMemo(
    () => timers.filter((t) => t.status === "completed"),
    [timers],
  );

  const nearestActive = activeTimers[0] ?? null;

  const addTimer = async (payload: AddTimerPayload) => {
    await invoke("add_timer", { payload });
    await refresh();
  };

  const removeTimer = async (id: string) => {
    await invoke("remove_timer", { id });
    await refresh();
  };

  const completeTimer = async (id: string) => {
    await invoke("complete_timer", { id });
    await refresh();
  };

  const restartTimer = async (id: string) => {
    await invoke("restart_timer", { id });
    await refresh();
  };

  const clearCompleted = async () => {
    await invoke("clear_completed");
    await refresh();
  };

  const toggleSound = async () => {
    const next = await invoke<AppSettings>("set_sound_enabled", {
      enabled: !settings.soundEnabled,
    });
    setSettings(next);
  };

  return {
    timers,
    summary,
    settings,
    loading,
    now,
    activeTimers,
    completedTimers,
    nearestActive,
    addTimer,
    removeTimer,
    completeTimer,
    restartTimer,
    clearCompleted,
    toggleSound,
    refresh,
  };
}
