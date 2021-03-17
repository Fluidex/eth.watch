//! Ethereum watcher polls the Ethereum node for new events
//! such as PriorityQueue events or NewToken events.
//! New events are accepted to the fluidex network once they have the sufficient amount of confirmations.
//!
//! Poll interval is configured using the `ETH_POLL_INTERVAL` constant.
//! Number of confirmations is configured using the `CONFIRMATIONS_FOR_ETH_EVENT` environment variable.

// Built-in deps
use std::{
    collections::HashMap,
    time::{Duration, Instant},
};

// External uses
use futures::{
    channel::{mpsc, oneshot},
    SinkExt, StreamExt,
};

use tokio::{task::JoinHandle, time};
use web3::types::{Address, BlockNumber};

// Local deps
use self::{
    client::EthClient,
    eth_state::ETHState
    received_ops::{sift_outdated_ops, ReceivedPriorityOp},
};

pub use client::EthHttpClient;

use crate::eth_client::ethereum_gateway::EthereumGateway;

mod client;
mod eth_state;
mod received_ops;

/// As `infura` may limit the requests, upon error we need to wait for a while
/// before repeating the request.
const RATE_LIMIT_DELAY: Duration = Duration::from_secs(30);

/// Ethereum Watcher operating mode.
///
/// Normally Ethereum watcher will always poll the Ethereum node upon request,
/// but unfortunately `infura` may decline requests if they are produced too
/// often. Thus, upon receiving the order to limit amount of request, Ethereum
/// watcher goes into "backoff" mode in which polling is disabled for a
/// certain amount of time.
#[derive(Debug)]
enum WatcherMode {
    /// ETHWatcher operates normally.
    Working,
    /// Polling is currently disabled.
    Backoff(Instant),
}

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

    /// Atomically replaces the stored Ethereum state.
    fn set_new_state(&mut self, new_state: ETHState) {
        self.eth_state = new_state;
    }

    // TODO:
    async fn poll_eth_node(&mut self) -> anyhow::Result<()> {
        // let start = Instant::now();
        // let last_block_number = self.client.block_number().await?;

        // if last_block_number > self.eth_state.last_ethereum_block() {
        //     self.process_new_blocks(last_block_number).await?;
        // }

        // metrics::histogram!("eth_watcher.poll_eth_node", start.elapsed());
        Ok(())
    }

    // TODO try to move it to eth client
    fn is_backoff_requested(&self, error: &anyhow::Error) -> bool {
        error.to_string().contains("429 Too Many Requests")
    }

    fn enter_backoff_mode(&mut self) {
        let backoff_until = Instant::now() + RATE_LIMIT_DELAY;
        self.mode = WatcherMode::Backoff(backoff_until);
        // This is needed to track how much time is spent in backoff mode
        // and trigger grafana alerts
        // TODO:
        // metrics::histogram!("eth_watcher.enter_backoff_mode", RATE_LIMIT_DELAY);
    }

    fn polling_allowed(&mut self) -> bool {
        match self.mode {
            WatcherMode::Working => true,
            WatcherMode::Backoff(delay_until) => {
                if Instant::now() >= delay_until {
                    log::info!("Exiting the backoff mode");
                    self.mode = WatcherMode::Working;
                    true
                } else {
                    // We have to wait more until backoff is disabled.
                    false
                }
            }
        }
    }

    // TODO:
    pub async fn run(mut self, mut eth_watch_req: mpsc::Receiver<EthWatchRequest>) {
        // As infura may be not responsive, we want to retry the query until we've actually got the
        // block number.
        // Normally, however, this loop is not expected to last more than one iteration.
        let block = loop {
            let block = self.client.block_number().await;

            match block {
                Ok(block) => {
                    break block;
                }
                Err(error) => {
                    log::warn!(
                        "Unable to fetch last block number: '{}'. Retrying again in {} seconds",
                        error,
                        RATE_LIMIT_DELAY.as_secs()
                    );

                    time::delay_for(RATE_LIMIT_DELAY).await;
                }
            }
        };

        // Code above is prepared for the possible rate limiting by `infura`, and will wait until we
        // can interact with the node again. We're not expecting the rate limiting to be applied
        // immediately after that, thus any error on this stage is considered critical and
        // irrecoverable.
        self.restore_state_from_eth(block)
            .await
            .expect("Unable to restore ETHWatcher state");

        while let Some(request) = eth_watch_req.next().await {
            match request {
                EthWatchRequest::PollETHNode => {
                    if !self.polling_allowed() {
                        // Polling is currently disabled, skip it.
                        continue;
                    }

                    let poll_result = self.poll_eth_node().await;

                    if let Err(error) = poll_result {
                        if self.is_backoff_requested(&error) {
                            log::warn!(
                                "Rate limit was reached, as reported by Ethereum node. \
                                Entering the backoff mode"
                            );
                            self.enter_backoff_mode();
                        } else {
                            // Some unexpected kind of error, we won't shutdown the node because of it,
                            // but rather expect node administrators to handle the situation.
                            log::error!("Failed to process new blocks {}", error);
                        }
                    }
                }
                EthWatchRequest::GetPriorityQueueOps {
                    op_start_id,
                    max_chunks,
                    resp,
                } => {
                    resp.send(self.get_priority_requests(op_start_id, max_chunks)).unwrap_or_default();
                }
                EthWatchRequest::GetUnconfirmedDeposits { address, resp } => {
                    let deposits_for_address = self.get_ongoing_deposits_for(address);
                    resp.send(deposits_for_address).ok();
                }
                EthWatchRequest::GetUnconfirmedOps { address, resp } => {
                    let deposits_for_address = self.get_ongoing_ops_for(address);
                    resp.send(deposits_for_address).ok();
                }
                EthWatchRequest::GetUnconfirmedOpByHash { eth_hash, resp } => {
                    let unconfirmed_op = self.find_ongoing_op_by_hash(&eth_hash);
                    resp.send(unconfirmed_op).unwrap_or_default();
                }
                EthWatchRequest::IsPubkeyChangeAuthorized {
                    address,
                    nonce,
                    pubkey_hash,
                    resp,
                } => {
                    let authorized = self
                        .is_new_pubkey_hash_authorized(address, nonce, &pubkey_hash)
                        .await
                        .unwrap_or(false);
                    resp.send(authorized).unwrap_or_default();
                }
            }
        }
    }
}
