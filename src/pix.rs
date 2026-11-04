use reqwest::Method;

use crate::client::{Client, resource_path};
use crate::error::Error;
use crate::types::{PixTransactionDetailResponse, PixTransactionPayload, PixTransactionResponse};

impl Client {
    /// Send using the caller's persisted operation identity. Never automatically replayed.
    /// HTTP acknowledgement/status strings do not establish settlement.
    pub async fn pix_send(
        &self,
        id_envio: &str,
        payload: &PixTransactionPayload,
    ) -> Result<PixTransactionResponse, Error> {
        let path = resource_path(&["v3", "gn", "pix", id_envio])?;
        let response: PixTransactionResponse = self
            .send_authenticated(Method::PUT, &path, Some(payload))
            .await?;
        if response.id_envio != id_envio {
            return Err(Error::Json(
                <serde_json::Error as serde::de::Error>::custom(
                    "Pix send response identity does not match operation",
                ),
            ));
        }
        Ok(response)
    }

    /// Read a sent Pix by its end-to-end identity; this is not received-Pix lookup.
    pub async fn pix_get_transaction(
        &self,
        end_to_end_id: &str,
    ) -> Result<PixTransactionDetailResponse, Error> {
        let path = resource_path(&["v2", "gn", "pix", "enviados", end_to_end_id])?;
        self.send_authenticated::<(), PixTransactionDetailResponse>(Method::GET, &path, None)
            .await
    }
}
