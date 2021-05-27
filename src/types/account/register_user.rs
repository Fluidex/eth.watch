use crate::basic_types::{Address, Log, H256, U256};
use crate::types::utils::h256_as_vec;
use anyhow::format_err;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FluidexRegUserOp {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegUserOp {
    /// register_user operation.
    pub data: FluidexRegUserOp,
    #[serde(with = "h256_as_vec")]
    /// Hash of the corresponding Ethereum transaction. Size should be 32 bytes
    pub eth_hash: H256,
    /// Block in which Ethereum transaction was included.
    pub eth_block: u64,
}
