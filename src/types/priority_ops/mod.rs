use super::{
    // operations::{DepositOp, FullExitOp},
    utils::h256_as_vec,
    AccountId,
    SerialId,
    TokenId,
};
use crate::basic_types::{Address, Log, H256, U256};
use crate::utils::BigUintSerdeAsRadix10Str;
use anyhow::{bail, ensure, format_err};
use num::{BigUint, ToPrimitive};
use serde::{Deserialize, Serialize};
use std::convert::{TryFrom, TryInto};

/// Deposit priority operation transfers funds from the L1 account to the desired L2 account.
/// If the target L2 account didn't exist at the moment of the operation execution, a new
/// account will be created.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Deposit {
    /// Address of the transaction initiator's L1 account.
    pub from: Address,
    /// Type of deposited token.
    pub token: TokenId,
    /// Amount of tokens deposited.
    #[serde(with = "BigUintSerdeAsRadix10Str")]
    pub amount: BigUint,
    /// Address of L2 account to deposit funds to.
    pub to: Address,
}

/// Performs a withdrawal of funds without direct interaction with the L2 network.
/// All the balance of the desired token will be withdrawn to the provided L1 address.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FullExit {
    pub account_id: AccountId,
    pub eth_address: Address,
    pub token: TokenId,
}

/// A set of L1 priority operations supported by the Fluidex network.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum FluidexPriorityOp {
    Deposit(Deposit),
    FullExit(FullExit),
}

impl FluidexPriorityOp {
    /// Returns the amount of chunks required to include the priority operation into the block.
    pub fn chunks(&self) -> usize {
        unimplemented!();
    }
}

/// Priority operation description with the metadata required for server to process it.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriorityOp {
    /// Unique ID of the priority operation.
    pub serial_id: SerialId,
    /// Priority operation.
    pub data: FluidexPriorityOp,
    /// Ethereum deadline block until which operation must be processed.
    pub deadline_block: u64,
    #[serde(with = "h256_as_vec")]
    /// Hash of the corresponding Ethereum transaction. Size should be 32 bytes
    pub eth_hash: H256,
    /// Block in which Ethereum transaction was included.
    pub eth_block: u64,
}

impl TryFrom<Log> for PriorityOp {
    type Error = anyhow::Error;

    fn try_from(event: Log) -> Result<PriorityOp, anyhow::Error> {
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
        Ok(PriorityOp {
            serial_id: dec_ev.remove(0).to_uint().as_ref().map(U256::as_u64).unwrap(),
            data: {
                let op_type = dec_ev.remove(0).to_uint().as_ref().map(|ui| U256::as_u32(ui) as u8).unwrap();
                let op_pubdata = dec_ev.remove(0).to_bytes().unwrap();
                // TODO:
                FluidexPriorityOp::parse_from_priority_queue_logs(&op_pubdata, op_type, sender)?
            },
            deadline_block: dec_ev.remove(0).to_uint().as_ref().map(U256::as_u64).unwrap(),
            eth_hash: event.transaction_hash.expect("Event transaction hash is missing"),
            eth_block: event.block_number.expect("Event block number is missing").as_u64(),
        })
    }
}
