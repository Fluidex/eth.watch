use crate::types::{AccountId, Deposit};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DepositOp {
    pub priority_op: Deposit,
    pub account_id: AccountId,
}

impl DepositOp {
    pub const CHUNKS: usize = 6; // TODO: ?
    pub const OP_CODE: u8 = 0x01;
}
