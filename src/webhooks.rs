use reqwest::Method;

use crate::client::{Client, query_path, resource_path};
use crate::error::Error;
use crate::types::{WebhookListQuery, WebhookPayload, WebhookResponse, WebhooksListResponse};

impl Client {
    /// Configure/reconfigure by Pix key. Acknowledges HTTP success, not settlement.
    /// The provider does not establish a success JSON schema for this operation.
    pub async fn webhook_configure(
        &self,
        chave: &str,
        payload: &WebhookPayload,
    ) -> Result<(), Error> {
        let path = resource_path(&["v2", "webhook", chave])?;
        self.send_authenticated_status(Method::PUT, &path, Some(payload))
            .await
    }

    pub async fn webhook_get(&self, chave: &str) -> Result<WebhookResponse, Error> {
        let path = resource_path(&["v2", "webhook", chave])?;
        self.send_authenticated::<(), WebhookResponse>(Method::GET, &path, None)
            .await
    }

    pub async fn webhook_list(
        &self,
        query: &WebhookListQuery,
    ) -> Result<WebhooksListResponse, Error> {
        let path = query_path(
            "/v2/webhook",
            &[("inicio", &query.inicio), ("fim", &query.fim)],
        );
        self.send_authenticated::<(), WebhooksListResponse>(Method::GET, &path, None)
            .await
    }

    pub async fn webhook_delete(&self, chave: &str) -> Result<(), Error> {
        let path = resource_path(&["v2", "webhook", chave])?;
        self.send_authenticated_status::<()>(Method::DELETE, &path, None)
            .await
    }
}
