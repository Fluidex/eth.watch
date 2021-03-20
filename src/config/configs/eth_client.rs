use serde::Deserialize;

/// Configuration for the Ethereum gateways.
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct ETHClientConfig {
    /// Numeric identifier of the L1 network (e.g. `9` for localhost).
    pub chain_id: u8,
    /// How much do we want to increase gas price provided by the network?
    /// Normally it's 1, we use the network-provided price (and limit it with the gas adjuster in eth sender).
    /// However, it can be increased to speed up the transaction mining time.
    pub gas_price_factor: f64,
    /// Address of the Ethereum node API.
    pub web3_url: Vec<String>,
}

impl ETHClientConfig {
    /// Get first web3 url, useful in direct web3 clients, which don't need any multiplexers
    pub fn web3_url(&self) -> String {
        self.web3_url.first().cloned().expect("Should be at least one")
    }
}
