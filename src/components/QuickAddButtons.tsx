import type { AgentType } from "../types";
import { AGENT_PRESETS } from "../types";

interface Props {
  onSelect: (agentType: AgentType) => void;
}

export function QuickAddButtons({ onSelect }: Props) {
  return (
    <div className="quick-add">
      {AGENT_PRESETS.map((preset) => (
        <button
          key={preset.type}
          type="button"
          className="quick-add-btn"
          onClick={() => onSelect(preset.type)}
        >
          + {preset.label}
        </button>
      ))}
    </div>
  );
}
