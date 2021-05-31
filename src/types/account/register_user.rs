use crate::basic_types::{Address, Log, H256, U256};
use crate::types::{utils::h256_as_vec, AccountId, L2Pubkey};
use anyhow::format_err;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FluidexRegUserOp {
    pub l1_address: Address,
    pub user_id: u16, // TODO: change to AccountId (u32)
    pub l2_pubkey: L2Pubkey,
}

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

impl TryFrom<Log> for RegUserOp {
    type Error = anyhow::Error;

    fn try_from(event: Log) -> Result<RegUserOp, anyhow::Error> {
        let mut dec_ev = ethabi::decode(
            &[
                ethabi::ParamType::Address,  // l1_address
                ethabi::ParamType::Uint(16), // user_id
                ethabi::ParamType::Bytes,    // l2_pubkey. TODO: FixedBytes?
            ],
            &event.data.0,
        )
        .map_err(|e| format_err!("Event data decode: {:?}", e))?;

        let l1_address = dec_ev.remove(0).to_address().unwrap();
        let user_id = dec_ev.remove(0).to_uint().as_ref().map(|ui| U256::as_u32(ui) as u16).unwrap();
        let l2_pubkey = dec_ev.remove(0).to_bytes().unwrap();

        log::info!("{}", l1_address);
        log::info!("{}", user_id);
        log::info!("{:?}", l2_pubkey);

        Ok(RegUserOp {
            data: FluidexRegUserOp {
                l1_address,
                user_id: user_id,
                l2_pubkey: L2Pubkey::from_slice(&l2_pubkey),
            },
            eth_hash: event.transaction_hash.expect("Event transaction hash is missing"),
            eth_block: event.block_number.expect("Event block number is missing").as_u64(),
        })
    }
}
