use std::{fs, path::PathBuf};

use reqwest::{Client as HttpClient, Identity, Method, StatusCode};
use serde::Serialize;
use serde::de::DeserializeOwned;

use crate::auth::TokenState;
use crate::environment::{Endpoints, Environment};
use crate::error::Error;

// Bound each native HTTP attempt. OAuth refresh is accounted for separately.
pub(crate) fn http_client_builder() -> reqwest::ClientBuilder {
    HttpClient::builder()
        .retry(reqwest::retry::never())
        .redirect(reqwest::redirect::Policy::none())
        .timeout(std::time::Duration::from_secs(20))
        .connect_timeout(std::time::Duration::from_secs(5))
}

pub struct Client {
    pub(crate) id: String,
    pub(crate) secret: String,
    pub(crate) environment: Environment,
    pub(crate) http: HttpClient,
    pub(crate) pix_token: TokenState,
    pub(crate) billing_token: TokenState,
    pub(crate) api_response_limit: usize,
    pub(crate) oauth_response_limit: usize,
    endpoints_override: Option<Endpoints>,
}

enum MtlsSource {
    Pkcs12Der { der: Vec<u8>, password: String },
    Pkcs12File { path: PathBuf, password: String },
}

pub struct ClientBuilder {
    id: Option<String>,
    secret: Option<String>,
    environment: Environment,
    http: Option<HttpClient>,
    mtls_source: Option<MtlsSource>,
    api_response_limit: usize,
    oauth_response_limit: usize,
}

impl Default for ClientBuilder {
    fn default() -> Self {
        Self {
            id: None,
            secret: None,
            environment: Environment::Sandbox,
            http: None,
            mtls_source: None,
            api_response_limit: 8 * 1024 * 1024,
            oauth_response_limit: 1024 * 1024,
        }
    }
}

impl ClientBuilder {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    #[must_use]
    pub fn client_id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    #[must_use]
    pub fn client_secret(mut self, value: impl Into<String>) -> Self {
        self.secret = Some(value.into());
        self
    }

    #[must_use]
    pub fn credentials(
        mut self,
        client_id: impl Into<String>,
        client_secret: impl Into<String>,
    ) -> Self {
        self.id = Some(client_id.into());
        self.secret = Some(client_secret.into());
        self
    }

    #[must_use]
    pub const fn environment(mut self, environment: Environment) -> Self {
        self.environment = environment;
        self
    }

    /// Supply a caller-owned transport, overriding the SDK defaults.
    ///
    /// The caller must disable retries and redirects and set finite request/connect
    /// timeouts to retain bounded native request accounting.
    #[must_use]
    pub fn http_client(mut self, http_client: HttpClient) -> Self {
        self.http = Some(http_client);
        self
    }

    #[must_use]
    pub fn pkcs12_der(mut self, der: impl Into<Vec<u8>>, password: impl Into<String>) -> Self {
        self.mtls_source = Some(MtlsSource::Pkcs12Der {
            der: der.into(),
            password: password.into(),
        });
        self
    }

    #[must_use]
    pub fn pkcs12_file(mut self, path: impl Into<PathBuf>, password: impl Into<String>) -> Self {
        self.mtls_source = Some(MtlsSource::Pkcs12File {
            path: path.into(),
            password: password.into(),
        });
        self
    }

    /// Set the API JSON/status/error body limit in bytes (default 8 MiB).
    /// This engineering limit is not a provider maximum. Zero is rejected.
    #[must_use]
    pub const fn api_response_limit(mut self, bytes: usize) -> Self {
        self.api_response_limit = bytes;
        self
    }

    /// Set the OAuth success/error body limit in bytes (default 1 MiB).
    /// Zero is rejected; unusually large responses can opt into a larger bound.
    #[must_use]
    pub const fn oauth_response_limit(mut self, bytes: usize) -> Self {
        self.oauth_response_limit = bytes;
        self
    }

