use crate::contracts::fluidex_contract;
use crate::eth_client::ethereum_gateway::EthereumGateway;
use crate::types::{Address, Nonce, PriorityOp, H160, U256};
use anyhow::format_err;
use ethabi::Hash;
use std::fmt::Debug;
use std::{convert::TryFrom, time::Instant};
use web3::{
    contract::Options,
    types::{BlockNumber, FilterBuilder, Log},
};

struct ContractTopics {
    new_token: Hash,
    // new_trading_pair: Hash,
    new_priority_request: Hash,
}

impl ContractTopics {
    fn new(fluidex_contract: &ethabi::Contract) -> Self {
        Self {
            new_token: fluidex_contract
                .event("NewToken")
                .expect("main contract NewToken abi error")
                .signature(),
            // new_trading_pair: fluidex_contract
            //     .event("NewTradingPair")
            //     .expect("main contract NewTradingPair abi error")
            //     .signature(),
            new_priority_request: fluidex_contract
                .event("NewPriorityRequest")
                .expect("main contract NewPriorityRequest abi error")
                .signature(),
        }
    }
}

#[async_trait::async_trait]
pub trait EthClient {
    // TODO: fix result type
    async fn get_new_token_events(&self, from: BlockNumber, to: BlockNumber) -> anyhow::Result<Vec<PriorityOp>>;
    // // TODO: fix result type
    // async fn get_new_trading_pair_events(&self, from: BlockNumber, to: BlockNumber) -> anyhow::Result<Vec<PriorityOp>>;
    async fn get_priority_op_events(&self, from: BlockNumber, to: BlockNumber) -> anyhow::Result<Vec<PriorityOp>>;
    async fn block_number(&self) -> anyhow::Result<u64>;
    async fn get_auth_fact(&self, address: Address, nonce: Nonce) -> anyhow::Result<Vec<u8>>;
    async fn get_auth_fact_reset_time(&self, address: Address, nonce: Nonce) -> anyhow::Result<u64>;
}

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

    async fn get_events<T>(&self, from: BlockNumber, to: BlockNumber, topics: Vec<Hash>) -> anyhow::Result<Vec<T>>
    where
        T: TryFrom<Log>,
        T::Error: Debug,
    {
        let filter = FilterBuilder::default()
            .address(vec![self.fluidex_contract_addr])
            .from_block(from)
            .to_block(to)
            .topics(Some(topics), None, None, None)
            .build();

        self.client
            .logs(filter)
            .await?
            .into_iter()
            .filter_map(|event| {
                if let Ok(event) = T::try_from(event) {
                    Some(Ok(event))
                } else {
                    None
                }
                // TODO: remove after update
                // .map_err(|e| format_err!("Failed to parse event log from ETH: {:?}", e))
            })
            .collect()
    }
}

#[async_trait::async_trait]
impl EthClient for EthHttpClient {
    // TODO: fix result type
    async fn get_new_token_events(&self, from: BlockNumber, to: BlockNumber) -> anyhow::Result<Vec<PriorityOp>> {
        let start = Instant::now();

        let result = self.get_events(from, to, vec![self.topics.new_token]).await;
        // metrics::histogram!("eth_watcher.get_new_token_events", start.elapsed());
        result
    }

    // // TODO: fix result type
    // async fn get_new_trading_pair_events(&self, from: BlockNumber, to: BlockNumber) -> anyhow::Result<Vec<PriorityOp>> {
    //     let start = Instant::now();

    //     let result = self.get_events(from, to, vec![self.topics.new_trading_pair]).await;
    //     // metrics::histogram!("eth_watcher.get_new_trading_pair_events", start.elapsed());
    //     result
    // }

    async fn get_priority_op_events(&self, from: BlockNumber, to: BlockNumber) -> anyhow::Result<Vec<PriorityOp>> {
        let start = Instant::now();

        let result = self.get_events(from, to, vec![self.topics.new_priority_request]).await;
        // metrics::histogram!("eth_watcher.get_priority_op_events", start.elapsed());
        result
    }

    async fn block_number(&self) -> anyhow::Result<u64> {
        Ok(self.client.block_number().await?.as_u64())
    }

    async fn get_auth_fact(&self, address: Address, nonce: Nonce) -> anyhow::Result<Vec<u8>> {
        self.client
            .call_main_contract_function("authFacts", (address, u64::from(*nonce)), None, Options::default(), None)
            .await
            .map_err(|e| format_err!("Failed to query contract authFacts: {}", e))
    }

    async fn get_auth_fact_reset_time(&self, address: Address, nonce: Nonce) -> anyhow::Result<u64> {
        self.client
            .call_main_contract_function("authFactsResetTimer", (address, u64::from(*nonce)), None, Options::default(), None)
            .await
            .map_err(|e| format_err!("Failed to query contract authFacts: {}", e))
            .map(|res: U256| res.as_u64())
    }
}
