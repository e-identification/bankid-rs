use derive_builder::Builder;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum Status {
    Pending,
    Complete,
    Failed,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum HintCode {
    // Pending
    OutstandingTransaction,
    NoClient,
    Started,
    UserMRTD,
    UserCallConfirm,
    UserSign,
    // Failed
    ExpiredTransaction,
    CertificateErr,
    UserCancel,
    Cancelled,
    StartFailed,
    UserDeclinedCall,
}

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
    pub status: Status,
    /// Describes the status of the order.
    pub hint_code: Option<HintCode>,
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
    pub user: User,
    /// Information related to the device.
    pub device: Device,
    /// The date the BankID was issued to the user.
    pub bank_id_issue_date: String,
    /// Information about extra verifications that were part of the transaction.
    /// mrtd: Indicate if there was a check of the mrtd (machine readable travel document).
    /// True if the mrtd check was performed and passed.
    pub step_up: Option<bool>,
    /// The signature. Base64-encoded
    pub signature: String,
    pub ocsp_response: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Builder)]
#[serde(rename_all = "camelCase")]
pub struct User {
    /// The personal number
    pub personal_number: String,
    /// The given name and surname of the user
    pub name: String,
    /// The given name of the user.
    pub given_name: String,
    /// The surname of the user.
    pub surname: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Builder)]
#[serde(rename_all = "camelCase")]
pub struct Device {
    /// The IP address of the user agent as the BankID server discovers it
    pub ip_address: String,
    /// Unique hardware identifier for the user’s device.
    pub uhi: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Builder)]
#[serde(rename_all = "camelCase")]
pub struct Cert {
    ///  Start of validity of the users BankID
    pub not_before: String,
    /// End of validity of the Users BankID
    pub not_after: String,
}
