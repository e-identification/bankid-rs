//! Crate for interacting with the BankID API.
//!
//! This crate includes tools for interacting with the BankID API v5.1.
//!
//! The crate support asynchronous paradigm with `async` and `await` by leveraging `reqwest` crate.
//!
//! ## Supported endpoints
//!
//! ### Authentication
//! Initiates an authentication order.
//!
//! ### Sign
//! Initiates an signing order.
//!
//! ## Collect
//! Collects the result of a sign or auth order using the orderRef as reference. RP should keep on
//! calling collect every two seconds as long as status indicates pending. RP must abort if status
//! indicates failed. The user identity is returned when complete.
//!
//! ## Cancel
//! Cancels an ongoing sign or auth order. This is typically used if the user cancels the order
//! in your service or app.
//!
//! ```toml
//! [dependencies]
//! bankid-rs = {
//!     version = "0.1.0",
//! }
//! ```
//!
//! [`reqwest`](https://docs.rs/reqwest/#proxies) supports system proxies by
//! default. It reads the environment variables `HTTP_PROXY` and `HTTPS_PROXY`
//! environmental variables to set HTTP and HTTPS proxies, respectively.
//!
//!
//! ## Examples
//!
//! There are some available examples [(https://github.com/NicklasWallgren/bankid-rs/examples)]
//!
//!

pub mod client;
pub mod config;
pub mod error;
mod http;
pub mod model;
mod tls;
