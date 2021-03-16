// External uses
use futures::{
    channel::{mpsc, oneshot},
    SinkExt, StreamExt,
};

// Local deps
use self::client::EthClient;

pub use client::EthHttpClient;

mod client;

// TODO:
#[derive(Debug)]
pub enum EthWatchRequest {
    PollETHNode,
}

pub struct EthWatch<W: EthClient> {
    client: W,
    eth_state: ETHState,
    /// All ethereum events are accepted after sufficient confirmations to eliminate risk of block reorg.
    number_of_confirmations_for_event: u64,
    mode: WatcherMode,
}

impl<W: EthClient> EthWatch<W> {
    pub fn new(client: W, number_of_confirmations_for_event: u64) -> Self {
        Self {
            client,
            eth_state: ETHState::default(),
            mode: WatcherMode::Working,
            number_of_confirmations_for_event,
        }
    }

    // TODO:
    pub async fn run(mut self, mut eth_watch_req: mpsc::Receiver<EthWatchRequest>) {}
}
