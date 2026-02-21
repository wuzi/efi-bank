use reqwest::Method;

use crate::client::Client;
use crate::error::Error;
use crate::types::{
    BillingChargeCreateRequest, BillingChargeOneStepRequest, BillingChargePayRequest,
    BillingChargeResponse, BillingNotificationResponse,
};

impl Client {
    pub async fn billing_charge_create(
        &self,
        payload: &BillingChargeCreateRequest,
    ) -> Result<BillingChargeResponse, Error> {
        self.send_authenticated_billing(Method::POST, "/v1/charge", Some(payload))
            .await
    }

    pub async fn billing_charge_one_step(
        &self,
        payload: &BillingChargeOneStepRequest,
    ) -> Result<BillingChargeResponse, Error> {
        self.send_authenticated_billing(Method::POST, "/v1/charge/one-step", Some(payload))
            .await
    }

    pub async fn billing_charge_pay(
        &self,
        charge_id: i64,
        payload: &BillingChargePayRequest,
    ) -> Result<BillingChargeResponse, Error> {
        let path = format!("/v1/charge/{charge_id}/pay");
        self.send_authenticated_billing(Method::POST, &path, Some(payload))
            .await
    }

    pub async fn billing_notification_get(
        &self,
        token: &str,
    ) -> Result<BillingNotificationResponse, Error> {
        let path = format!("/v1/notification/{token}");
        self.send_authenticated_billing::<serde_json::Value, BillingNotificationResponse>(
            Method::GET,
            &path,
            None,
        )
        .await
    }
}
