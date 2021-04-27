use super::{utils::h256_as_vec, SerialId};
use crate::basic_types::{Log, H256, U256};
use anyhow::{bail, ensure, format_err};
use serde::{Deserialize, Serialize};
use std::convert::{TryFrom, TryInto};

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

impl TryFrom<Log> for NewTokenOp {
    type Error = anyhow::Error;

    fn try_from(event: Log) -> Result<NewTokenOp, anyhow::Error> {
        let mut dec_ev = ethabi::decode(
            &[
                ethabi::ParamType::Address,
                ethabi::ParamType::Uint(64),  // Serial id
                ethabi::ParamType::Uint(8),   // OpType
                ethabi::ParamType::Bytes,     // Pubdata
                ethabi::ParamType::Uint(256), // expir. block
            ],
            &event.data.0,
        )
        .map_err(|e| format_err!("Event data decode: {:?}", e))?;

        let sender = dec_ev.remove(0).to_address().unwrap();
        Ok(NewTokenOp {
            serial_id: dec_ev.remove(0).to_uint().as_ref().map(U256::as_u64).unwrap(),
            // TODO:
            data: FluidexNewTokenOp {},
            deadline_block: dec_ev.remove(0).to_uint().as_ref().map(U256::as_u64).unwrap(),
            eth_hash: event.transaction_hash.expect("Event transaction hash is missing"),
            eth_block: event.block_number.expect("Event block number is missing").as_u64(),
        })
    }
}
