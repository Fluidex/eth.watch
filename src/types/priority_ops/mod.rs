use super::{operations::DepositOp, utils::h256_as_vec, AccountId, SerialId, TokenId};
use crate::basic_types::{Address, Log, H256, U256};
use crate::params::{ACCOUNT_ID_BIT_WIDTH, BALANCE_BIT_WIDTH, FR_ADDRESS_LEN, TOKEN_BIT_WIDTH, TX_TYPE_BIT_WIDTH, BJJ_ADDRESS_LEN};
use crate::utils::BigUintSerdeAsRadix10Str;
use anyhow::{bail, ensure, format_err};
use num::BigUint;
// use num::ToPrimitive;
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
    pub to: web3::types::H256,
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
    /// Parses priority operation from the Ethereum logs.
    pub fn parse_from_priority_queue_logs(pub_data: &[u8], op_type_id: u8, sender: Address) -> Result<Self, anyhow::Error> {
        // see contracts/contracts/Operations.sol
        match op_type_id {
            DepositOp::OP_CODE => {
                let pub_data_left = pub_data;

                ensure!(pub_data_left.len() >= TX_TYPE_BIT_WIDTH / 8, "PubData length mismatch");
                let (_, pub_data_left) = pub_data_left.split_at(TX_TYPE_BIT_WIDTH / 8);

                // account_id
                ensure!(pub_data_left.len() >= ACCOUNT_ID_BIT_WIDTH / 8, "PubData length mismatch");
                let (_, pub_data_left) = pub_data_left.split_at(ACCOUNT_ID_BIT_WIDTH / 8);

                // token
                let (token, pub_data_left) = {
                    ensure!(pub_data_left.len() >= TOKEN_BIT_WIDTH / 8, "PubData length mismatch");
                    let (token, left) = pub_data_left.split_at(TOKEN_BIT_WIDTH / 8);
                    (u16::from_be_bytes(token.try_into().unwrap()), left)
                };

                // amount
                let (amount, pub_data_left) = {
                    ensure!(pub_data_left.len() >= BALANCE_BIT_WIDTH / 8, "PubData length mismatch");
                    let (amount, left) = pub_data_left.split_at(BALANCE_BIT_WIDTH / 8);
                    // TODO: double check this logic
                    let amount = BigUint::from_bytes_be(amount /*.try_into().unwrap()*/);
                    (amount, left)
                };

                // account
                let (account, pub_data_left) = {
                    ensure!(pub_data_left.len() >= BJJ_ADDRESS_LEN, "PubData length mismatch");
                    let (account, left) = pub_data_left.split_at(BJJ_ADDRESS_LEN);
                    (web3::types::H256::from_slice(account), left)
                };

                ensure!(pub_data_left.is_empty(), "DepositOp parse failed: input too big");

                Ok(Self::Deposit(Deposit {
                    from: sender,
                    token: TokenId(token),
                    amount,
                    to: account,
                }))
            }
            _ => {
                bail!("Unsupported priority op type");
            }
        }
    }

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
                FluidexPriorityOp::parse_from_priority_queue_logs(&op_pubdata, op_type, sender)?
            },
            deadline_block: dec_ev.remove(0).to_uint().as_ref().map(U256::as_u64).unwrap(),
            eth_hash: event.transaction_hash.expect("Event transaction hash is missing"),
            eth_block: event.block_number.expect("Event block number is missing").as_u64(),
        })
    }
}
