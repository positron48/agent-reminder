import type { Timer } from "../types";
import { formatCountdown } from "../types";
import { TimerItem } from "./TimerItem";

interface Props {
  activeTimers: Timer[];
  completedTimers: Timer[];
  now: number;
  onRestart: (id: string) => void;
  onComplete: (id: string) => void;
  onRemove: (id: string) => void;
  onClearCompleted: () => void;
}

export function TimerList({
  activeTimers,
  completedTimers,
  now,
  onRestart,
  onComplete,
  onRemove,
  onClearCompleted,
}: Props) {
  if (activeTimers.length === 0 && completedTimers.length === 0) {
    return (
      <div className="empty-state">
        <p>Нет активных таймеров</p>
        <span>Добавьте лимит агента, чтобы отслеживать сброс</span>
      </div>
    );
  }

  return (
    <div className="timer-list">
      {activeTimers.length > 0 && (
        <section>
          <h2 className="section-title">Ожидают</h2>
          {activeTimers.map((timer) => (
            <TimerItem
              key={timer.id}
              timer={timer}
              now={now}
              onRestart={onRestart}
              onComplete={onComplete}
              onRemove={onRemove}
            />
          ))}
        </section>
      )}

      {completedTimers.length > 0 && (
        <section>
          <div className="section-header">
            <h2 className="section-title">Свободны</h2>
            <button
              type="button"
              className="link-btn"
              onClick={onClearCompleted}
            >
              Очистить
            </button>
          </div>
          {completedTimers.map((timer) => (
            <TimerItem
              key={timer.id}
              timer={timer}
              now={now}
              onRestart={onRestart}
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
        <span className="hero-label">Статус</span>
        <strong className="hero-value">Все агенты свободны</strong>
        <span className="hero-sub">Добавьте таймер при достижении лимита</span>
      </header>
    );
  }

  if (!nearest) {
    return (
      <header className="hero ready">
        <span className="hero-label">Доступно</span>
        <strong className="hero-value">{availableCount} агент(ов)</strong>
        <span className="hero-sub">Можно продолжать работу</span>
      </header>
    );
  }

  const remaining = Math.max(0, nearest.endsAt - now);

  return (
    <header className="hero waiting">
      <span className="hero-label">Ближайший свободный</span>
      <strong className="hero-value">{nearest.agentLabel}</strong>
      <span className="hero-countdown">{formatCountdown(remaining)}</span>
      {nearest.comment && (
        <span className="hero-sub">{nearest.comment}</span>
      )}
    </header>
  );
}
