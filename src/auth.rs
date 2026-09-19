use std::time::{Duration, Instant};

use serde::Deserialize;
use serde_json::json;

use crate::client::Client;
use crate::error::Error;

const TOKEN_REFRESH_SKEW_SECS: u64 = 30;

#[derive(Debug)]
pub(crate) struct AccessToken {
    pub(crate) value: String,
    pub(crate) expires_at: Instant,
}

impl AccessToken {
    pub(crate) fn is_expired(&self) -> bool {
        let refresh_skew = Duration::from_secs(TOKEN_REFRESH_SKEW_SECS);
        Instant::now() + refresh_skew >= self.expires_at
    }
}

#[derive(Debug, Deserialize)]
struct OAuthResponse {
    access_token: String,
    expires_in: u64,
}

impl Client {
    pub async fn authenticate(&self) -> Result<(), Error> {
        self.refresh_access_token().await.map(|_| ())
    }

    pub async fn authenticate_billing(&self) -> Result<(), Error> {
        self.refresh_billing_access_token().await.map(|_| ())
    }

    pub(crate) async fn refresh_access_token(&self) -> Result<String, Error> {
        self.authenticate_with_url(self.endpoints().pix_api_oauth_token_url)
            .await
    }

    pub(crate) async fn refresh_billing_access_token(&self) -> Result<String, Error> {
        self.authenticate_with_url(self.endpoints().billing_api_oauth_token_url)
            .await
    }

    pub(crate) async fn get_valid_access_token(&self) -> Result<String, Error> {
        let endpoints = self.endpoints();
        self.get_valid_access_token_with_url(endpoints.pix_api_oauth_token_url)
            .await
    }

    pub(crate) async fn get_valid_billing_access_token(&self) -> Result<String, Error> {
        let endpoints = self.endpoints();
        self.get_valid_access_token_with_url(endpoints.billing_api_oauth_token_url)
            .await
    }

    async fn authenticate_with_url(&self, token_url: &str) -> Result<String, Error> {
        let response = self
            .http
            .post(token_url)
            .basic_auth(&self.id, Some(&self.secret))
            .json(&json!({ "grant_type": "client_credentials" }))
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let retry_after = response
                .headers()
                .get("Retry-After")
                .and_then(|value| value.to_str().ok())
                .map(ToOwned::to_owned);
            let body = response.text().await.unwrap_or_else(|_| String::new());
            return Err(Error::RequestFailed {
                status,
                body,
                retry_after,
            });
        }

        let oauth = response.json::<OAuthResponse>().await?;
        if oauth.access_token.trim().is_empty() || oauth.expires_in <= TOKEN_REFRESH_SKEW_SECS {
            return Err(Error::AuthUnavailable);
        }
        let expires_at = Instant::now()
            .checked_add(Duration::from_secs(oauth.expires_in))
            .ok_or(Error::AuthUnavailable)?;

        self.token
            .lock()
            .map_err(|_| Error::AuthUnavailable)?
            .replace(AccessToken {
                value: oauth.access_token.clone(),
                expires_at,
            });

        Ok(oauth.access_token)
    }

    async fn get_valid_access_token_with_url(&self, token_url: &str) -> Result<String, Error> {
        let needs_authentication = {
            let token = self.token.lock().map_err(|_| Error::AuthUnavailable)?;
            token.as_ref().is_none_or(AccessToken::is_expired)
        };

        if needs_authentication {
            return self.authenticate_with_url(token_url).await;
        }

        let token = self.token.lock().map_err(|_| Error::AuthUnavailable)?;
        token
            .as_ref()
            .map(|cached| cached.value.clone())
            .ok_or(Error::AuthUnavailable)
    }
}
