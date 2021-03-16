use serde::{Deserialize, Serialize};
use std::str::FromStr;

pub mod configs;

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(default)]
pub struct Settings {
    pub contract_addr: String,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            contract_addr: Default::default(),
        }
    }
}
