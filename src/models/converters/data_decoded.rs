use crate::models::commons::{DataDecoded, ParamValue, Parameter, ValueDecodedType};
use crate::models::service::transactions::SettingsInfo;
use crate::providers::address_info::AddressInfo;
use crate::providers::info::InfoProvider;
use crate::utils::{
    ADD_OWNER_WITH_THRESHOLD, CHANGE_MASTER_COPY, CHANGE_THRESHOLD, DISABLE_MODULE, ENABLE_MODULE,
    MULTI_SEND, MULTI_SEND_TRANSACTIONS_PARAM, REMOVE_OWNER, SET_FALLBACK_HANDLER, SWAP_OWNER,
};
use std::collections::HashMap;

impl DataDecoded {
    pub(super) fn to_settings_info(
        &self,
        info_provider: &mut dyn InfoProvider,
    ) -> Option<SettingsInfo> {
        match self.method.as_str() {
            SET_FALLBACK_HANDLER => {
                let handler = self.get_parameter_single_value_at(0)?;
                Some(SettingsInfo::SetFallbackHandler {
                    handler_info: info_provider.contract_info(&handler).ok(),
                    handler,
                })
            }
            ADD_OWNER_WITH_THRESHOLD => {
                let owner = self.get_parameter_single_value_at(0)?;
                Some(SettingsInfo::AddOwner {
                    owner_info: None,
                    owner,
                    threshold: self.get_parameter_single_value_at(1)?.parse().ok()?,
                })
            }
            REMOVE_OWNER => {
                let owner = self.get_parameter_single_value_at(1)?;
                Some(SettingsInfo::RemoveOwner {
                    owner_info: None,
                    owner,
                    threshold: self.get_parameter_single_value_at(2)?.parse().ok()?,
                })
            }
            SWAP_OWNER => {
                let old_owner = self.get_parameter_single_value_at(1)?;
                let new_owner = self.get_parameter_single_value_at(2)?;
                Some(SettingsInfo::SwapOwner {
                    old_owner_info: None,
                    old_owner,
                    new_owner_info: None,
                    new_owner,
                })
            }
            CHANGE_MASTER_COPY => {
                let implementation = self.get_parameter_single_value_at(0)?;
                Some(SettingsInfo::ChangeImplementation {
                    implementation_info: info_provider.contract_info(&implementation).ok(),
                    implementation,
                })
            }
            ENABLE_MODULE => {
                let module = self.get_parameter_single_value_at(0)?;
                Some(SettingsInfo::EnableModule {
                    module_info: info_provider.contract_info(&module).ok(),
                    module,
                })
            }
            DISABLE_MODULE => {
                let module = self.get_parameter_single_value_at(1)?;
                Some(SettingsInfo::DisableModule {
                    module_info: info_provider.contract_info(&module).ok(),
                    module,
                })
            }
            CHANGE_THRESHOLD => Some(SettingsInfo::ChangeThreshold {
                threshold: self.get_parameter_single_value_at(0)?.parse().ok()?,
            }),
            _ => None,
        }
    }
}

impl DataDecoded {
    pub(super) fn build_address_info_index(
        &self,
        info_provider: &mut impl InfoProvider,
    ) -> Option<HashMap<String, AddressInfo>> {
        let mut index = HashMap::new();
        if self.method == MULTI_SEND {
            if let Some(value_decoded_type) =
                &self.get_parameter_value_decoded(MULTI_SEND_TRANSACTIONS_PARAM)
            {
                match value_decoded_type {
                    ValueDecodedType::InternalTransaction(transactions) => {
                        transactions.iter().for_each(|transaction| {
                            insert_value_into_index(&transaction.to, &mut index, info_provider);
                            put_parameter_into_index(
                                &transaction
                                    .data_decoded
                                    .as_ref()
                                    .map(|it| it.parameters.to_owned())
                                    .flatten(),
                                &mut index,
                                info_provider,
                            )
                        })
                    }
                }
            }
        } else {
            put_parameter_into_index(&self.parameters, &mut index, info_provider);
        }
        if index.is_empty() {
            None
        } else {
            Some(index)
        }
    }
}

fn put_parameter_into_index(
    parameters: &Option<Vec<Parameter>>,
    index: &mut HashMap<String, AddressInfo>,
    info_provider: &mut impl InfoProvider,
) {
    if let Some(parameters) = parameters {
        for parameter in parameters {
            match &parameter.value {
                ParamValue::SingleValue(value) => {
                    insert_value_into_index(value, index, info_provider)
                }
                ParamValue::ArrayValue(values) => {
                    for value in values {
                        if let ParamValue::SingleValue(value) = value {
                            insert_value_into_index(value, index, info_provider)
                        }
                    }
                }
            }
        }
    }
}

fn insert_value_into_index(
    value: &String,
    index: &mut HashMap<String, AddressInfo>,
    info_provider: &mut impl InfoProvider,
) {
    if value.len() == 42
        && value.starts_with("0x")
        && value != "0x0000000000000000000000000000000000000000"
        && !index.contains_key(value)
    {
        if let Some(address_info) = info_provider.full_address_info_search(&value).ok() {
            index.insert(value.to_owned(), address_info);
        };
    }
}
