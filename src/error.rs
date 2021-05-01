use std::fmt;

use reqwest::StatusCode;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiError {
    error_code: String,
    details: String,
}

impl std::error::Error for ApiError {}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(errorCode: {}, details: {})", self.error_code, self.details)
    }
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("request error: {0}")]
    Request(String),

    #[error("status code {0}: {1}")]
    StatusCode(u16, String),

    #[error("exceeded request limit")]
    RateLimited(Option<usize>),

    #[error("bad request")]
    BadRequest,

    #[error("request unauthorized")]
    Unauthorized,

    #[error("api error: {0}")]
    Api(#[from] ApiError),

    #[error("json parse error: {0}")]
    ParseJson(#[from] serde_json::Error),

    #[error("url parse error: {0}")]
    ParseUrl(#[from] url::ParseError),

    #[error("input/output error: {0}")]
    IO(#[from] std::io::Error),

    #[error("Read error")]
    ReadError { source: std::io::Error },

    #[error("Parsing PKCS12 error")]
    Pkcs12Error { source: reqwest::Error },

    #[error("Parsing Certificate error")]
    CertificateError { source: reqwest::Error },
}

impl Error {
    #[cfg(feature = "__async")]
    pub async fn from_response(response: reqwest::Response) -> Self {
        match response.status() {
            // StatusCode::BAD_REQUEST => Self::BadRequest,
            status @ StatusCode::UNAUTHORIZED
            | status @ StatusCode::BAD_REQUEST
            | status @ StatusCode::PAYMENT_REQUIRED
            | status @ StatusCode::FORBIDDEN => response
                .json::<ApiError>()
                .await
                .map(Into::into)
                .unwrap_or_else(|_| status.into()),
            status => status.into(),
        }
    }

    #[cfg(feature = "__sync")]
    pub fn from_response(response: reqwest::blocking::Response) -> Self {
        match response.status() {
            // StatusCode::BAD_REQUEST => Self::BadRequest,
            status @ StatusCode::UNAUTHORIZED
            | status @ StatusCode::BAD_REQUEST
            | status @ StatusCode::PAYMENT_REQUIRED
            | status @ StatusCode::FORBIDDEN => response
                .json::<ApiError>()
                .map(Into::into)
                .unwrap_or_else(|_| status.into()),
            status => status.into(),
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Self::Request(err.to_string())
    }
}

impl From<reqwest::StatusCode> for Error {
    fn from(code: reqwest::StatusCode) -> Self {
        Self::StatusCode(code.as_u16(), code.canonical_reason().unwrap_or("unknown").to_string())
    }
}
