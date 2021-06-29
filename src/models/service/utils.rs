use crate::models::commons::Operation;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct SafeTransactionEstimationRequest {
    pub to: String,
    pub value: String,
    pub data: String,
    pub operation: Operation,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SafeTransactionEstimation {
    pub safe_tx_gas: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DataDecoderRequest {
    pub data: String,
}
