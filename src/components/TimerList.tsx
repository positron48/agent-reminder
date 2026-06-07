import type { Timer } from "../types";
import { formatCountdown } from "../types";
import { TimerItem } from "./TimerItem";

interface Props {
  activeTimers: Timer[];
  completedTimers: Timer[];
  now: number;
  onComplete: (id: string) => void;
  onRemove: (id: string) => void;
  onClearCompleted: () => void;
}

export function TimerList({
  activeTimers,
  completedTimers,
  now,
  onComplete,
  onRemove,
  onClearCompleted,
}: Props) {
  if (activeTimers.length === 0 && completedTimers.length === 0) {
    return (
      <div className="empty-state">
        <p>No active timers</p>
        <span>Add an agent limit to track when it resets</span>
      </div>
    );
  }

  return (
    <div className="timer-list">
      {activeTimers.length > 0 && (
        <section>
          <h2 className="section-title">Waiting</h2>
          {activeTimers.map((timer) => (
            <TimerItem
              key={timer.id}
              timer={timer}
              now={now}
              onComplete={onComplete}
              onRemove={onRemove}
            />
          ))}
        </section>
      )}

      {completedTimers.length > 0 && (
        <section>
          <div className="section-header">
            <h2 className="section-title">Available</h2>
            <button
              type="button"
              className="link-btn"
              onClick={onClearCompleted}
            >
              Clear
            </button>
          </div>
          {completedTimers.map((timer) => (
            <TimerItem
              key={timer.id}
              timer={timer}
              now={now}
              onComplete={onComplete}
              onRemove={onRemove}
            />
          ))}
        </section>
      )}
    </div>
  );
}

interface NextProps {
  nearest: Timer | null;
  now: number;
  availableCount: number;
}

export function NextAvailable({ nearest, now, availableCount }: NextProps) {
  if (!nearest && availableCount === 0) {
    return (
      <header className="hero idle">
        <span className="hero-label">Status</span>
        <strong className="hero-value">All agents available</strong>
        <span className="hero-sub">Add a timer when you hit a limit</span>
      </header>
    );
  }

  if (!nearest) {
    return (
      <header className="hero ready">
        <span className="hero-label">Available</span>
        <strong className="hero-value">
          {availableCount} agent{availableCount === 1 ? "" : "s"}
        </strong>
        <span className="hero-sub">Ready to go</span>
      </header>
    );
  }

  const remaining = Math.max(0, nearest.endsAt - now);

  return (
    <header className="hero waiting">
      <span className="hero-label">Next available</span>
      <strong className="hero-value">{nearest.agentLabel}</strong>
      <span className="hero-countdown">{formatCountdown(remaining)}</span>
      {nearest.comment && (
        <span className="hero-sub">{nearest.comment}</span>
      )}
    </header>
  );
}