    pub fn build(self) -> Result<Client, Error> {
        if self.api_response_limit == 0 || self.oauth_response_limit == 0 {
            return Err(Error::BuilderConflict("response limits must be nonzero"));
        }
        let client_id = self.id.ok_or(Error::BuilderMissingField("client_id"))?;
        let client_secret = self
            .secret
            .ok_or(Error::BuilderMissingField("client_secret"))?;

        let http_client = if let Some(http_client) = self.http {
            if self.mtls_source.is_some() {
                return Err(Error::BuilderConflict(
                    "cannot set both http_client and pkcs12_* options",
                ));
            }
            http_client
        } else if let Some(mtls_source) = self.mtls_source {
            match mtls_source {
                MtlsSource::Pkcs12Der { der, password } => {
                    let identity = Identity::from_pkcs12_der(&der, &password)?;
                    http_client_builder().identity(identity).build()?
                }
                MtlsSource::Pkcs12File { path, password } => {
                    let der = fs::read(path)?;
                    let identity = Identity::from_pkcs12_der(&der, &password)?;
                    http_client_builder().identity(identity).build()?
                }
            }
        } else {
            http_client_builder().build()?
        };

        let mut client =
            Client::from_parts(client_id, client_secret, self.environment, http_client);
        client.api_response_limit = self.api_response_limit;
        client.oauth_response_limit = self.oauth_response_limit;
        Ok(client)
    }
}

impl Client {
    fn from_parts(
        client_id: String,
        client_secret: String,
        environment: Environment,
        http_client: HttpClient,
    ) -> Self {
        Self {
            id: client_id,
            secret: client_secret,
            environment,
            http: http_client,
            pix_token: TokenState::default(),
            billing_token: TokenState::default(),
            api_response_limit: 8 * 1024 * 1024,
            oauth_response_limit: 1024 * 1024,
            endpoints_override: None,
        }
    }

    #[must_use]
    pub const fn endpoints(&self) -> Endpoints {
        match self.endpoints_override {
            Some(endpoints) => endpoints,
            None => self.environment.endpoints(),
        }
    }

    #[cfg(test)]
    pub(crate) fn test_billing_client(base_url: &str) -> Self {
        Self::test_client_from_builder(
            base_url,
            ClientBuilder::new().credentials("fixture-client", "fixture-secret"),
        )
    }

    #[cfg(test)]
    pub(crate) fn test_client_from_builder(base_url: &str, builder: ClientBuilder) -> Self {
        let base_url = Box::leak(base_url.to_owned().into_boxed_str());
        let authorize_url = Box::leak(format!("{base_url}/v1/authorize").into_boxed_str());
        let mut client = builder.build().unwrap();
        client.endpoints_override = Some(Endpoints {
            pix_api_base_url: base_url,
            pix_api_oauth_token_url: authorize_url,
            billing_api_base_url: base_url,
            billing_api_oauth_token_url: authorize_url,
        });
        client
    }

    #[cfg(test)]
    pub(crate) fn test_distinct_oauth(&mut self) {
        let url = Box::leak(
            format!("{}/oauth/token", self.endpoints().pix_api_base_url).into_boxed_str(),
        );
        self.endpoints_override
            .as_mut()
            .unwrap()
            .pix_api_oauth_token_url = url;
    }

    pub(crate) fn token_state(&self, domain: Domain) -> &TokenState {
        match domain {
            Domain::Pix => &self.pix_token,
            Domain::Billing => &self.billing_token,
        }
    }

    pub(crate) async fn send_authenticated<Req, Res>(
        &self,
        method: Method,
        path: &str,
        payload: Option<&Req>,
    ) -> Result<Res, Error>
    where
        Req: Serialize + Sync,
        Res: DeserializeOwned,
    {
        let response = self
            .authenticated_attempt(Domain::Pix, method, path, payload)
            .await?;
        self.parse_json(response).await
    }

