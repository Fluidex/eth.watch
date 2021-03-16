use serde::{Deserialize, Serialize};
use std::str::FromStr;

pub use crate::config::configs::{
	ContractsConfig
};

pub mod configs;

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(default)]
pub struct Settings {
    pub contracts: ContractsConfig,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            contracts: ContractsConfig::default(),
        }
    }
}
