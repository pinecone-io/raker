use crate::client::RakerClient;
use crate::config::load_config;
use crate::output;
use crate::types::*;
use anyhow::Result;

#[allow(dead_code)]
pub async fn retrieve(
    context_id: &str,
    instruction: &str,
    background: Option<bool>,
    json: bool,
) -> Result<()> {
    let cfg = load_config()?;
    let client = RakerClient::new(&cfg)?;

    let instruction_str = if let Some(path) = instruction.strip_prefix('@') {
        std::fs::read_to_string(path)
            .map_err(|e| anyhow::anyhow!("failed to read instruction file {path}: {e}"))?
    } else {
        instruction.to_string()
    };

    let req = CreateTaskRequest {
        instruction: instruction_str,
        background,
        timeout_seconds: None,
    };
    let task = client.create_task(context_id, "retrieve", &req).await?;
    output::print_task(&task, json);
    Ok(())
}

pub async fn curate(context_id: &str, json: bool) -> Result<()> {
    let cfg = load_config()?;
    let client = RakerClient::new(&cfg)?;
    let result = client.create_curate_task(context_id).await?;

    if json {
        output::print_json(&result);
    } else {
        println!("Curate task scheduled successfully.");
    }
    Ok(())
}

pub async fn build(context_id: &str, json: bool) -> Result<()> {
    let cfg = load_config()?;
    let client = RakerClient::new(&cfg)?;
    let result = client.create_build_task(context_id).await?;

    if json {
        output::print_json(&result);
    } else {
        println!("Build loop started successfully.");
    }
    Ok(())
}
