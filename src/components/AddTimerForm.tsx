import { useEffect, useState } from "react";
import type { AgentType } from "../types";
import { AGENT_PRESETS } from "../types";

interface Props {
  preset?: AgentType | null;
  onSubmit: (payload: {
    agentType: AgentType;
    agentLabel?: string;
    days: number;
    hours: number;
    minutes: number;
    comment?: string;
  }) => Promise<void>;
  onCancel: () => void;
}

export function AddTimerForm({ preset, onSubmit, onCancel }: Props) {
  const [agentType, setAgentType] = useState<AgentType>(preset ?? "claude");
  const [agentLabel, setAgentLabel] = useState("");
  const [days, setDays] = useState(0);
  const [hours, setHours] = useState(1);
  const [minutes, setMinutes] = useState(0);
  const [comment, setComment] = useState("");
  const [submitting, setSubmitting] = useState(false);

  useEffect(() => {
    if (preset) {
      setAgentType(preset);
    }
  }, [preset]);

  const handleSubmit = async (event: React.FormEvent) => {
    event.preventDefault();
    setSubmitting(true);
    try {
      await onSubmit({
        agentType,
        agentLabel: agentType === "custom" ? agentLabel : undefined,
        days,
        hours,
        minutes,
        comment: comment.trim() || undefined,
      });
      setComment("");
      setAgentLabel("");
      onCancel();
    } finally {
      setSubmitting(false);
    }
  };

  return (
    <form className="add-form" onSubmit={handleSubmit}>
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

      <div className="form-row">
        <label htmlFor="comment">Comment</label>
        <input
          id="comment"
          value={comment}
          onChange={(e) => setComment(e.target.value)}
          placeholder="Optional"
        />
      </div>

      <div className="form-actions">
        <button type="button" className="btn-secondary" onClick={onCancel}>
          Cancel
        </button>
        <button type="submit" className="btn-primary" disabled={submitting}>
          Add
        </button>
      </div>
    </form>
  );
}