    pub(crate) async fn send_authenticated_billing<Req, Res>(
        &self,
        method: Method,
        path: &str,
        payload: Option<&Req>,
    ) -> Result<Res, Error>
    where
        Req: Serialize + Sync,
        Res: DeserializeOwned,
    {
        let response = self
            .authenticated_attempt(Domain::Billing, method, path, payload)
            .await?;
        self.parse_json(response).await
    }

    pub(crate) async fn send_authenticated_status<Req>(
        &self,
        method: Method,
        path: &str,
        payload: Option<&Req>,
    ) -> Result<(), Error>
    where
        Req: Serialize + Sync,
    {
        let response = self
            .authenticated_attempt(Domain::Pix, method, path, payload)
            .await?;
        crate::response::read_response(response, self.api_response_limit).await?;
        Ok(())
    }

    async fn authenticated_attempt<Req>(
        &self,
        domain: Domain,
        method: Method,
        path: &str,
        payload: Option<&Req>,
    ) -> Result<reqwest::Response, Error>
    where
        Req: Serialize + Sync,
    {
        let mut token = self.access_token(domain).await?;
        // Only GET may replay once. Every other method returns its first native result.
        for attempt in 0..=1 {
            let mut request = self
                .http
                .request(
                    method.clone(),
                    format!("{}{path}", domain.base_url(self.endpoints())),
                )
                .bearer_auth(&token);
            if let Some(payload) = payload {
                request = request.json(payload);
            }
            let response = request.send().await?;
            if response.status() != StatusCode::UNAUTHORIZED {
                return Ok(response);
            }
            self.token_state(domain).clear_rejected(&token)?;
            if method != Method::GET || attempt == 1 {
                return Ok(response);
            }
            drop(response);
            // A delayed rejection of A cannot erase a newer B. The gate rechecks B.
            token = self.access_token(domain).await?;
        }
        unreachable!("the second response always returns")
    }

    async fn parse_json<Res: DeserializeOwned>(
        &self,
        response: reqwest::Response,
    ) -> Result<Res, Error> {
        let bytes = crate::response::read_response(response, self.api_response_limit).await?;
        if bytes.iter().all(u8::is_ascii_whitespace) {
            return Err(Error::EmptyResponse);
        }
        Ok(serde_json::from_slice(&bytes)?)
    }
}

#[derive(Clone, Copy)]
pub(crate) enum Domain {
    Pix,
    Billing,
}

impl Domain {
    pub(crate) const fn token_url(self, endpoints: Endpoints) -> &'static str {
        match self {
            Self::Pix => endpoints.pix_api_oauth_token_url,
            Self::Billing => endpoints.billing_api_oauth_token_url,
        }
    }
    const fn base_url(self, endpoints: Endpoints) -> &'static str {
        match self {
            Self::Pix => endpoints.pix_api_base_url,
            Self::Billing => endpoints.billing_api_base_url,
        }
    }
}

/// Encode each supplied identity exactly once; no decoding or restrictive provider alphabet.
/// Literal dot segments are rejected before authentication and cannot be revived by URL parsing.
pub(crate) fn resource_path(segments: &[&str]) -> Result<String, Error> {
    use std::fmt::Write;
    let mut path = String::new();
    for segment in segments {
        if matches!(*segment, "" | "." | "..") {
            return Err(Error::InvalidPath);
        }
        path.push('/');
        for byte in segment.bytes() {
            if byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.' | b'~') {
                path.push(char::from(byte));
            } else {
                write!(&mut path, "%{byte:02X}").expect("formatting into a String cannot fail");
            }
        }
    }
    Ok(path)
}

/// Encode query pairs on SDK-owned static paths; identifiers use resource_path instead.
pub(crate) fn query_path(path: &str, pairs: &[(&str, &str)]) -> String {
    let mut url = reqwest::Url::parse(&format!("https://unused.invalid{path}"))
        .expect("SDK-owned static query path is valid");
    url.query_pairs_mut().extend_pairs(pairs.iter().copied());
    format!(
        "{}?{}",
        url.path(),
        url.query().expect("query pairs supplied")
    )
}
