use crate::contracts::fluidex_contract;
use crate::eth_client::ethereum_gateway::EthereumGateway;
use crate::types::{AddTokenOp, PriorityOp, H160};
use ethabi::Hash;
use std::convert::TryFrom;
use std::fmt::Debug;
// use std::time::Instant;
use web3::types::{BlockNumber, FilterBuilder, Log};

struct ContractTopics {
    new_token: Hash,
    new_priority_request: Hash,
}

impl ContractTopics {
    fn new(fluidex_contract: &ethabi::Contract) -> Self {
        Self {
            new_token: fluidex_contract
                .event("NewToken")
                .expect("main contract NewToken abi error")
                .signature(),
            new_priority_request: fluidex_contract
                .event("NewPriorityRequest")
                .expect("main contract NewPriorityRequest abi error")
                .signature(),
        }
    }
}

#[async_trait::async_trait]
pub trait EthClient {
    async fn get_new_token_events(&self, from: BlockNumber, to: BlockNumber) -> anyhow::Result<Vec<AddTokenOp>>;
    async fn get_priority_op_events(&self, from: BlockNumber, to: BlockNumber) -> anyhow::Result<Vec<PriorityOp>>;
    async fn block_number(&self) -> anyhow::Result<u64>;
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
            .filter_map(|event| match T::try_from(event) {
                Ok(ev) => Some(Ok(ev)),
                Err(e) => {
                    log::error!("{:?}", e);
                    None
                }
            })
            .collect()
    }
}

#[async_trait::async_trait]
impl EthClient for EthHttpClient {
    async fn get_new_token_events(&self, from: BlockNumber, to: BlockNumber) -> anyhow::Result<Vec<AddTokenOp>> {
        // let start = Instant::now();

        let result = self.get_events(from, to, vec![self.topics.new_token]).await;
        // metrics::histogram!("eth_watcher.get_new_token_events", start.elapsed());
        result
    }

    async fn get_priority_op_events(&self, from: BlockNumber, to: BlockNumber) -> anyhow::Result<Vec<PriorityOp>> {
        // let start = Instant::now();

        let result = self.get_events(from, to, vec![self.topics.new_priority_request]).await;
        // metrics::histogram!("eth_watcher.get_priority_op_events", start.elapsed());
        result
    }

    async fn block_number(&self) -> anyhow::Result<u64> {
        Ok(self.client.block_number().await?.as_u64())
    }
}
