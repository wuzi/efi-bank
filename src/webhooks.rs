use reqwest::Method;

use crate::client::Client;
use crate::error::Error;
use crate::types::{WebhookPayload, WebhookResponse, WebhooksListResponse};

impl Client {
    pub async fn webhook_create(&self, payload: &WebhookPayload) -> Result<WebhookResponse, Error> {
        self.send_authenticated(Method::POST, "/v2/webhook", Some(payload))
            .await
    }

    pub async fn webhook_update(
        &self,
        webhook_id: &str,
        payload: &WebhookPayload,
    ) -> Result<WebhookResponse, Error> {
        let path = format!("/v2/webhook/{webhook_id}");
        self.send_authenticated(Method::PUT, &path, Some(payload))
            .await
    }

    pub async fn webhook_get(&self, webhook_id: &str) -> Result<WebhookResponse, Error> {
        let path = format!("/v2/webhook/{webhook_id}");
        self.send_authenticated::<serde_json::Value, WebhookResponse>(Method::GET, &path, None)
            .await
    }

    pub async fn webhook_list(&self) -> Result<WebhooksListResponse, Error> {
        self.send_authenticated::<serde_json::Value, WebhooksListResponse>(
            Method::GET,
            "/v2/webhook",
            None,
        )
        .await
    }

    pub async fn webhook_delete(&self, webhook_id: &str) -> Result<(), Error> {
        let path = format!("/v2/webhook/{webhook_id}");
        self.send_authenticated::<serde_json::Value, serde_json::Value>(
            Method::DELETE,
            &path,
            None,
        )
        .await?;
        Ok(())
    }

    pub async fn webhook_config(
        &self,
        chave: &str,
        payload: &WebhookPayload,
        skip_mtls: bool,
    ) -> Result<WebhookResponse, Error> {
        let path = format!("/v2/webhook/{chave}");
        let mut req = self.http.put(format!("{}{}", self.endpoints().pix_api_base_url, path))
            .bearer_auth(self.get_valid_access_token().await?)
            .json(payload);
        if skip_mtls {
            req = req.header("x-skip-mtls", "true").header("x-skip-mtls-checking", "true");
        }
        let resp = req.send().await?;
        Self::parse_response(resp).await
    }
}
