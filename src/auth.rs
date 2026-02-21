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
        let endpoints = self.endpoints();
        self.authenticate_with_url(endpoints.pix_api_oauth_token_url)
            .await
    }

    pub async fn authenticate_billing(&self) -> Result<(), Error> {
        let endpoints = self.endpoints();
        self.authenticate_with_url(endpoints.billing_api_oauth_token_url)
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

    async fn authenticate_with_url(&self, token_url: &str) -> Result<(), Error> {
        let response = self
            .http
            .post(token_url)
            .basic_auth(&self.id, Some(&self.secret))
            .json(&json!({ "grant_type": "client_credentials" }))
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_else(|_| String::new());
            return Err(Error::RequestFailed { status, body });
        }

        let oauth = response.json::<OAuthResponse>().await?;
        let expires_at = Instant::now() + Duration::from_secs(oauth.expires_in);

        self.token
            .lock()
            .map_err(|_| Error::AuthUnavailable)?
            .replace(AccessToken {
                value: oauth.access_token,
                expires_at,
            });

        Ok(())
    }

    async fn get_valid_access_token_with_url(&self, token_url: &str) -> Result<String, Error> {
        let needs_authentication = {
            let token = self.token.lock().map_err(|_| Error::AuthUnavailable)?;
            token.as_ref().is_none_or(AccessToken::is_expired)
        };

        if needs_authentication {
            self.authenticate_with_url(token_url).await?;
        }

        let token = self.token.lock().map_err(|_| Error::AuthUnavailable)?;
        token
            .as_ref()
            .map(|cached| cached.value.clone())
            .ok_or(Error::AuthUnavailable)
    }
}
