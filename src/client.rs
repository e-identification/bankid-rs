use derive_builder::Builder;
use maybe_async::maybe_async;
use serde::Deserialize;
use serde_json::json;

use crate::{
    config::Config,
    error::Result,
    http,
    http::HttpClient,
    model::{
        authenticate::{Authenticate, AuthenticatePayload},
        cancel::{Cancel, CancelPayload},
        collect::{Collect, CollectPayload},
        sign::{Sign, SignPayload},
    },
};

/// Client for communicating with the BankID API.
///
/// The best way to instantiate the client is with an existing [`Config`]
/// using [`Client::new`]
#[derive(Builder, Clone, Debug)]
pub struct BankID {
    pub(in crate) client: Box<dyn http::HttpClient>,
}

impl BankID {
    /// Create and initialize a [`BankID`] using the given configuration.
    ///
    /// # Panics
    /// Panics if the configuration supplied leads to an invalid [`HttpClient`].
    /// Refer to the [`reqwest::ClientBuilder::build`] docs for information
    /// on situations where this might fail.
    pub fn new(config: Config) -> Self {
        let client =
            http::client::Client::try_from(config).expect("Could not create a client from the supplied config");

        Self {
            client: Box::new(client),
        }
    }

    /// Authenticate - Initiates an authentication order.
    ///
    /// Use the collect method to query the status of the order.
    /// If the request is successful, the orderRef and autoStartToken is returned.
    #[maybe_async]
    pub async fn authenticate(&self, payload: AuthenticatePayload) -> Result<Authenticate> {
        let result = self.client.post("/auth", &json!(payload)).await?;
        self.convert_result(&result)
    }

    /// Sign - Initiates an sign order.
    ///
    /// Use the collect method to query the status of the order.
    /// If the request is successful, the orderRef and autoStartToken is returned.
    #[maybe_async]
    pub async fn sign(&self, payload: SignPayload) -> Result<Sign> {
        let result = self.client.post("/sign", &json!(payload)).await?;
        self.convert_result(&result)
    }

    /// Collect - Collects the result of a sign or auth order suing the orderRef as reference.
    ///
    /// RP should keep calling collect every two seconds as long as status indicates pending.
    /// RP must abort if status indicates failed. The user identity is returned when complete.
    #[maybe_async]
    pub async fn collect(&self, payload: CollectPayload) -> Result<Collect> {
        let result = self.client.post("/collect", &json!(payload)).await?;
        self.convert_result(&result)
    }

    /// Cancel - Cancels an ongoing sign or auth order.
    ///
    /// This is typically used if the user cancels the order in your service or app.
    #[maybe_async]
    pub async fn cancel(&self, payload: CancelPayload) -> Result<Cancel> {
        let result = self.client.post("/cancel", &json!(payload)).await?;
        self.convert_result(&result)
    }

    /// Converts a JSON response into its model.
    fn convert_result<'a, T: Deserialize<'a>>(&self, input: &'a str) -> Result<T> {
        serde_json::from_str::<T>(input).map_err(Into::into)
    }
}
