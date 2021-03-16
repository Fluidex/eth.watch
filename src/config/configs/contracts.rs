// External uses
use serde::Deserialize;
// Local uses
use crate::types::{Address};

/// Data about deployed contracts.
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct ContractsConfig {
    // TODO:
    pub contract_addr: Address,
}
