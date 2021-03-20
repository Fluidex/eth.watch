use serde::{Deserialize, Serialize};
use std::str::FromStr;

pub use crate::config::configs::{ContractsConfig, ETHClientConfig, ETHSenderConfig};

pub mod configs;

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Settings {
    pub contracts: ContractsConfig,
    pub eth_client: ETHClientConfig,
    pub eth_sender: ETHSenderConfig,
}
