use serde::Deserialize;

/// Data about deployed contracts.
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct ContractsConfig {
    pub contract_addr: String,
}

impl Default for ContractsConfig {
    fn default() -> Self {
        ContractsConfig {
            contract_addr: Default::default(),
        }
    }
}
