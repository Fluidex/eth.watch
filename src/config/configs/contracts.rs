use crate::types::Address;
use serde::Deserialize;

/// Data about deployed contracts.
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct ContractsConfig {
    pub contract_addr: Address,
}
