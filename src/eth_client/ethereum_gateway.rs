use std::fmt::Debug;

use web3::contract::tokens::{Detokenize, Tokenize};
use web3::contract::Options;
use web3::types::{Address, BlockId, Filter, Log, U64};

use crate::config;

#[derive(Debug, Clone)]
pub enum EthereumGateway {
    // TODO:
    Mock,
}

impl EthereumGateway {
    pub fn from_config(config: &config::Settings) -> Self {
        // TODO:
        Self::Mock
    }
}
