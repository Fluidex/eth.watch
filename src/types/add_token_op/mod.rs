use super::{utils::h256_as_vec, TokenId};
use crate::basic_types::{Address, Log, H256, U256};
use anyhow::format_err;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FluidexAddTokenOp {
    pub token_address: Address,
    pub token_id: TokenId,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddTokenOp {
    /// add_token operation.
    pub data: FluidexAddTokenOp,
    #[serde(with = "h256_as_vec")]
    /// Hash of the corresponding Ethereum transaction. Size should be 32 bytes
    pub eth_hash: H256,
    /// Block in which Ethereum transaction was included.
    pub eth_block: u64,
}

impl TryFrom<Log> for AddTokenOp {
    type Error = anyhow::Error;

    fn try_from(event: Log) -> Result<AddTokenOp, anyhow::Error> {
        let mut dec_ev = ethabi::decode(
            &[
                ethabi::ParamType::Address,  // token_address
                ethabi::ParamType::Uint(16), // token_id
            ],
            &event.data.0,
        )
        .map_err(|e| format_err!("AddTokenOp Event data decode: {:?}", e))?;

        let token_address = dec_ev.remove(0).to_address().unwrap();
        let token_id = dec_ev.remove(0).to_uint().as_ref().map(|ui| U256::as_u32(ui) as u16).unwrap();
        Ok(AddTokenOp {
            data: FluidexAddTokenOp {
                token_address,
                token_id: TokenId(token_id),
            },
            eth_hash: event.transaction_hash.expect("Event transaction hash is missing"),
            eth_block: event.block_number.expect("Event block number is missing").as_u64(),
        })
    }
}
