//! HTTP client for interacting with the BankID API

use dyn_clone::{clone_trait_object, DynClone};
use maybe_async::maybe_async;
use serde_json::Value;

use crate::{config::Config, error::Result};

pub mod client;

#[maybe_async]
pub trait HttpClient: DynClone + std::fmt::Debug {
    fn try_from(config: Config) -> Result<Self>
    where
        Self: Sized;
    async fn post(&self, url: &str, payload: &Value) -> Result<String>;
}

clone_trait_object!(HttpClient);
