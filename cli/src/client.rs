use anyhow::{bail, Context as AnyhowContext, Result};
use reqwest::Client;

use crate::config::CliConfig;
use crate::types::*;

/// HTTP client wrapper for the Autocontext API.
pub struct RakerClient {
    http: Client,
    base_url: String,
    token: Option<String>,
}

impl RakerClient {
    pub fn new(cfg: &CliConfig) -> Result<Self> {
        let http = Client::builder()
            .timeout(std::time::Duration::from_secs(900)) // 15 min for sync tasks
            .build()?;
        Ok(Self {
            http,
            base_url: cfg.api_url.trim_end_matches('/').to_string(),
            token: cfg.token.clone(),
        })
    }

    fn api(&self, path: &str) -> String {
        format!("{}/api/v0{}", self.base_url, path)
    }

    fn auth_header(&self) -> Result<String> {
        let has_system_config = dirs::home_dir()
            .map(|h| h.join(".raker").join("cli.json").exists())
            .unwrap_or(false);

        match &self.token {
            Some(t) => Ok(format!("Bearer {}", t)),
            None => {
                if !has_system_config {
                    bail!("no system config found — run `raker login` first");
                }
                bail!("not authenticated — run `raker login` first")
            }
        }
    }

    /// Handle non-2xx responses by reading the body as text.
    async fn check(resp: reqwest::Response) -> Result<reqwest::Response> {
        let status = resp.status();
        if !status.is_success() {
            let url = resp.url().to_string();
            let body = resp.text().await.unwrap_or_default();
            bail!("API error {status} from {url}: {body}");
        }
        Ok(resp)
    }

    // ── Auth ──

    pub async fn login(&self, api_key: &str) -> Result<LoginResponse> {
        let resp = self
            .http
            .post(self.api("/auth/login"))
            .json(&LoginRequest {
                api_key: api_key.to_string(),
            })
            .send()
            .await
            .context("failed to connect to Autocontext API")?;
        let resp = Self::check(resp).await?;
        Ok(resp.json().await?)
    }

    // ── Stats ──

    pub async fn global_stats(&self) -> Result<GlobalStats> {
        let resp = self
            .http
            .get(self.api("/stats"))
            .header("Authorization", self.auth_header()?)
            .send()
            .await?;
        let resp = Self::check(resp).await?;
        Ok(resp.json().await?)
    }

    // ── Contexts ──

    pub async fn create_context(&self, req: &CreateContextRequest) -> Result<Context> {
        let resp = self
            .http
            .post(self.api("/contexts"))
            .header("Authorization", self.auth_header()?)
            .json(req)
            .send()
            .await?;
        let resp = Self::check(resp).await?;
        Ok(resp.json().await?)
    }

    pub async fn list_contexts(&self) -> Result<Vec<ContextWithStats>> {
        let resp = self
            .http
            .get(self.api("/contexts"))
            .header("Authorization", self.auth_header()?)
            .send()
            .await?;
        let resp = Self::check(resp).await?;
        Ok(resp.json().await?)
    }

    pub async fn get_context(&self, context_id: &str) -> Result<Context> {
        let resp = self
            .http
            .get(self.api(&format!("/contexts/{context_id}")))
            .header("Authorization", self.auth_header()?)
            .send()
            .await?;
        let resp = Self::check(resp).await?;
        Ok(resp.json().await?)
    }

    pub async fn delete_context(&self, context_id: &str) -> Result<()> {
        let resp = self
            .http
            .delete(self.api(&format!("/contexts/{context_id}")))
            .header("Authorization", self.auth_header()?)
            .send()
            .await?;
        Self::check(resp).await?;
        Ok(())
    }

    // ── Tasks ──

    pub async fn create_task(
        &self,
        context_id: &str,
        workflow: &str,
        req: &CreateTaskRequest,
    ) -> Result<Task> {
        let resp = self
            .http
            .post(self.api(&format!("/contexts/{context_id}/{workflow}")))
            .header("Authorization", self.auth_header()?)
            .json(req)
            .send()
            .await?;
        let resp = Self::check(resp).await?;
        Ok(resp.json().await?)
    }
}
