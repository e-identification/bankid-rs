use derive_builder::Builder;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Cancel {}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Builder)]
#[serde(rename_all = "camelCase")]
pub struct CancelPayload {
    // Used to collect the status of the order.
    pub order_ref: String,
}
