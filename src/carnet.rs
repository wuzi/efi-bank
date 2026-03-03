use reqwest::Method;

use crate::client::Client;
use crate::error::Error;
use crate::types::{
    CarnetCreateRequest, CarnetHistoryRequest, CarnetMetadataRequest, CarnetParcelRequest,
    CarnetParcelsRequest, CarnetResponse,
};

impl Client {
    pub async fn carnet_create(
        &self,
        payload: &CarnetCreateRequest,
    ) -> Result<CarnetResponse, Error> {
        self.send_authenticated_billing(Method::POST, "/v1/carnet", Some(payload))
            .await
    }

    pub async fn carnet_get(&self, carnet_id: i64) -> Result<CarnetResponse, Error> {
        let path = format!("/v1/carnet/{carnet_id}");
        self.send_authenticated_billing::<serde_json::Value, CarnetResponse>(
            Method::GET,
            &path,
            None,
        )
        .await
    }

    pub async fn carnet_list(
        &self,
        begin_date: &str,
        end_date: &str,
    ) -> Result<CarnetResponse, Error> {
        let path =
            format!("/v1/charges?begin_date={begin_date}&end_date={end_date}&charge_type=carnet");
        self.send_authenticated_billing::<serde_json::Value, CarnetResponse>(
            Method::GET,
            &path,
            None,
        )
        .await
    }

    pub async fn carnet_update_metadata(
        &self,
        carnet_id: i64,
        payload: &CarnetMetadataRequest,
    ) -> Result<CarnetResponse, Error> {
        let path = format!("/v1/carnet/{carnet_id}/metadata");
        self.send_authenticated_billing(Method::PUT, &path, Some(payload))
            .await
    }

    pub async fn carnet_update_parcel(
        &self,
        carnet_id: i64,
        parcel: i32,
        payload: &CarnetParcelRequest,
    ) -> Result<CarnetResponse, Error> {
        let path = format!("/v1/carnet/{carnet_id}/parcel/{parcel}");
        self.send_authenticated_billing(Method::PUT, &path, Some(payload))
            .await
    }

    pub async fn carnet_update_parcels(
        &self,
        carnet_id: i64,
        payload: &CarnetParcelsRequest,
    ) -> Result<CarnetResponse, Error> {
        let path = format!("/v1/carnet/{carnet_id}/parcels");
        self.send_authenticated_billing(Method::PUT, &path, Some(payload))
            .await
    }

    pub async fn carnet_cancel(&self, carnet_id: i64) -> Result<CarnetResponse, Error> {
        let path = format!("/v1/carnet/{carnet_id}/cancel");
        self.send_authenticated_billing::<serde_json::Value, CarnetResponse>(
            Method::PUT,
            &path,
            None,
        )
        .await
    }

    pub async fn carnet_cancel_parcel(
        &self,
        carnet_id: i64,
        parcel: i32,
    ) -> Result<CarnetResponse, Error> {
        let path = format!("/v1/carnet/{carnet_id}/parcel/{parcel}/cancel");
        self.send_authenticated_billing::<serde_json::Value, CarnetResponse>(
            Method::PUT,
            &path,
            None,
        )
        .await
    }

    pub async fn carnet_resend(&self, carnet_id: i64) -> Result<CarnetResponse, Error> {
        let path = format!("/v1/carnet/{carnet_id}/resend");
        self.send_authenticated_billing::<serde_json::Value, CarnetResponse>(
            Method::POST,
            &path,
            None,
        )
        .await
    }

    pub async fn carnet_resend_parcel(
        &self,
        carnet_id: i64,
        parcel: i32,
    ) -> Result<CarnetResponse, Error> {
        let path = format!("/v1/carnet/{carnet_id}/parcel/{parcel}/resend");
        self.send_authenticated_billing::<serde_json::Value, CarnetResponse>(
            Method::POST,
            &path,
            None,
        )
        .await
    }

    pub async fn carnet_add_history(
        &self,
        carnet_id: i64,
        payload: &CarnetHistoryRequest,
    ) -> Result<CarnetResponse, Error> {
        let path = format!("/v1/carnet/{carnet_id}/history");
        self.send_authenticated_billing(Method::POST, &path, Some(payload))
            .await
    }

    pub async fn carnet_settle(&self, carnet_id: i64) -> Result<CarnetResponse, Error> {
        let path = format!("/v1/carnet/{carnet_id}/settle");
        self.send_authenticated_billing::<serde_json::Value, CarnetResponse>(
            Method::PUT,
            &path,
            None,
        )
        .await
    }

    pub async fn carnet_settle_parcel(
        &self,
        carnet_id: i64,
        parcel: i32,
    ) -> Result<CarnetResponse, Error> {
        let path = format!("/v1/carnet/{carnet_id}/parcel/{parcel}/settle");
        self.send_authenticated_billing::<serde_json::Value, CarnetResponse>(
            Method::PUT,
            &path,
            None,
        )
        .await
    }
}
