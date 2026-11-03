use reqwest::Method;

use crate::client::{Client, resource_path};
use crate::error::Error;
use crate::types::{SplitChargeResponse, SplitConfigResponse, SplitPayload};

impl Client {
    pub async fn split_create_config(
        &self,
        payload: &SplitPayload,
    ) -> Result<SplitConfigResponse, Error> {
        self.send_authenticated(Method::POST, "/v2/gn/split/config", Some(payload))
            .await
    }

    pub async fn split_update_config(
        &self,
        config_id: &str,
        payload: &SplitPayload,
    ) -> Result<SplitConfigResponse, Error> {
        let path = resource_path(&["v2", "gn", "split", "config", config_id])?;
        self.send_authenticated(Method::PUT, &path, Some(payload))
            .await
    }

    /// The provider reference carries a maintenance notice; current availability is not established.
    pub async fn split_get_config(&self, config_id: &str) -> Result<SplitConfigResponse, Error> {
        let path = resource_path(&["v2", "gn", "split", "config", config_id])?;
        self.send_authenticated::<serde_json::Value, SplitConfigResponse>(Method::GET, &path, None)
            .await
    }

    pub async fn split_link_cob(&self, txid: &str, split_config_id: &str) -> Result<(), Error> {
        let path = resource_path(&["v2", "gn", "split", "cob", txid, "vinculo", split_config_id])?;
        self.send_authenticated_status::<()>(Method::PUT, &path, None)
            .await
    }

    pub async fn split_get_cob(&self, txid: &str) -> Result<SplitChargeResponse, Error> {
        let path = resource_path(&["v2", "gn", "split", "cob", txid])?;
        self.send_authenticated::<serde_json::Value, SplitChargeResponse>(Method::GET, &path, None)
            .await
    }

    pub async fn split_unlink_cob(&self, txid: &str) -> Result<(), Error> {
        let path = resource_path(&["v2", "gn", "split", "cob", txid, "vinculo"])?;
        self.send_authenticated_status::<()>(Method::DELETE, &path, None)
            .await
    }

    pub async fn split_link_cobv(&self, txid: &str, split_config_id: &str) -> Result<(), Error> {
        let path = resource_path(&[
            "v2",
            "gn",
            "split",
            "cobv",
            txid,
            "vinculo",
            split_config_id,
        ])?;
        self.send_authenticated_status::<()>(Method::PUT, &path, None)
            .await
    }

    pub async fn split_get_cobv(&self, txid: &str) -> Result<SplitChargeResponse, Error> {
        let path = resource_path(&["v2", "gn", "split", "cobv", txid])?;
        self.send_authenticated::<serde_json::Value, SplitChargeResponse>(Method::GET, &path, None)
            .await
    }

    pub async fn split_unlink_cobv(&self, txid: &str) -> Result<(), Error> {
        let path = resource_path(&["v2", "gn", "split", "cobv", txid, "vinculo"])?;
        self.send_authenticated_status::<()>(Method::DELETE, &path, None)
            .await
    }
}
