//! The client implementation for the reqwest HTTP client, which is async by
//! default.

use std::convert::TryInto;

use maybe_async::{async_impl, sync_impl};
use reqwest::Method;
use serde_json::Value;

use crate::{
    config::Config,
    error::{Error, Result},
};

use super::HttpClient;

#[derive(Clone, Debug)]
pub struct Client {
    #[cfg(feature = "__async")]
    inner: reqwest::Client,
    #[cfg(feature = "__sync")]
    inner: reqwest::blocking::Client,
    config: Config,
}

impl Client {
    fn endpoint_url(&self, url: &str) -> String {
        self.config.url.clone() + url
    }
}

#[async_impl]
impl HttpClient for Client {
    /// Convert [`Config`] into a [`Client`]
    fn try_from(config: Config) -> Result<Self, Error> {
        let builder: reqwest::ClientBuilder = config.clone().try_into()?;

        Ok(Self {
            inner: builder.build()?,
            config,
        })
    }

    #[inline]
    async fn post(&self, url: &str, payload: &Value) -> Result<String> {
        let mut request = self.inner.request(Method::POST, self.endpoint_url(url));
        request = request.json(payload);

        let response = request.send().await?;

        if response.status().is_success() {
            return response.text().await.map_err(Into::into);
        }

        Err(Error::from_response(response).await)
    }
}

#[sync_impl]
impl HttpClient for Client {
    /// Convert [`Config`] into a [`Client`]
    fn try_from(config: Config) -> Result<Self, Error> {
        let builder: reqwest::blocking::ClientBuilder = config.clone().try_into()?;

        Ok(Self {
            inner: builder.build()?,
            config,
        })
    }

    #[inline]
    fn post(&self, url: &str, payload: &Value) -> Result<String> {
        let mut request: reqwest::blocking::RequestBuilder = self.inner.request(Method::POST, &self.endpoint_url(url));

        request = request.json(payload);

        let response = request.send().unwrap();

        if response.status().is_success() {
            return response.text().map_err(Into::into);
        }

        Err(Error::from_response(response))
    }
}
