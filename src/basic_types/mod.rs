//! The declaration of the most primitive types used in Fluidex network.
//! Most of them are just re-exported from the `web3` crate.

#[macro_use]
mod macros;

use serde::{Deserialize, Serialize};
use std::fmt;
use std::num::ParseIntError;
use std::ops::{Add, Deref, DerefMut, Sub};
use std::str::FromStr;

pub use web3::types::{Address, Log, TransactionReceipt, H160, H256, U128, U256};

pub type L2Pubkey = web3::types::H256;

basic_type!(
    /// Unique identifier of the token in the Fluidex network.
    TokenId,
    u16
);

basic_type!(
    /// Unique identifier of the account in the Fluidex network.
    AccountId,
    u32 // TODO: u16?
);

basic_type!(
    /// Fluidex network block sequential index.
    BlockNumber,
    u32
);

basic_type!(
    /// Fluidex account nonce.
    Nonce,
    u32
);

basic_type!(
    /// Unique identifier of the priority operation in the Fluidex network.
    PriorityOpId,
    u64
);

basic_type!(
    /// Block number in the Ethereum network.
    EthBlockId,
    u64
);
