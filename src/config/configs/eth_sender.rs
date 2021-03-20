use crate::types::{Address, H256};
use serde::Deserialize;
use std::time::Duration;

/// Configuration for the Ethereum sender crate.
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct ETHSenderConfig {
    /// Options related to the Ethereum sender directly.
    pub sender: Sender,
    /// Options related to the `gas_adjuster` submodule.
    pub gas_price_limit: GasLimit,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Sender {
    /// Private key of the operator account.
    pub operator_private_key: H256,
    /// Address of the operator account.
    pub operator_commit_eth_addr: Address,
    /// mount of confirmations required to consider L1 transaction committed.
    pub wait_confirmations: u64,
    /// Amount of blocks we will wait before considering L1 transaction stuck.
    pub expected_wait_time_block: u64,
    /// Node polling period in seconds.
    pub tx_poll_period: u64,
    /// The maximum amount of simultaneously sent Ethereum transactions.
    pub max_txs_in_flight: u64,
    /// Whether sender should interact with L1 or not.
    pub is_enabled: bool,
}

impl Sender {
    /// Converts `self.tx_poll_period` into `Duration`.
    pub fn tx_poll_period(&self) -> Duration {
        Duration::from_secs(self.tx_poll_period)
    }
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct GasLimit {
    /// Gas price limit to be used by GasAdjuster until the statistics data is gathered.
    pub default: u64,
    /// Interval between updates of the gas price limit (used by GasAdjuster) in seconds.
    pub update_interval: u64,
    /// Interval between adding the Ethereum node gas price to the GasAdjuster in seconds.
    pub sample_interval: u64,
    /// Scale factor for gas price limit (used by GasAdjuster).
    pub scale_factor: f64,
}

impl GasLimit {
    /// Converts `self.update_interval` into `Duration`.
    pub fn update_interval(&self) -> Duration {
        Duration::from_secs(self.update_interval)
    }

    /// Converts `self.sample_interval` into `Duration`.
    pub fn sample_interval(&self) -> Duration {
        Duration::from_secs(self.sample_interval)
    }
}
