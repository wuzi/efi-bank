use reqwest::Method;

use crate::client::Client;
use crate::error::Error;
use crate::types::{SplitConfigResponse, SplitLinkResponse, SplitPayload};

impl Client {
    pub fn split_create_config(
        &self,
        payload: &SplitPayload,
    ) -> Result<SplitConfigResponse, Error> {
        self.send_authenticated(Method::POST, "/v2/gn/split/config", Some(payload))
    }

    pub fn split_update_config(
        &self,
        config_id: &str,
        payload: &SplitPayload,
    ) -> Result<SplitConfigResponse, Error> {
        let path = format!("/v2/gn/split/config/{config_id}");
        self.send_authenticated(Method::PUT, &path, Some(payload))
    }

    pub fn split_get_config(&self, config_id: &str) -> Result<SplitConfigResponse, Error> {
        let path = format!("/v2/gn/split/config/{config_id}");
        self.send_authenticated::<serde_json::Value, SplitConfigResponse>(Method::GET, &path, None)
    }

    pub fn split_link_cob(
        &self,
        txid: &str,
        split_config_id: &str,
    ) -> Result<SplitLinkResponse, Error> {
        let path = format!("/v2/gn/split/cob/{txid}/vinculo/{split_config_id}");
        self.send_authenticated::<serde_json::Value, SplitLinkResponse>(Method::PUT, &path, None)
    }

    pub fn split_get_cob(&self, txid: &str) -> Result<SplitConfigResponse, Error> {
        let path = format!("/v2/gn/split/cob/{txid}");
        self.send_authenticated::<serde_json::Value, SplitConfigResponse>(Method::GET, &path, None)
    }

    pub fn split_unlink_cob(&self, txid: &str) -> Result<SplitLinkResponse, Error> {
        let path = format!("/v2/gn/split/cob/{txid}/vinculo");
        self.send_authenticated::<serde_json::Value, SplitLinkResponse>(Method::DELETE, &path, None)
    }

    pub fn split_link_cobv(
        &self,
        txid: &str,
        split_config_id: &str,
    ) -> Result<SplitLinkResponse, Error> {
        let path = format!("/v2/gn/split/cobv/{txid}/vinculo/{split_config_id}");
        self.send_authenticated::<serde_json::Value, SplitLinkResponse>(Method::PUT, &path, None)
    }

    pub fn split_get_cobv(&self, txid: &str) -> Result<SplitConfigResponse, Error> {
        let path = format!("/v2/gn/split/cobv/{txid}");
        self.send_authenticated::<serde_json::Value, SplitConfigResponse>(Method::GET, &path, None)
    }

    pub fn split_unlink_cobv(&self, txid: &str) -> Result<SplitLinkResponse, Error> {
        let path = format!("/v2/gn/split/cobv/{txid}/vinculo");
        self.send_authenticated::<serde_json::Value, SplitLinkResponse>(Method::DELETE, &path, None)
    }
}
