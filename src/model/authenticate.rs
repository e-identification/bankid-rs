use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::model::Requirement;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Authenticate {
    /// Used to compile the start url
    pub auto_start_token: String,
    /// Used to collect the status of the order
    pub order_ref: String,
    /// Used to compute the animated QR code
    pub qr_start_token: String,
    /// Used to compute the animated QR code
    pub qr_start_secret: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(setter(strip_option))]
pub struct AuthenticatePayload {
    /// The personal number of the user. String. 12 digits. Century must be included.
    /// If the personal number is excluded, the client must be started with the
    /// autoStartToken returned in the response
    pub personal_number: Option<String>,
    /// The user IP address as seen by RP. String. IPv4 and IPv6 is allowed.
    /// Note the importance of using the correct IP address. It must be the IP address
    /// representing the user agent (the end user device) as seen by the RP. If there is a
    /// proxy for inbound traffic, special considerations may need to be taken to get the
    /// correct address. In some use cases the IP address is not available, for instance for
    /// voice based services. In this case, the internal representation of those systems IP
    /// address is ok to use
    pub end_user_ip: String,
    /// Requirements on how the auth or sign order must be performed
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default = "Option::None")]
    pub requirement: Option<Requirement>,
}
