use reqwest::Method;

use crate::client::Client;
use crate::error::EfiError;
use crate::types::{
    BillingChargeCreateRequest, BillingChargeOneStepRequest, BillingChargePayRequest,
    BillingChargeResponse, BillingNotificationResponse,
};

impl Client {
    pub fn billing_charge_create(
        &self,
        payload: &BillingChargeCreateRequest,
    ) -> Result<BillingChargeResponse, EfiError> {
        self.send_authenticated_billing(Method::POST, "/v1/charge", Some(payload))
    }

    pub fn billing_charge_one_step(
        &self,
        payload: &BillingChargeOneStepRequest,
    ) -> Result<BillingChargeResponse, EfiError> {
        self.send_authenticated_billing(Method::POST, "/v1/charge/one-step", Some(payload))
    }

    pub fn billing_charge_pay(
        &self,
        charge_id: i64,
        payload: &BillingChargePayRequest,
    ) -> Result<BillingChargeResponse, EfiError> {
        let path = format!("/v1/charge/{charge_id}/pay");
        self.send_authenticated_billing(Method::POST, &path, Some(payload))
    }

    pub fn billing_notification_get(
        &self,
        token: &str,
    ) -> Result<BillingNotificationResponse, EfiError> {
        let path = format!("/v1/notification/{token}");
        self.send_authenticated_billing::<serde_json::Value, BillingNotificationResponse>(
            Method::GET,
            &path,
            None,
        )
    }
}
