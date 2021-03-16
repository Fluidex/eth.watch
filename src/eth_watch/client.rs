use std::fmt::Debug;
use std::{convert::TryFrom, time::Instant};

use anyhow::format_err;
use ethabi::Hash;
use web3::{
    contract::Options,
    types::{BlockNumber, FilterBuilder, Log},
};

use crate::contracts::fluidex_contract;
use crate::eth_client::ethereum_gateway::EthereumGateway;
use crate::types::{Address, H160, U256};

struct ContractTopics {
    new_priority_request: Hash,
}

impl ContractTopics {
    fn new(fluidex_contract: &ethabi::Contract) -> Self {
        Self {
            new_priority_request: fluidex_contract
                .event("NewPriorityRequest")
                .expect("main contract abi error")
                .signature(),
        }
    }
}

#[async_trait::async_trait]
pub trait EthClient {}

pub struct EthHttpClient {
    client: EthereumGateway,
    topics: ContractTopics,
    fluidex_contract_addr: H160,
}

impl EthHttpClient {
    pub fn new(client: EthereumGateway, fluidex_contract_addr: H160) -> Self {
        let topics = ContractTopics::new(&fluidex_contract());
        Self {
            client,
            topics,
            fluidex_contract_addr,
        }
    }

    // TODO: other functions
}

// TODO:
#[async_trait::async_trait]
impl EthClient for EthHttpClient {}
