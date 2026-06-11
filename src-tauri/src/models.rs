use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum AgentType {
    Claude,
    Codex,
    Cursor,
    Custom,
}

impl AgentType {
    pub fn default_label(&self) -> &'static str {
        match self {
            AgentType::Claude => "Claude",
            AgentType::Codex => "Codex",
            AgentType::Cursor => "Cursor",
            AgentType::Custom => "Custom",
        }
    }

    pub fn from_str(value: &str) -> Self {
        match value.to_lowercase().as_str() {
            "claude" => AgentType::Claude,
            "codex" => AgentType::Codex,
            "cursor" => AgentType::Cursor,
            _ => AgentType::Custom,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum TimerStatus {
    Active,
    Completed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Timer {
    pub id: String,
    pub agent_type: AgentType,
    pub agent_label: String,
    pub duration_ms: u64,
    pub started_at: i64,
    pub ends_at: i64,
    pub comment: Option<String>,
    pub status: TimerStatus,
    pub notified: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddTimerPayload {
    pub agent_type: String,
    pub agent_label: Option<String>,
    #[serde(default)]
    pub days: u32,
    #[serde(default)]
    pub hours: u32,
    #[serde(default)]
    pub minutes: u32,
    pub ends_at: Option<i64>,
    pub comment: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RestartTimerPayload {
    pub id: String,
    #[serde(default)]
    pub days: u32,
    #[serde(default)]
    pub hours: u32,
    #[serde(default)]
    pub minutes: u32,
    pub ends_at: Option<i64>,
    pub comment: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TraySummary {
    pub available_count: usize,
    pub waiting_count: usize,
    pub nearest_ms: Option<i64>,
    pub nearest_label: Option<String>,
    pub status: TrayStatusKind,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum TrayStatusKind {
    Idle,
    Available,
    Waiting,
    Soon,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppSettings {
    pub sound_enabled: bool,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            sound_enabled: true,
        }
    }
}

pub fn compute_tray_summary(timers: &[Timer], now_ms: i64) -> TraySummary {
    let completed: Vec<_> = timers
        .iter()
        .filter(|t| t.status == TimerStatus::Completed)
        .collect();
    let active: Vec<_> = timers
        .iter()
        .filter(|t| t.status == TimerStatus::Active && t.ends_at > now_ms)
        .collect();

    let available_count = completed.len();
    let waiting_count = active.len();

    let nearest = active.iter().min_by_key(|t| t.ends_at);
    let nearest_ms = nearest.map(|t| (t.ends_at - now_ms).max(0));
    let nearest_label = nearest.map(|t| t.agent_label.clone());

    let status = if timers.is_empty() {
        TrayStatusKind::Idle
    } else if waiting_count == 0 {
        TrayStatusKind::Available
    } else if nearest_ms.is_some_and(|ms| ms <= 5 * 60 * 1000) {
        TrayStatusKind::Soon
    } else if available_count > 0 {
        TrayStatusKind::Available
    } else {
        TrayStatusKind::Waiting
    };

    TraySummary {
        available_count,
        waiting_count,
        nearest_ms,
        nearest_label,
        status,
    }
}
