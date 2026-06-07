import { useState } from "react";
import { AddTimerForm } from "./components/AddTimerForm";
import { QuickAddButtons } from "./components/QuickAddButtons";
import { NextAvailable, TimerList } from "./components/TimerList";
import { useTimers } from "./state/useTimers";
import type { AgentType } from "./types";
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
    clearCompleted,
    toggleSound,
  } = useTimers();

  const [showForm, setShowForm] = useState(false);
  const [formPreset, setFormPreset] = useState<AgentType | null>(null);

  const openForm = (preset?: AgentType) => {
    setFormPreset(preset ?? null);
    setShowForm(true);
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
          {settings.soundEnabled ? "🔔" : "🔕"}
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
          preset={formPreset}
          onSubmit={addTimer}
          onCancel={() => {
            setShowForm(false);
            setFormPreset(null);
          }}
        />
      )}

      <TimerList
        activeTimers={activeTimers}
        completedTimers={completedTimers}
        now={now}
        onComplete={completeTimer}
        onRemove={removeTimer}
        onClearCompleted={clearCompleted}
      />
      </div>
    </div>
  );
}

export default App;
