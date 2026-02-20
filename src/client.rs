use std::sync::Mutex;
use std::{fs, path::PathBuf};

use reqwest::blocking::Client as HttpClient;
use reqwest::{Identity, Method, StatusCode};
use serde::Serialize;
use serde::de::DeserializeOwned;

use crate::auth::AccessToken;
use crate::environment::{Endpoints, Environment};
use crate::error::EfiError;

pub struct Client {
    pub(crate) id: String,
    pub(crate) secret: String,
    pub(crate) environment: Environment,
    pub(crate) http: HttpClient,
    pub(crate) token: Mutex<Option<AccessToken>>,
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

    pub fn build(self) -> Result<Client, EfiError> {
        let client_id = self.id.ok_or(EfiError::BuilderMissingField("client_id"))?;
        let client_secret = self
            .secret
            .ok_or(EfiError::BuilderMissingField("client_secret"))?;

        let http_client = if let Some(http_client) = self.http {
            if self.mtls_source.is_some() {
                return Err(EfiError::BuilderConflict(
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
        }
    }

    #[must_use]
    pub const fn endpoints(&self) -> Endpoints {
        self.environment.endpoints()
    }

    pub(crate) fn send_authenticated<Req, Res>(
        &self,
        method: Method,
        path: &str,
        payload: Option<&Req>,
    ) -> Result<Res, EfiError>
    where
        Req: Serialize,
        Res: DeserializeOwned,
    {
        let token = self.get_valid_access_token()?;
        let first_response =
            self.send_with_token_typed::<Req>(&token, method.clone(), path, payload)?;

        if first_response.status() == StatusCode::UNAUTHORIZED {
            self.authenticate()?;
            let refreshed_token = self.get_valid_access_token()?;
            let retry_response =
                self.send_with_token_typed::<Req>(&refreshed_token, method, path, payload)?;
            return Self::parse_response::<Res>(retry_response);
        }

        Self::parse_response::<Res>(first_response)
    }

    fn send_with_token_typed<Req>(
        &self,
        access_token: &str,
        method: Method,
        path: &str,
        payload: Option<&Req>,
    ) -> Result<reqwest::blocking::Response, EfiError>
    where
        Req: Serialize,
    {
        let endpoints = self.endpoints();
        let url = format!("{}{}", endpoints.pix_api_base_url, path);

        let mut request = self.http.request(method, url).bearer_auth(access_token);

        if let Some(json_payload) = payload {
            request = request.json(json_payload);
        }

        Ok(request.send()?)
    }

    fn parse_response<Res>(response: reqwest::blocking::Response) -> Result<Res, EfiError>
    where
        Res: DeserializeOwned,
    {
        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().unwrap_or_else(|_| String::new());
            return Err(EfiError::RequestFailed { status, body });
        }

        let body = response.text()?;
        if body.trim().is_empty() {
            return Err(EfiError::EmptyResponse);
        }

        Ok(serde_json::from_str(&body)?)
    }
}
