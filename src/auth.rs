use std::sync::Mutex;
use std::time::{Duration, Instant};

use serde::Deserialize;
use serde_json::json;

use crate::client::{Client, Domain};
use crate::error::Error;

const TOKEN_REFRESH_SKEW_SECS: u64 = 30;

#[derive(Debug)]
pub(crate) struct AccessToken {
    pub(crate) value: String,
    pub(crate) expires_at: Instant,
}

impl AccessToken {
    pub(crate) fn is_expired(&self) -> bool {
        Instant::now()
            .checked_add(Duration::from_secs(TOKEN_REFRESH_SKEW_SECS))
            .is_none_or(|threshold| threshold >= self.expires_at)
    }
}

#[derive(Default)]
pub(crate) struct TokenState {
    pub(crate) cached: Mutex<Option<AccessToken>>,
    refresh: tokio::sync::Mutex<()>,
}

impl TokenState {
    fn valid(&self) -> Result<Option<String>, Error> {
        Ok(self
            .cached
            .lock()
            .map_err(|_| Error::AuthUnavailable)?
            .as_ref()
            .filter(|token| !token.is_expired())
            .map(|token| token.value.clone()))
    }

    pub(crate) fn clear_rejected(&self, rejected: &str) -> Result<(), Error> {
        let mut cached = self.cached.lock().map_err(|_| Error::AuthUnavailable)?;
        if cached.as_ref().is_some_and(|token| token.value == rejected) {
            cached.take();
        }
        Ok(())
    }
}

#[derive(Debug, Deserialize)]
struct OAuthResponse {
    access_token: String,
    expires_in: u64,
}

impl Client {
    /// Explicitly exchange a Pix token, even when the cached token is fresh.
    pub async fn authenticate(&self) -> Result<(), Error> {
        self.acquire_token(Domain::Pix, true).await.map(|_| ())
    }

    /// Explicitly exchange a Billing token, even when the cached token is fresh.
    pub async fn authenticate_billing(&self) -> Result<(), Error> {
        self.acquire_token(Domain::Billing, true).await.map(|_| ())
    }

    pub(crate) async fn access_token(&self, domain: Domain) -> Result<String, Error> {
        self.acquire_token(domain, false).await
    }

    async fn acquire_token(&self, domain: Domain, force_refresh: bool) -> Result<String, Error> {
        let state = self.token_state(domain);
        let reusable = if force_refresh { None } else { state.valid()? };
        if let Some(token) = reusable {
            return Ok(token);
        }
        // This gate is cancellation safe and never blocks the other authorization domain.
        // A failed owner's waiters must not accumulate unbounded serial exchanges.
        // This limits queue time only; the OAuth attempt retains its HTTP deadline.
        let _refresh = tokio::time::timeout(Duration::from_secs(20), state.refresh.lock())
            .await
            .map_err(|_| Error::AuthUnavailable)?;
        let reusable = if force_refresh { None } else { state.valid()? };
        if let Some(token) = reusable {
            return Ok(token);
        }
        let response = self
            .http
            .post(domain.token_url(self.endpoints()))
            .basic_auth(&self.id, Some(&self.secret))
            .json(&json!({ "grant_type": "client_credentials" }))
            .send()
            .await?;
        let body = crate::response::read_response(response, self.oauth_response_limit).await?;
        if body.iter().all(u8::is_ascii_whitespace) {
            return Err(Error::EmptyResponse);
        }
        let oauth: OAuthResponse = serde_json::from_slice(&body)?;
        if oauth.access_token.trim().is_empty() || oauth.expires_in <= TOKEN_REFRESH_SKEW_SECS {
            return Err(Error::AuthUnavailable);
        }
        let expires_at = Instant::now()
            .checked_add(Duration::from_secs(oauth.expires_in))
            .ok_or(Error::AuthUnavailable)?;
        state
            .cached
            .lock()
            .map_err(|_| Error::AuthUnavailable)?
            .replace(AccessToken {
                value: oauth.access_token.clone(),
                expires_at,
            });
        Ok(oauth.access_token)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test(start_paused = true)]
    async fn held_refresh_gate_has_finite_pre_dispatch_wait() {
        let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        listener.set_nonblocking(true).unwrap();
        let client =
            Client::test_billing_client(&format!("http://{}", listener.local_addr().unwrap()));
        let _held = client.pix_token.refresh.lock().await;
        let result = tokio::time::timeout(Duration::from_secs(21), client.authenticate()).await;
        assert!(matches!(result, Ok(Err(Error::AuthUnavailable))));
        assert!(
            matches!(listener.accept(), Err(error) if error.kind() == std::io::ErrorKind::WouldBlock)
        );
    }
}
