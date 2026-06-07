import type { Timer } from "../types";
import { agentBadgeClass, formatCountdown } from "../types";

interface Props {
  timer: Timer;
  now: number;
  onComplete: (id: string) => void;
  onRemove: (id: string) => void;
}

export function TimerItem({
  timer,
  now,
  onComplete,
  onRemove,
}: Props) {
  const remaining =
    timer.status === "active" ? Math.max(0, timer.endsAt - now) : 0;
  const isCompleted = timer.status === "completed";

  return (
    <article className={`timer-item ${isCompleted ? "completed" : "active"}`}>
      <div className="timer-main">
        <div className="timer-header">
          <span className={`agent-badge ${agentBadgeClass(timer.agentType)}`}>
            {timer.agentLabel}
          </span>
          <span className="timer-countdown">
            {isCompleted ? "Ready" : formatCountdown(remaining)}
          </span>
        </div>
        {timer.comment && <p className="timer-comment">{timer.comment}</p>}
      </div>
      <div className="timer-actions">
        {!isCompleted && (
          <button
            type="button"
            className="icon-btn"
            title="Mark ready now"
            onClick={() => onComplete(timer.id)}
          >
            ✓
          </button>
        )}
        <button
          type="button"
          className="icon-btn danger"
          title="Remove"
          onClick={() => onRemove(timer.id)}
        >
          ×
        </button>
      </div>
    </article>
  );
}
