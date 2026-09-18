use reqwest::Method;

use crate::client::Client;
use crate::error::Error;
use crate::types::{
    BillingActionResponse, BillingChargeCreateRequest, BillingChargeDetailResponse,
    BillingChargeListQuery, BillingChargeListResponse, BillingChargeOneStepRequest,
    BillingChargePayRequest, BillingChargeReadResponse, BillingChargeResponse,
    BillingNotificationResponse,
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

    pub async fn billing_charge_get(
        &self,
        charge_id: i64,
    ) -> Result<BillingChargeDetailResponse, Error> {
        let path = format!("/v1/charge/{charge_id}");
        self.send_authenticated_billing::<serde_json::Value, BillingChargeDetailResponse>(
            Method::GET,
            &path,
            None,
        )
        .await
    }

    pub async fn billing_charge_read(
        &self,
        charge_id: i64,
    ) -> Result<BillingChargeReadResponse, Error> {
        let path = format!("/v1/charge/{charge_id}");
        self.send_authenticated_billing::<serde_json::Value, BillingChargeReadResponse>(
            Method::GET,
            &path,
            None,
        )
        .await
    }

    pub async fn billing_charges_list(
        &self,
        query: &BillingChargeListQuery,
    ) -> Result<BillingChargeListResponse, Error> {
        let mut url = reqwest::Url::parse("https://unused.invalid/v1/charges")
            .expect("the static lifecycle charges URL is valid");
        {
            let mut pairs = url.query_pairs_mut();
            pairs
                .append_pair("charge_type", &query.charge_type)
                .append_pair("begin_date", &query.begin_date)
                .append_pair("end_date", &query.end_date);
            if let Some(date_of) = query.date_of {
                let date_of = match date_of {
                    crate::types::BillingChargeDateOf::Creation => "creation",
                    crate::types::BillingChargeDateOf::Payment => "payment",
                    crate::types::BillingChargeDateOf::Expired => "expired",
                };
                pairs.append_pair("date_of", date_of);
            }
            if let Some(custom_id) = &query.custom_id {
                pairs.append_pair("custom_id", custom_id);
            }
            if let Some(limit) = query.limit {
                pairs.append_pair("limit", &limit.to_string());
            }
            if let Some(page) = query.page {
                pairs.append_pair("page", &page.to_string());
            }
            if let Some(offset) = query.offset {
                pairs.append_pair("offset", &offset.to_string());
            }
        }
        let path = format!(
            "{}?{}",
            url.path(),
            url.query().expect("lifecycle charges query is not empty")
        );
        self.send_authenticated_billing::<serde_json::Value, BillingChargeListResponse>(
            Method::GET,
            &path,
            None,
        )
        .await
    }

    pub async fn billing_charge_cancel(&self, charge_id: i64) -> Result<(), Error> {
        let path = format!("/v1/charge/{charge_id}/cancel");
        self.send_authenticated_billing::<serde_json::Value, serde_json::Value>(
            Method::PUT,
            &path,
            None,
        )
        .await?;
        Ok(())
    }

    pub async fn billing_charge_cancel_response(
        &self,
        charge_id: i64,
    ) -> Result<BillingActionResponse, Error> {
        let path = format!("/v1/charge/{charge_id}/cancel");
        self.send_authenticated_billing::<serde_json::Value, BillingActionResponse>(
            Method::PUT,
            &path,
            None,
        )
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
