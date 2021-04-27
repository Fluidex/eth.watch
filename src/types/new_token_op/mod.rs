use super::{utils::h256_as_vec, SerialId};
use crate::basic_types::H256;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FluidexNewTokenOp {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewTokenOp {
    /// Unique ID of the priority operation.
    pub serial_id: SerialId,
    /// new_token operation.
    pub data: FluidexNewTokenOp,
    /// Ethereum deadline block until which operation must be processed.
    pub deadline_block: u64,
    #[serde(with = "h256_as_vec")]
    /// Hash of the corresponding Ethereum transaction. Size should be 32 bytes
    pub eth_hash: H256,
    /// Block in which Ethereum transaction was included.
    pub eth_block: u64,
}
