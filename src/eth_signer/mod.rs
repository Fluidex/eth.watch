use crate::types::tx::TxEthSignature;
use crate::types::Address;
use async_trait::async_trait;
use error::SignerError;

pub use json_rpc_signer::JsonRpcSigner;
pub use pk_signer::PrivateKeySigner;
pub use raw_ethereum_tx::RawTransaction;

pub mod error;
pub mod json_rpc_signer;
pub mod pk_signer;
pub mod raw_ethereum_tx;

#[async_trait]
pub trait EthereumSigner: Send + Sync + Clone {
    async fn sign_message(&self, message: &[u8]) -> Result<TxEthSignature, SignerError>;
    async fn sign_transaction(&self, raw_tx: RawTransaction) -> Result<Vec<u8>, SignerError>;
    async fn get_address(&self) -> Result<Address, SignerError>;
}
