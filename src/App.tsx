import { useState } from "react";
import { BellIcon } from "./components/BellIcon";
import { AddTimerForm } from "./components/AddTimerForm";
import { QuickAddButtons } from "./components/QuickAddButtons";
import { NextAvailable, TimerList } from "./components/TimerList";
import { useTimers } from "./state/useTimers";
import type { AddTimerPayload, AgentType, RestartTimerPayload, Timer } from "./types";
import "./styles.css";

function App() {
  const {
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
  } = useTimers();

  const [showForm, setShowForm] = useState(false);
  const [formMode, setFormMode] = useState<"add" | "restart">("add");
  const [formPreset, setFormPreset] = useState<AgentType | null>(null);
  const [restartTarget, setRestartTarget] = useState<Timer | null>(null);

  const closeForm = () => {
    setShowForm(false);
    setFormMode("add");
    setFormPreset(null);
    setRestartTarget(null);
  };

  const openForm = (preset?: AgentType) => {
    setFormMode("add");
    setFormPreset(preset ?? null);
    setRestartTarget(null);
    setShowForm(true);
  };

  const openRestartForm = (timer: Timer) => {
    setFormMode("restart");
    setFormPreset(null);
    setRestartTarget(timer);
    setShowForm(true);
  };

  const handleFormSubmit = async (
    payload: AddTimerPayload | RestartTimerPayload,
  ) => {
    if ("id" in payload) {
      await restartTimer(payload);
      return;
    }
    await addTimer(payload);
  };

  if (loading) {
    return (
      <div className="shell">
        <div className="panel">
          <div className="loading">Loading…</div>
        </div>
      </div>
    );
  }

  return (
    <div className="shell">
      <div className="panel">
      <div className="panel-header">
        <div>
          <h1>Agent Reminder</h1>
          <p className="panel-subtitle">AI agent rate limits</p>
        </div>
        <button
          type="button"
          className={`sound-toggle ${settings.soundEnabled ? "on" : "off"}`}
          onClick={() => toggleSound()}
          title={settings.soundEnabled ? "Sound on" : "Sound off"}
        >
          <BellIcon muted={!settings.soundEnabled} />
        </button>
      </div>

      <NextAvailable
        nearest={nearestActive}
        now={now}
        availableCount={summary?.availableCount ?? 0}
      />

      {!showForm ? (
        <>
          <QuickAddButtons onSelect={(type) => openForm(type)} />
          <button
            type="button"
            className="btn-primary full-width"
            onClick={() => openForm()}
          >
            + New timer
          </button>
        </>
      ) : (
        <AddTimerForm
          mode={formMode}
          preset={formPreset}
          restartTimer={restartTarget}
          onSubmit={handleFormSubmit}
          onCancel={closeForm}
        />
      )}

      <TimerList
        activeTimers={activeTimers}
        completedTimers={completedTimers}
        now={now}
        onComplete={completeTimer}
        onRestart={openRestartForm}
        onRemove={removeTimer}
        onClearCompleted={clearCompleted}
      />
      </div>
    </div>
  );
}

export default App;
