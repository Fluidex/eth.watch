use serde::Deserialize;

pub use crate::config::configs::{ContractsConfig, ETHClientConfig, ETHSenderConfig, ETHWatchConfig};

pub mod configs;

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Settings {
    pub contracts: ContractsConfig,
    pub eth_client: ETHClientConfig,
    pub eth_sender: ETHSenderConfig,
    pub eth_watch: ETHWatchConfig,
}
