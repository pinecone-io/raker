use crate::client::RakerClient;
use crate::config::load_config;
use crate::output;
use anyhow::Result;

pub async fn global(json: bool) -> Result<()> {
    let cfg = load_config()?;
    let client = RakerClient::new(&cfg)?;
    let stats = client.global_stats().await?;
    output::print_global_stats(&stats, json);
    Ok(())
}
