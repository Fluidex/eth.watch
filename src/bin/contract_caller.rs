use eth_watcher::config;
use eth_watcher::contracts::fluidex_contract;
use eth_watcher::eth_client::{ETHDirectClient, EthereumGateway};
use eth_watcher::eth_signer::PrivateKeySigner;
use eth_watcher::eth_watch::{EthHttpClient, EthWatch, EthWatchRequest};
use eth_watcher::types::U256;
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
    let web3 = web3::Web3::new(transport);

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
    let contract = web3::contract::Contract::from_json(web3.eth(), contract_addr, min_abi.as_bytes()).unwrap();

    main_runtime.block_on(async move {
        // let sasa: U256 = q
        //     .call_contract_function(
        //         "allowance",
        //         // ("0x...".parse().unwrap(), "0x...".parse().unwrap()),
        //         (),
        //         None,
        //         web3::contract::Options::default(),
        //         None,
        //         contract_addr,
        //         ethabi::Contract::load(min_abi.as_bytes()).unwrap(),
        //     )
        //     .await
        //     .unwrap();
        let result: U256 = contract
            .query("balanceOf", (), None, web3::contract::Options::default(), None)
            .await
            .unwrap();
    });
}
