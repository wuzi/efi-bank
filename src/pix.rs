use reqwest::Method;

use crate::client::Client;
use crate::error::Error;
use crate::types::{PixTransactionDetailResponse, PixTransactionPayload, PixTransactionResponse};

impl Client {
    pub fn pix_send(
        &self,
        payload: &PixTransactionPayload,
    ) -> Result<PixTransactionResponse, Error> {
        self.send_authenticated(Method::POST, "/v2/pix/send", Some(payload))
    }

    pub fn pix_get_transaction(
        &self,
        end_to_end_id: &str,
    ) -> Result<PixTransactionDetailResponse, Error> {
        let path = format!("/v2/pix/transaction/{end_to_end_id}");
        self.send_authenticated::<serde_json::Value, PixTransactionDetailResponse>(
            Method::GET,
            &path,
            None,
        )
    }
}
