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

/// Requirement restricts the type of BankID that can be used as well as other requirements.
///
/// # Notice
/// If personal number is included in the call to the service, RP must
/// consider setting the requirement tokenStartRequired to true. By this, the
/// system enforces that no other device than the one started using the QR code
/// or autostarttoken is used.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Builder, Default)]
#[serde(rename_all = "camelCase")]
#[builder(setter(strip_option))]
pub struct Requirement {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default = "Option::None")]
    pub card_reader: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[builder(default = "vec!()")]
    pub certificate_policies: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default = "Option::None")]
    pub issuer_cn: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default = "Option::None")]
    pub auto_start_token_required: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default = "Option::None")]
    pub allow_fingerprint: Option<bool>,
}
