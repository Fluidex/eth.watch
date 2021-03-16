use serde::{Deserialize, Serialize};
use std::str::FromStr;

pub use crate::config::configs::{
	ContractsConfig
};

pub mod configs;

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Settings {
    pub contracts: ContractsConfig,
}
