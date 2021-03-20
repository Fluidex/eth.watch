use serde::Deserialize;
use std::time::Duration;

/// Configuration for the Ethereum sender crate.
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct ETHWatchConfig {
    /// Amount of confirmations for the priority operation to be processed.
    /// In production this should be a non-zero value because of block reverts.
    pub confirmations_for_eth_event: u64,
    /// How often we want to poll the Ethereum node.
    /// Value in milliseconds.
    pub eth_node_poll_interval: u64,
}

impl ETHWatchConfig {
    /// Converts `self.eth_node_poll_interval` into `Duration`.
    pub fn poll_interval(&self) -> Duration {
        Duration::from_millis(self.eth_node_poll_interval)
    }
}
