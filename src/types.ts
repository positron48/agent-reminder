export type AgentType = "claude" | "codex" | "cursor" | "custom";

export type TimerStatus = "active" | "completed";

export type TrayStatusKind = "idle" | "available" | "waiting" | "soon";

export interface Timer {
  id: string;
  agentType: AgentType;
  agentLabel: string;
  durationMs: number;
  startedAt: number;
  endsAt: number;
  comment?: string;
  status: TimerStatus;
  notified: boolean;
}

export interface TraySummary {
  availableCount: number;
  waitingCount: number;
  nearestMs: number | null;
  nearestLabel: string | null;
  status: TrayStatusKind;
}

export interface AppSettings {
  soundEnabled: boolean;
}

export interface AddTimerPayload {
  agentType: string;
  agentLabel?: string;
  days: number;
  hours: number;
  minutes: number;
  comment?: string;
}

export const AGENT_PRESETS: { type: AgentType; label: string }[] = [
  { type: "claude", label: "Claude" },
  { type: "codex", label: "Codex" },
  { type: "cursor", label: "Cursor" },
  { type: "custom", label: "Custom" },
];

export function formatCountdown(ms: number): string {
  const total = Math.max(0, Math.floor(ms / 1000));
  const days = Math.floor(total / 86400);
  const hours = Math.floor((total % 86400) / 3600);
  const minutes = Math.floor((total % 3600) / 60);
  const seconds = total % 60;
  if (days > 0) {
    return `${days}d ${hours}:${String(minutes).padStart(2, "0")}:${String(seconds).padStart(2, "0")}`;
  }
  if (hours > 0) {
    return `${hours}:${String(minutes).padStart(2, "0")}:${String(seconds).padStart(2, "0")}`;
  }
  return `${String(minutes).padStart(2, "0")}:${String(seconds).padStart(2, "0")}`;
}

export function agentBadgeClass(agentType: AgentType): string {
  switch (agentType) {
    case "claude":
      return "badge-claude";
    case "codex":
      return "badge-codex";
    case "cursor":
      return "badge-cursor";
    default:
      return "badge-custom";
  }
}
