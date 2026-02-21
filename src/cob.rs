use reqwest::Method;

use crate::client::Client;
use crate::error::Error;
use crate::types::{CobPayload, CobResponse};

impl Client {
    pub fn cob_create(&self, payload: &CobPayload) -> Result<CobResponse, Error> {
        self.send_authenticated(Method::POST, "/v2/cob", Some(payload))
    }

    pub fn cob_update(&self, txid: &str, payload: &CobPayload) -> Result<CobResponse, Error> {
        let path = format!("/v2/cob/{txid}");
        self.send_authenticated(Method::PUT, &path, Some(payload))
    }

    pub fn cob_patch(&self, txid: &str, payload: &CobPayload) -> Result<CobResponse, Error> {
        let path = format!("/v2/cob/{txid}");
        self.send_authenticated(Method::PATCH, &path, Some(payload))
    }

    pub fn cob_get(&self, txid: &str) -> Result<CobResponse, Error> {
        let path = format!("/v2/cob/{txid}");
        self.send_authenticated::<serde_json::Value, CobResponse>(Method::GET, &path, None)
    }

    pub fn cob_list(
        &self,
        cpf: Option<&str>,
        status: Option<&str>,
        limit: Option<i32>,
    ) -> Result<Vec<CobResponse>, Error> {
        let mut path = String::from("/v2/cob");
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

        self.send_authenticated::<serde_json::Value, Vec<CobResponse>>(Method::GET, &path, None)
    }
}
