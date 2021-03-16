use serde::Deserialize;

/// Data about deployed contracts.
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct ContractsConfig {
    // TODO:
    pub contract_addr: String,
}
