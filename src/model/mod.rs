//! All BankID API endpoint request and response object

use derive_builder::Builder;
use serde::{Deserialize, Serialize};

pub use authenticate::*;
pub use cancel::*;
pub use collect::*;
pub use sign::*;

pub mod authenticate;
pub mod cancel;
pub mod collect;
pub mod sign;

/// RP may use the requirement parameter to describe how a signature must be created and verified.
/// A typical use case is to require Mobile BankID or a certain card reader.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Builder, Default)]
#[serde(rename_all = "camelCase")]
#[builder(setter(strip_option))]
pub struct Requirement {
    /// Users are required to sign the transaction with their PIN code, even if they have biometrics activated.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default = "Option::None")]
    pub pin_code: Option<bool>,
    /// If present, and set to "true",
    /// the client needs to provide MRTD (Machine readable travel document) information to complete the order.
    /// Only Swedish passports and national ID cards are supported.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default = "Option::None")]
    pub mrtd: Option<bool>,
    /// "class1" (default) – The transaction must be performed using a card reader where the PIN code is entered
    /// on a computer keyboard, or a card reader of higher class.
    /// "class2" – The transaction must be performed using a card reader where the PIN code is entered on the reader,
    /// or a reader of higher class.
    /// "<"no value">" – defaults to "class1". This condition should be combined with a certificatePolicies
    /// for a smart card to avoid undefined behaviour.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default = "Option::None")]
    pub card_reader: Option<String>,
    /// The oid in certificate policies in the user certificate.
    /// One wildcard ”” is allowed from position 5 and forward ie. 1.2.752.78.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[builder(default = "vec!()")]
    pub certificate_policies: Vec<String>,
    /// A personal identification number to be used to complete the transaction.
    /// If a BankID with another personal number attempts to sign the transaction, it fails.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default = "Option::None")]
    pub personal_number: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub enum UserVisibleDataFormat {
    #[serde(rename = "simpleMarkdownV1")]
    SimpleMarkdownV1,
}
