use crate::types::{AccountId, Deposit};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DepositOp {
    pub priority_op: Deposit,
    pub account_id: AccountId,
}

impl DepositOp {
    pub const CHUNKS: usize = 6; // TODO: need to double check
    pub const OP_CODE: u8 = 0x01; // need to be consistent with Operations.sol
}
