use crate::config;
use crate::types::U256;

const min_abi: &str = r#"[
                      {
                        "name":"symbol",
                        "inputs":[],
                        "outputs":[{"name":"","type":"string"}],
                        "type":"function",
                        "constant":true
                      },
                      {
                        "name":"name",
                        "inputs":[],
                        "outputs":[{"name":"","type":"string"}],
                        "type":"function",
                        "constant":true
                      },
                      {
                        "name":"decimals",
                        "inputs":[],
                        "outputs":[{"name":"","type":"uint8"}],
                        "type":"function",
                        "constant":true
                      },
                      {
                        "name":"totalSupply",
                        "inputs":[],
                        "outputs":[{"name":"","type":"uint256"}],
                        "type":"function",
                        "constant":true
                      }
                    ]"#;

#[derive(Debug, Clone)]
pub struct TokenInquirer {
    pub web3: web3::Web3<web3::transports::Http>,
}

#[derive(Debug, Clone)]
pub struct TokenInfo {
    pub symbol: String,
    pub name: String,
    pub decimals: u8,
    pub total_supply: U256,
}

impl TokenInquirer {
    pub fn from_config(config: &config::Settings) -> Self {
        let transport = web3::transports::Http::new(&config.eth_client.web3_url()).unwrap();
        let web3 = web3::Web3::new(transport);
        Self { web3: web3 }
    }
}
