use std::time::Duration;

use futures::{channel::mpsc, SinkExt};
use tokio::{runtime::Runtime, time};

use eth_watcher::config;

fn main() {
    let mut main_runtime = Runtime::new().expect("main runtime start");

    dotenv::dotenv().ok();
    env_logger::init();
    log::info!("ETH watcher started");

    let mut conf = config_rs::Config::new();
    let config_file = dotenv::var("CONFIG_FILE").unwrap();
    conf.merge(config_rs::File::with_name(&config_file)).unwrap();
    let mut settings: config::Settings = conf.try_into().unwrap();
    // log::info!("{:?}", settings);

    // let config = FluidexConfig::from_env();
    // let client = EthereumGateway::from_config(&config);

    // let (eth_req_sender, eth_req_receiver) = mpsc::channel(256);

    // let eth_client = EthHttpClient::new(client, config.contracts.contract_addr);
    // let watcher = EthWatch::new(eth_client, 0);

    // main_runtime.spawn(watcher.run(eth_req_receiver));
    // main_runtime.block_on(async move {
    //     let mut timer = time::interval(Duration::from_secs(1));

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
