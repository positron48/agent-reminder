import { useEffect, useState } from "react";
import type {
  AddTimerPayload,
  AgentType,
  RestartTimerPayload,
  Timer,
  TimerScheduleMode,
  TimerSchedulePayload,
} from "../types";
import {
  AGENT_PRESETS,
  agentBadgeClass,
  defaultTargetDatetimeLocal,
  durationMsToParts,
  formatCountdown,
} from "../types";

type FormMode = "add" | "restart";

interface Props {
  mode?: FormMode;
  preset?: AgentType | null;
  restartTimer?: Timer | null;
  onSubmit: (payload: AddTimerPayload | RestartTimerPayload) => Promise<void>;
  onCancel: () => void;
}

export function AddTimerForm({
  mode = "add",
  preset,
  restartTimer,
  onSubmit,
  onCancel,
}: Props) {
  const isRestart = mode === "restart" && restartTimer != null;

  const [agentType, setAgentType] = useState<AgentType>(
    restartTimer?.agentType ?? preset ?? "claude",
  );
  const [agentLabel, setAgentLabel] = useState(
    restartTimer?.agentType === "custom" ? restartTimer.agentLabel : "",
  );
  const [scheduleMode, setScheduleMode] = useState<TimerScheduleMode>("duration");
  const [days, setDays] = useState(0);
  const [hours, setHours] = useState(1);
  const [minutes, setMinutes] = useState(0);
  const [targetAt, setTargetAt] = useState(defaultTargetDatetimeLocal);
  const [comment, setComment] = useState(restartTimer?.comment ?? "");
  const [submitting, setSubmitting] = useState(false);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    if (preset && !isRestart) {
      setAgentType(preset);
    }
  }, [preset, isRestart]);

  useEffect(() => {
    if (!restartTimer) {
      return;
    }

    const parts = durationMsToParts(restartTimer.durationMs);
    setDays(parts.days);
    setHours(parts.hours);
    setMinutes(parts.minutes);
    setComment(restartTimer.comment ?? "");
    setTargetAt(defaultTargetDatetimeLocal());
    setScheduleMode("duration");
    setError(null);
  }, [restartTimer]);

  const targetPreviewMs = (() => {
    if (scheduleMode !== "target" || !targetAt) {
      return null;
    }
    const endsAt = new Date(targetAt).getTime();
    if (Number.isNaN(endsAt)) {
      return null;
    }
    return Math.max(0, endsAt - Date.now());
  })();

  const buildSchedule = (): TimerSchedulePayload => {
    if (scheduleMode === "target") {
      const endsAt = new Date(targetAt).getTime();
      if (Number.isNaN(endsAt)) {
        throw new Error("Enter a valid date and time");
      }
      if (endsAt <= Date.now()) {
        throw new Error("Reset time must be in the future");
      }
      return { endsAt };
    }

    if (days === 0 && hours === 0 && minutes === 0) {
      throw new Error("Duration must be greater than 0");
    }

    return { days, hours, minutes };
  };

  const handleSubmit = async (event: React.FormEvent) => {
    event.preventDefault();
    setSubmitting(true);
    setError(null);

    try {
      const schedule = buildSchedule();
      const trimmedComment = comment.trim() || undefined;

      if (isRestart && restartTimer) {
        await onSubmit({
          id: restartTimer.id,
          ...schedule,
          comment: trimmedComment,
        });
      } else {
        await onSubmit({
          agentType,
          agentLabel: agentType === "custom" ? agentLabel : undefined,
          ...schedule,
          comment: trimmedComment,
        });
        setComment("");
        setAgentLabel("");
      }

      onCancel();
    } catch (err) {
      setError(err instanceof Error ? err.message : String(err));
    } finally {
      setSubmitting(false);
    }
  };

  return (
    <form className="add-form" onSubmit={handleSubmit}>
      {isRestart && restartTimer ? (
        <div className="form-row">
          <label>Agent</label>
          <span className={`agent-badge ${agentBadgeClass(restartTimer.agentType)}`}>
            {restartTimer.agentLabel}
          </span>
        </div>
      ) : (
        <>
          <div className="form-row">
            <label htmlFor="agent-type">Agent</label>
            <select
              id="agent-type"
              value={agentType}
              onChange={(e) => setAgentType(e.target.value as AgentType)}
            >
              {AGENT_PRESETS.map((item) => (
                <option key={item.type} value={item.type}>
                  {item.label}
                </option>
              ))}
            </select>
          </div>

          {agentType === "custom" && (
            <div className="form-row">
              <label htmlFor="agent-label">Name</label>
              <input
                id="agent-label"
                value={agentLabel}
                onChange={(e) => setAgentLabel(e.target.value)}
                placeholder="e.g. Gemini"
                required
              />
            </div>
          )}
        </>
      )}

      <div className="form-row">
        <label>When does the limit reset?</label>
        <div className="schedule-mode" role="tablist" aria-label="Timer mode">
          <button
            type="button"
            role="tab"
            aria-selected={scheduleMode === "duration"}
            className={scheduleMode === "duration" ? "active" : ""}
            onClick={() => setScheduleMode("duration")}
          >
            Duration
          </button>
          <button
            type="button"
            role="tab"
            aria-selected={scheduleMode === "target"}
            className={scheduleMode === "target" ? "active" : ""}
            onClick={() => setScheduleMode("target")}
          >
            Target time
          </button>
        </div>
      </div>

      {scheduleMode === "duration" ? (
        <div className="form-row duration-row">
          <div>
            <label htmlFor="days">Days</label>
            <input
              id="days"
              type="number"
              min={0}
              max={365}
              value={days}
              onChange={(e) => setDays(Number(e.target.value))}
            />
          </div>
          <div>
            <label htmlFor="hours">Hours</label>
            <input
              id="hours"
              type="number"
              min={0}
              max={99}
              value={hours}
              onChange={(e) => setHours(Number(e.target.value))}
            />
          </div>
          <div>
            <label htmlFor="minutes">Minutes</label>
            <input
              id="minutes"
              type="number"
              min={0}
              max={59}
              value={minutes}
              onChange={(e) => setMinutes(Number(e.target.value))}
            />
          </div>
        </div>
      ) : (
        <div className="form-row">
          <label htmlFor="target-at">Reset at</label>
          <input
            id="target-at"
            type="datetime-local"
            value={targetAt}
            onChange={(e) => setTargetAt(e.target.value)}
            required
          />
          {targetPreviewMs != null && targetPreviewMs > 0 && (
            <span className="form-hint">
              Countdown: {formatCountdown(targetPreviewMs)}
            </span>
          )}
        </div>
      )}

      <div className="form-row">
        <label htmlFor="comment">Comment</label>
        <input
          id="comment"
          value={comment}
          onChange={(e) => setComment(e.target.value)}
          placeholder="Optional"
        />
      </div>

      {error && <p className="form-error">{error}</p>}

      <div className="form-actions">
        <button type="button" className="btn-secondary" onClick={onCancel}>
          Cancel
        </button>
        <button type="submit" className="btn-primary" disabled={submitting}>
          {isRestart ? "Restart" : "Add"}
        </button>
      </div>
    </form>
  );
}
