use crate::client::RakerClient;
use crate::config::{load_config, save_config, CliConfig};
use anyhow::Result;

/// Login with a Pinecone API key and persist the JWT token.
/// Resets cli.json completely — clears any previously active context and
/// server URL so stale state from a prior session cannot carry over.
pub async fn login(api_url: Option<&str>, api_key: &str) -> Result<()> {
    // Start from a fresh config, only preserving the api_url from the
    // existing file if --api-url was not explicitly provided.
    let previous = load_config().unwrap_or_default();
    let server_url = match api_url {
        Some(url) => url.trim_end_matches('/').to_string(),
        None => previous.api_url,
    };

    let mut cfg = CliConfig {
        api_url: server_url,
        token: None,
        context_id: None,
    };

    let client = RakerClient::new(&cfg)?;
    let resp = client.login(api_key).await?;
    cfg.token = Some(resp.token.clone());
    save_config(&cfg)?;

    // Create system config if it doesn't exist or update the pinecone api key
    if let Some(home) = dirs::home_dir() {
        let system_cfg_dir = home.join(".raker");
        let system_cfg_path = system_cfg_dir.join("cli.json");

        let mut sys_cfg = serde_json::json!({
            "host": {
                "name": "Raker",
                "url": cfg.api_url
            },
            "gemini_api_key": "",
            "pinecone_api_key": api_key
        });

        if system_cfg_path.exists() {
            if let Ok(data) = std::fs::read_to_string(&system_cfg_path) {
                if let Ok(mut existing_cfg) = serde_json::from_str::<serde_json::Value>(&data) {
                    if let Some(obj) = existing_cfg.as_object_mut() {
                        obj.insert("pinecone_api_key".to_string(), serde_json::json!(api_key));
                        sys_cfg = existing_cfg;
                    }
                }
            }
        } else {
            let _ = std::fs::create_dir_all(&system_cfg_dir);
        }

        if let Ok(data) = serde_json::to_string_pretty(&sys_cfg) {
            let _ = std::fs::write(&system_cfg_path, data);
        }
    }

    eprintln!("Logged in successfully. Token saved to ~/.raker/cli.json");
    Ok(())
}

/// Logout the current user by removing the token
pub async fn logout() -> Result<()> {
    let mut cfg = load_config().unwrap_or_default();
    cfg.token = None;
    save_config(&cfg)?;
    eprintln!("Logged out successfully.");
    Ok(())
}
