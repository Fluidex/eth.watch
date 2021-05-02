use eth_watcher::config;
use eth_watcher::contracts::fluidex_contract;
use eth_watcher::eth_client::{ETHDirectClient, EthereumGateway};
use eth_watcher::eth_signer::PrivateKeySigner;
use eth_watcher::eth_watch::{EthHttpClient, EthWatch, EthWatchRequest};
use futures::{channel::mpsc, SinkExt};
use std::time::Duration;
use tokio::{runtime::Runtime, time};

fn main() {
    let mut main_runtime = Runtime::new().expect("main runtime start");

    dotenv::dotenv().ok();
    env_logger::init();
    log::info!("ETH watcher started");

    let mut conf = config_rs::Config::new();
    let config_file = dotenv::var("CONFIG_FILE").unwrap();
    conf.merge(config_rs::File::with_name(&config_file)).unwrap();
    let settings: config::Settings = conf.try_into().unwrap();
    log::debug!("{:?}", settings);

    let transport = web3::transports::Http::new(&settings.eth_client.web3_url()).unwrap();

    let min_abi = r#"[
                      {
                        "constant":true,
                        "inputs":[{"name":"_owner","type":"address"}],
                        "name":"balanceOf",
                        "outputs":[{"name":"balance","type":"uint256"}],
                        "type":"function"
                      },
                      {
                        "constant":true,
                        "inputs":[],
                        "name":"decimals",
                        "outputs":[{"name":"","type":"uint8"}],
                        "type":"function"
                      }
                    ]"#;
    let contract_addr: web3::types::Address = "0x...".parse().unwrap();
    let q = ETHDirectClient::new(
        transport,
        ethabi::Contract::load(min_abi.as_bytes()).unwrap(),
        settings.eth_sender.sender.operator_commit_eth_addr,
        PrivateKeySigner::new(settings.eth_sender.sender.operator_private_key),
        contract_addr,
        settings.eth_client.chain_id,
        settings.eth_client.gas_price_factor,
    );

    // q.call_contract_function(
    //     "allowance",
    //     (self.sender_account, self.contract_addr),
    //     None,
    //     Options::default(),
    //     None,
    //     token_address,
    //     erc20_abi,
    // )
    // .await;

    // let base = EthereumGateway::from_config(&settings);

    // main_runtime.spawn(watcher.run(eth_req_receiver));
    // let poll_interval = settings.eth_watch.poll_interval();
    // main_runtime.block_on(async move {
    //     let mut timer = time::interval(poll_interval);

    //     loop {
    //         timer.tick().await;
    //         eth_req_sender
    //             .clone()
    //             .send(EthWatchRequest::PollETHNode)
    //             .await
    //             .expect("ETH watch receiver dropped");
    //     }
    // });
}
