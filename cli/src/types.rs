use serde::{Deserialize, Serialize};

// ── Auth ──

#[derive(Debug, Serialize)]
pub struct LoginRequest {
    pub api_key: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginResponse {
    pub token: String,
}

// ── Context ──

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Context {
    pub id: Option<String>,
    pub environment: Option<String>,
    pub name: Option<String>,
    pub created_by: Option<String>,
    pub description: Option<String>,
    pub guardrails: Option<String>,
    pub index_id: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextStats {
    pub tasks_total: Option<i64>,
    pub tasks_active: Option<i64>,
    pub tasks_completed: Option<i64>,
    pub tasks_failed: Option<i64>,
    pub tasks_cancelled: Option<i64>,
    pub tokens_total: Option<i64>,
    pub runtime_seconds: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextWithStats {
    #[serde(flatten)]
    pub context: Context,
    pub stats: Option<ContextStats>,
}

#[derive(Debug, Serialize)]
pub struct CreateContextRequest {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guardrails: Option<String>,
}

// ── Task ──

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: Option<String>,
    pub context_id: Option<String>,
    pub created_by: Option<String>,
    pub workflow: Option<String>,
    pub state: Option<String>,
    pub error: Option<String>,
    pub input: Option<serde_json::Value>,
    pub output: Option<serde_json::Value>,
    pub steps: Option<Vec<serde_json::Value>>,
    pub tokens_prompt: Option<i64>,
    pub tokens_completion: Option<i64>,
    pub runtime_seconds: Option<i64>,
    pub running_from: Option<String>,
    pub timeout_seconds: Option<i64>,
    pub timeout_at: Option<String>,
    pub archived_at: Option<String>,
    pub created_at: Option<String>,
    pub last_activity_at: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct CreateTaskRequest {
    pub instruction: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_seconds: Option<u64>,
}

// ── Global Stats ──

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalStats {
    pub tasks_total: Option<i64>,
    pub tasks_active: Option<i64>,
    pub tasks_completed: Option<i64>,
    pub tasks_cancelled: Option<i64>,
    pub tasks_by_state: Option<serde_json::Value>,
    pub tasks_by_workflow: Option<serde_json::Value>,
    pub captured_at: Option<String>,
}
