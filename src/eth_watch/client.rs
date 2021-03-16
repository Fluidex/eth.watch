use crate::eth_client::ethereum_gateway::EthereumGateway;

pub struct EthHttpClient {
    client: EthereumGateway,
    fluidex_contract_addr: String, // TODO: H160
}

impl EthHttpClient {
    // pub fn new(client: EthereumGateway, zksync_contract_addr: H160) -> Self {
    //     let topics = ContractTopics::new(&zksync_contract());
    //     Self {
    //         client,
    //         topics,
    //         zksync_contract_addr,
    //     }
    // }

    pub fn new(client: EthereumGateway, fluidex_contract_addr: String) -> Self {
        Self {
            client,
            fluidex_contract_addr,
        }
    }
}
