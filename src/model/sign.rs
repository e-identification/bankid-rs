use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::model::Requirement;

use super::UserVisibleDataFormat;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Sign {
    /// Used to compile the start url
    pub auto_start_token: String,
    /// Used to collect the status of the order.
    pub order_ref: String,
    /// Used to compute the animated QR code
    pub qr_start_token: String,
    /// Used to compute the animated QR code
    pub qr_start_secret: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(setter(strip_option))]
pub struct SignPayload {
    /// The user IP address as seen by RP. String. IPv4 and IPv6 is allowed.
    /// Note the importance of using the correct IP address. It must be the IP address
    /// representing the user agent (the end user device) as seen by the RP. If there is a
    /// proxy for inbound traffic, special considerations may need to be taken to get the
    /// correct address. In some use cases the IP address is not available, for instance for
    /// voice based services. In this case, the internal representation of those systems IP
    /// address is ok to use
    pub end_user_ip: String,
    /// The text to be displayed and signed. String. The text can be formatted using CR,
    /// LF and CRLF for new lines. The text must be encoded as UTF-8 and then base 64
    /// encoded. 1-40 000 characters after base 64 encoding.
    pub user_visible_data: String,
    /// Data not displayed to the user. String. The value must be base 64-encoded.
    /// 1-200 000 characters after base 64-encoding.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default = "Option::None")]
    pub user_non_visible_data: Option<String>,
    /// If present, and set to “simpleMarkdownV1”, this parameter indicates that
    /// userVisibleData holds formatting characters which, if used correctly, will make
    /// the text displayed with the user nicer to look at. For further information of
    /// formatting options, please study the document Guidelines for Formatted Text.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default = "Option::None")]
    pub user_visible_data_format: Option<UserVisibleDataFormat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default = "Option::None")]
    pub requirement: Option<Requirement>,
}
