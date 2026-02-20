use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum EfiError {
    BuilderMissingField(&'static str),
    BuilderConflict(&'static str),
    Io(std::io::Error),
    Http(reqwest::Error),
    Json(serde_json::Error),
    AuthUnavailable,
    EmptyResponse,
    RequestFailed {
        status: reqwest::StatusCode,
        body: String,
    },
}

impl Display for EfiError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BuilderMissingField(field) => {
                write!(f, "missing required builder field: {field}")
            }
            Self::BuilderConflict(message) => write!(f, "invalid builder configuration: {message}"),
            Self::Io(error) => write!(f, "io error: {error}"),
            Self::Http(error) => write!(f, "http error: {error}"),
            Self::Json(error) => write!(f, "json error: {error}"),
            Self::AuthUnavailable => write!(f, "authentication token is unavailable"),
            Self::EmptyResponse => write!(f, "received empty response from server"),
            Self::RequestFailed { status, body } => {
                write!(f, "request failed with status {status}: {body}")
            }
        }
    }
}

impl Error for EfiError {}

impl From<std::io::Error> for EfiError {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<reqwest::Error> for EfiError {
    fn from(value: reqwest::Error) -> Self {
        Self::Http(value)
    }
}

impl From<serde_json::Error> for EfiError {
    fn from(value: serde_json::Error) -> Self {
        Self::Json(value)
    }
}
