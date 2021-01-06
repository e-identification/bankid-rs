use derive_builder::Builder;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(setter(strip_option))]
pub struct Collect {
    /// The orderRef in question.
    pub order_ref: String,
    /// The status of the order in question.
    ///
    /// # Status
    /// pending - The order is being processed. hintCode describes the status of the order.
    /// failed - Something went wrong with the order. hintCode describes the error.
    /// complete - The order is complete. completionData holds user information.
    pub status: String,
    /// Describes the status of the order.
    pub hint_code: String,
    /// Only present for complete orders.
    pub completion_data: Option<CompletionData>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Builder)]
#[serde(rename_all = "camelCase")]
pub struct CollectPayload {
    /// Used to collect the status of the order.
    pub order_ref: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CompletionData {
    /// Information related to the user.
    user: User,
    /// Information related to the device.
    device: Device,
    /// Information related to the user’s certificate.
    cert: Cert,
    /// The signature. Base64-encoded
    signature: String,
    ocsp_response: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Builder)]
#[serde(rename_all = "camelCase")]
pub struct User {
    /// The personal number
    personal_number: String,
    /// The given name and surname of the user
    name: String,
    /// The given name of the user.
    given_name: String,
    /// The surname of the user.
    surname: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Builder)]
#[serde(rename_all = "camelCase")]
pub struct Device {
    /// The IP address of the user agent as the BankID server discovers it
    ip_address: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Builder)]
#[serde(rename_all = "camelCase")]
pub struct Cert {
    ///  Start of validity of the users BankID
    not_before: String,
    /// End of validity of the Users BankID
    not_after: String,
}
