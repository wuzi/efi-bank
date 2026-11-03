use reqwest::Method;

use crate::client::{Client, query_path, resource_path};
use crate::error::Error;
use crate::types::{CobvListResponse, CobvPayload, CobvResponse, PixChargeListQuery};

impl Client {
    /// Create a due-date charge with a caller-supplied txid.
    pub async fn cobv_create(
        &self,
        txid: &str,
        payload: &CobvPayload,
    ) -> Result<CobvResponse, Error> {
        let path = resource_path(&["v2", "cobv", txid])?;
        self.send_authenticated(Method::PUT, &path, Some(payload))
            .await
    }

    /// PUT creates with the caller txid; use PATCH for revision.
    pub async fn cobv_update(
        &self,
        txid: &str,
        payload: &CobvPayload,
    ) -> Result<CobvResponse, Error> {
        self.cobv_create(txid, payload).await
    }

    pub async fn cobv_patch(
        &self,
        txid: &str,
        payload: &CobvPayload,
    ) -> Result<CobvResponse, Error> {
        let path = resource_path(&["v2", "cobv", txid])?;
        self.send_authenticated(Method::PATCH, &path, Some(payload))
            .await
    }

    pub async fn cobv_get(&self, txid: &str) -> Result<CobvResponse, Error> {
        let path = resource_path(&["v2", "cobv", txid])?;
        self.send_authenticated::<(), CobvResponse>(Method::GET, &path, None)
            .await
    }

    /// Retrieve one page; the provider also names the due-date array `cobs`.
    pub async fn cobv_list(&self, query: &PixChargeListQuery) -> Result<CobvListResponse, Error> {
        let path = query_path("/v2/cobv", &query.pairs());
        self.send_authenticated::<(), CobvListResponse>(Method::GET, &path, None)
            .await
    }
}
