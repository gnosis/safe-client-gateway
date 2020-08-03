extern crate chrono;

use crate::models::backend::transactions::{ModuleTransaction, MultisigTransaction};
use crate::models::commons::Operation;
use crate::models::service::transactions::details::{
    DetailedExecutionInfo, TransactionData, TransactionDetails,
};
use crate::models::service::transactions::TransactionStatus;
use crate::providers::info::InfoProvider;
use anyhow::Result;

impl MultisigTransaction {
    pub fn to_transaction_details(
        &self,
        info_provider: &mut InfoProvider,
    ) -> Result<TransactionDetails> {
        let safe_info = info_provider.safe_info(&self.safe.to_string())?;
        Ok(TransactionDetails {
            executed_at: self.execution_date.map(|data| data.timestamp_millis()),
            submitted_at: Some(self.submission_date.timestamp_millis()),
            tx_status: self.map_status(&safe_info),
            tx_info: self.transaction_info(info_provider),
            tx_data: self.data.as_ref().map(|data| TransactionData {
                hex_data: data.to_owned(),
                data_decoded: self.data_decoded.clone(),
            }),
            tx_hash: self.transaction_hash.as_ref().map(|hash| hash.to_owned()),
            detailed_execution_info: Some(DetailedExecutionInfo {
                nonce: self.nonce,
                operation: self.operation.unwrap_or(Operation::CALL),
                safe_tx_hash: self.safe_tx_hash.to_owned(),
                signers: safe_info.owners,
                confirmations_required: self.confirmations_required.unwrap_or(safe_info.threshold),
                confirmations: self
                    .confirmations
                    .as_ref()
                    .unwrap_or(&vec![])
                    .into_iter()
                    .map(|confirmation| confirmation.owner.to_owned())
                    .collect(),
            }),
        })
    }
}

impl ModuleTransaction {
    pub fn to_transaction_details(&self) -> Result<TransactionDetails> {
        Ok(TransactionDetails {
            executed_at: Some(self.execution_date.timestamp_millis()),
            submitted_at: None,
            tx_status: TransactionStatus::Success,
            tx_info: self.to_transaction_info(),
            tx_data: self.data.as_ref().map(|data| TransactionData {
                hex_data: data.to_owned(),
                data_decoded: self.data_decoded.clone(),
            }),
            tx_hash: Some(self.transaction_hash.to_owned()),
            detailed_execution_info: None,
        })
    }
}
