use serde::Serialize;
use crate::models::commons::DataDecoded;

pub mod details;
pub mod summary;

pub const ID_SEPARATOR: &str = "_";
pub const ID_PREFIX_MULTISIG_TX: &str = "multisig";
pub const ID_PREFIX_MODULE_TX: &str = "module";
pub const ID_PREFIX_ETHEREUM_TX: &str = "ethereum";
pub const ID_PREFIX_CREATION_TX: &str = "creation";

#[derive(Serialize, Debug, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TransactionStatus {
    AwaitingConfirmations,
    AwaitingExecution,
    Cancelled,
    Failed,
    Success,
}

#[derive(Serialize, Debug, PartialEq)]
#[serde(tag = "type")]
pub enum TransactionInfo {
    Transfer(Transfer),
    SettingsChange(SettingsChange),
    Custom(Custom),
    Creation(Creation),
    Unknown,
}

#[derive(Serialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Transfer {
    pub sender: String,
    pub recipient: String,
    pub direction: TransferDirection,
    pub transfer_info: TransferInfo,
}

#[derive(Serialize, Debug, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TransferDirection {
    Incoming,
    Outgoing,
    Unknown,
}

#[derive(Serialize, Debug, PartialEq)]
#[serde(tag = "type", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TransferInfo {
    Erc20(Erc20Transfer),
    Erc721(Erc721Transfer),
    Ether(EtherTransfer),
}

#[derive(Serialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Erc20Transfer {
    pub token_address: String,
    pub token_name: Option<String>,
    pub token_symbol: Option<String>,
    pub logo_uri: Option<String>,
    pub decimals: Option<u64>,
    pub value: String,
}

#[derive(Serialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Erc721Transfer {
    pub token_address: String,
    pub token_id: String,
    pub token_name: Option<String>,
    pub token_symbol: Option<String>,
    pub logo_uri: Option<String>,
}

#[derive(Serialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct EtherTransfer {
    pub value: String,
}

#[derive(Serialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SettingsChange {
    pub data_decoded: DataDecoded,
    pub settings_info: SettingsInfo,
}

#[derive(Serialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum SettingsInfo {

}

#[derive(Serialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Custom {
    pub to: String,
    pub data_size: String,
    pub value: String,
}

#[derive(Serialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Creation {
    pub creator: String,
    pub transaction_hash: String,
    pub implementation: Option<String>,
    pub factory: Option<String>,
}