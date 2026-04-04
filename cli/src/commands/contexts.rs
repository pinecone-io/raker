use crate::client::RakerClient;
use crate::config::{load_config, save_config};
use crate::output;
use crate::types::*;
use anyhow::Result;

pub async fn create(
    name: &str,
    environment: Option<&str>,
    description: Option<&str>,
    guardrails: Option<&str>,
    json: bool,
) -> Result<()> {
    let cfg = load_config()?;
    let client = RakerClient::new(&cfg)?;

    let guardrails_content = read_if_file(guardrails)?;

    let req = CreateContextRequest {
        name: name.to_string(),
        environment: environment.map(|s| s.to_string()),
        description: description.map(|s| s.to_string()),
        guardrails: guardrails_content,
    };
    let context = client.create_context(&req).await?;
    output::print_context(&context, json);
    Ok(())
}

pub async fn list(json: bool) -> Result<()> {
    let cfg = load_config()?;
    let client = RakerClient::new(&cfg)?;
    let contexts = client.list_contexts().await?;
    output::print_contexts(&contexts, json);
    Ok(())
}

pub async fn delete(context_id: &str) -> Result<()> {
    let cfg = load_config()?;
    let client = RakerClient::new(&cfg)?;
    client.delete_context(context_id).await?;
    eprintln!("Context {context_id} deleted.");
    Ok(())
}

/// Switch to a context for the current session (persists to config).
pub async fn switch(context_id: &str, json: bool) -> Result<()> {
    // Validate the context exists
    let cfg = load_config()?;
    let client = RakerClient::new(&cfg)?;
    let context = client.get_context(context_id).await?;

    // Persist the selection
    let mut cfg = load_config()?;
    cfg.context_id = Some(context_id.to_string());
    save_config(&cfg)?;

    if json {
        output::print_json(&serde_json::json!({
            "context_id": context_id,
            "name": context.name,
        }));
    } else {
        println!(
            "Switched to context {} ({})",
            context_id,
            context.name.as_deref().unwrap_or("-")
        );
    }
    Ok(())
}

/// Show the currently active context.
pub async fn which(json: bool) -> Result<()> {
    let cfg = load_config()?;

    // Check env var first
    let from_env = std::env::var("RAKER_CONTEXT_ID")
        .ok()
        .filter(|s| !s.is_empty());
    let context_id = from_env.as_deref().or(cfg.context_id.as_deref());

    match context_id {
        Some(id) => {
            let source = if from_env.is_some() {
                "RAKER_CONTEXT_ID"
            } else {
                "config"
            };
            if json {
                output::print_json(&serde_json::json!({
                    "context_id": id,
                    "source": source,
                }));
            } else {
                println!("{} (from {})", id, source);
            }
        }
        None => {
            if json {
                output::print_json(&serde_json::json!({
                    "context_id": null,
                    "source": null,
                }));
            } else {
                println!("No active context.");
            }
        }
    }
    Ok(())
}

/// If the value starts with @ read from file. Otherwise return as-is.
fn read_if_file(val: Option<&str>) -> Result<Option<String>> {
    match val {
        None => Ok(None),
        Some(s) if s.starts_with('@') => {
            let path = &s[1..];
            let content = std::fs::read_to_string(path)
                .map_err(|e| anyhow::anyhow!("failed to read file {path}: {e}"))?;
            Ok(Some(content))
        }
        Some(s) => Ok(Some(s.to_string())),
    }
}
