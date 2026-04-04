use anyhow::{Context as AnyhowContext, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

const CONFIG_DIR: &str = ".raker";
const CONFIG_FILE: &str = "cli.json";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CliConfig {
    /// Base URL for the Autocontext API (e.g. https://alpha.autocontext.io)
    pub api_url: String,
    /// JWT bearer token obtained from login
    pub token: Option<String>,
    /// Currently active context ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context_id: Option<String>,
}

impl Default for CliConfig {
    fn default() -> Self {
        let mut api_url = "https://alpha.autocontext.io".to_string();

        // Fallback to checking the system config JSON for the API URL
        if let Some(home) = dirs::home_dir() {
            let system_cfg_path = home.join(CONFIG_DIR).join("cli.json");
            if let Ok(data) = std::fs::read_to_string(system_cfg_path) {
                if let Ok(json) = serde_json::from_str::<serde_json::Value>(&data) {
                    if let Some(url) = json
                        .get("host")
                        .and_then(|h| h.get("url"))
                        .and_then(|u| u.as_str())
                    {
                        api_url = url.to_string();
                    }
                }
            }
        }

        Self {
            api_url,
            token: None,
            context_id: None,
        }
    }
}

/// Resolve the context ID from (in priority order):
/// 1. Explicit --context-id flag
/// 2. RAKER_CONTEXT_ID env var
/// 3. Persisted config from `raker context switch`
pub fn resolve_context_id(flag: Option<&str>) -> Result<String> {
    if let Some(id) = flag {
        return Ok(id.to_string());
    }
    if let Ok(id) = std::env::var("RAKER_CONTEXT_ID") {
        if !id.is_empty() {
            return Ok(id);
        }
    }
    let cfg = load_config()?;
    if let Some(id) = cfg.context_id {
        return Ok(id);
    }
    anyhow::bail!(
        "no context active — use --context-id, set RAKER_CONTEXT_ID, or run `raker context switch <id>`"
    )
}

/// Returns ~/.raker/cli.json
fn config_path() -> Result<PathBuf> {
    let home = dirs::home_dir().context("cannot determine home directory")?;
    Ok(home.join(CONFIG_DIR).join(CONFIG_FILE))
}

pub fn load_config() -> Result<CliConfig> {
    let path = config_path()?;
    let cfg = if !path.exists() {
        CliConfig::default()
    } else {
        let data = std::fs::read_to_string(&path)
            .with_context(|| format!("failed to read {}", path.display()))?;
        serde_json::from_str(&data)
            .with_context(|| format!("invalid JSON in {}", path.display()))?
    };
    Ok(cfg)
}

pub fn save_config(cfg: &CliConfig) -> Result<()> {
    let path = config_path()?;
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let data = serde_json::to_string_pretty(cfg)?;
    std::fs::write(&path, data)?;
    Ok(())
}
