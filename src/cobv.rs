use reqwest::Method;

use crate::client::Client;
use crate::error::Error;
use crate::types::{CobvPayload, CobvResponse};

impl Client {
    pub async fn cobv_create(&self, payload: &CobvPayload) -> Result<CobvResponse, Error> {
        self.send_authenticated(Method::POST, "/v2/cobv", Some(payload))
            .await
    }

    pub async fn cobv_update(
        &self,
        txid: &str,
        payload: &CobvPayload,
    ) -> Result<CobvResponse, Error> {
        let path = format!("/v2/cobv/{txid}");
        self.send_authenticated(Method::PUT, &path, Some(payload))
            .await
    }

    pub async fn cobv_patch(
        &self,
        txid: &str,
        payload: &CobvPayload,
    ) -> Result<CobvResponse, Error> {
        let path = format!("/v2/cobv/{txid}");
        self.send_authenticated(Method::PATCH, &path, Some(payload))
            .await
    }

    pub async fn cobv_get(&self, txid: &str) -> Result<CobvResponse, Error> {
        let path = format!("/v2/cobv/{txid}");
        self.send_authenticated::<serde_json::Value, CobvResponse>(Method::GET, &path, None)
            .await
    }

    pub async fn cobv_list(
        &self,
        cpf: Option<&str>,
        status: Option<&str>,
        limit: Option<i32>,
    ) -> Result<Vec<CobvResponse>, Error> {
        let mut path = String::from("/v2/cobv");
        let mut params = Vec::new();

        if let Some(c) = cpf {
            params.push(format!("cpf={c}"));
        }
        if let Some(s) = status {
            params.push(format!("status={s}"));
        }
        if let Some(l) = limit {
            params.push(format!("limit={l}"));
        }

        if !params.is_empty() {
            path.push('?');
            path.push_str(&params.join("&"));
        }

        self.send_authenticated::<serde_json::Value, Vec<CobvResponse>>(Method::GET, &path, None)
            .await
    }
}
