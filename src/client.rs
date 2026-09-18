use std::sync::Mutex;
use std::{fs, path::PathBuf};

use reqwest::{Client as HttpClient, Identity, Method, StatusCode};
use serde::Serialize;
use serde::de::DeserializeOwned;

use crate::auth::AccessToken;
use crate::environment::{Endpoints, Environment};
use crate::error::Error;

pub struct Client {
    pub(crate) id: String,
    pub(crate) secret: String,
    pub(crate) environment: Environment,
    pub(crate) http: HttpClient,
    pub(crate) token: Mutex<Option<AccessToken>>,
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
}

impl Default for ClientBuilder {
    fn default() -> Self {
        Self {
            id: None,
            secret: None,
            environment: Environment::Sandbox,
            http: None,
            mtls_source: None,
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

    pub fn build(self) -> Result<Client, Error> {
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
                    HttpClient::builder().identity(identity).build()?
                }
                MtlsSource::Pkcs12File { path, password } => {
                    let der = fs::read(path)?;
                    let identity = Identity::from_pkcs12_der(&der, &password)?;
                    HttpClient::builder().identity(identity).build()?
                }
            }
        } else {
            HttpClient::new()
        };

        Ok(Client::from_parts(
            client_id,
            client_secret,
            self.environment,
            http_client,
        ))
    }
}

impl Client {
    const fn from_parts(
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
            token: Mutex::new(None),
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
        let base_url = Box::leak(base_url.to_owned().into_boxed_str());
        let authorize_url = Box::leak(format!("{base_url}/v1/authorize").into_boxed_str());
        let mut client = Self::from_parts(
            "fixture-client".into(),
            "fixture-secret".into(),
            Environment::Sandbox,
            HttpClient::new(),
        );
        client.endpoints_override = Some(Endpoints {
            pix_api_base_url: base_url,
            pix_api_oauth_token_url: authorize_url,
            billing_api_base_url: base_url,
            billing_api_oauth_token_url: authorize_url,
        });
        client
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
        let token = self.get_valid_access_token().await?;
        let first_response = self
            .send_with_token_typed::<Req>(&token, method.clone(), path, payload)
            .await?;

        if first_response.status() == StatusCode::UNAUTHORIZED {
            let refreshed_token = self.refresh_access_token().await?;
            let retry_response = self
                .send_with_token_typed::<Req>(&refreshed_token, method, path, payload)
                .await?;
            return Self::parse_response::<Res>(retry_response).await;
        }

        Self::parse_response::<Res>(first_response).await
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
        let token = self.get_valid_billing_access_token().await?;
        let first_response = self
            .send_with_token_typed_base::<Req>(
                &token,
                self.endpoints().billing_api_base_url,
                method.clone(),
                path,
                payload,
            )
            .await?;

        if first_response.status() == StatusCode::UNAUTHORIZED {
            let refreshed_token = self.refresh_billing_access_token().await?;
            let retry_response = self
                .send_with_token_typed_base::<Req>(
                    &refreshed_token,
                    self.endpoints().billing_api_base_url,
                    method,
                    path,
                    payload,
                )
                .await?;
            return Self::parse_response::<Res>(retry_response).await;
        }

        Self::parse_response::<Res>(first_response).await
    }

    async fn send_with_token_typed<Req>(
        &self,
        access_token: &str,
        method: Method,
        path: &str,
        payload: Option<&Req>,
    ) -> Result<reqwest::Response, Error>
    where
        Req: Serialize + Sync,
    {
        self.send_with_token_typed_base(
            access_token,
            self.endpoints().pix_api_base_url,
            method,
            path,
            payload,
        )
        .await
    }

    async fn send_with_token_typed_base<Req>(
        &self,
        access_token: &str,
        base_url: &str,
        method: Method,
        path: &str,
        payload: Option<&Req>,
    ) -> Result<reqwest::Response, Error>
    where
        Req: Serialize + Sync,
    {
        let url = format!("{base_url}{path}");

        let mut request = self.http.request(method, url).bearer_auth(access_token);

        if let Some(json_payload) = payload {
            request = request.json(json_payload);
        }

        Ok(request.send().await?)
    }

    async fn parse_response<Res>(response: reqwest::Response) -> Result<Res, Error>
    where
        Res: DeserializeOwned,
    {
        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_else(|_| String::new());
            return Err(Error::RequestFailed { status, body });
        }

        let body = response.text().await?;
        if body.trim().is_empty() {
            return Err(Error::EmptyResponse);
        }

        Ok(serde_json::from_str(&body)?)
    }
}
