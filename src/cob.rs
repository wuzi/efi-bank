use reqwest::Method;

use crate::client::{Client, query_path, resource_path};
use crate::error::Error;
use crate::types::{CobListResponse, CobPayload, CobResponse, PixChargeListQuery};

impl Client {
    pub async fn cob_create(&self, payload: &CobPayload) -> Result<CobResponse, Error> {
        self.send_authenticated(Method::POST, "/v2/cob", Some(payload))
            .await
    }

    /// Create an immediate charge with a caller-supplied txid (PUT semantics).
    pub async fn cob_update(&self, txid: &str, payload: &CobPayload) -> Result<CobResponse, Error> {
        let path = resource_path(&["v2", "cob", txid])?;
        self.send_authenticated(Method::PUT, &path, Some(payload))
            .await
    }

    pub async fn cob_patch(&self, txid: &str, payload: &CobPayload) -> Result<CobResponse, Error> {
        let path = resource_path(&["v2", "cob", txid])?;
        self.send_authenticated(Method::PATCH, &path, Some(payload))
            .await
    }

    pub async fn cob_get(&self, txid: &str) -> Result<CobResponse, Error> {
        let path = resource_path(&["v2", "cob", txid])?;
        self.send_authenticated::<(), CobResponse>(Method::GET, &path, None)
            .await
    }

    /// Retrieve one page for the required interval; metadata is retained.
    pub async fn cob_list(&self, query: &PixChargeListQuery) -> Result<CobListResponse, Error> {
        let path = query_path("/v2/cob", &query.pairs());
        self.send_authenticated::<(), CobListResponse>(Method::GET, &path, None)
            .await
    }
}
