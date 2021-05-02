use crate::config;
use crate::types::U256;

const MIN_ABI: &str = r#"[
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

    pub async fn get_token_info(&self, contract_addr_no_prefix: String) -> Result<TokenInfo, anyhow::Error> {
        let contract_addr: web3::types::Address = contract_addr_no_prefix.parse()?;
        let contract = web3::contract::Contract::from_json(self.web3.eth(), contract_addr, MIN_ABI.as_bytes())?;

        let symbol: String = contract.query("symbol", (), None, web3::contract::Options::default(), None).await?;
        let name: String = contract.query("name", (), None, web3::contract::Options::default(), None).await?;
        let decimals: u8 = contract
            .query("decimals", (), None, web3::contract::Options::default(), None)
            .await?;
        let total_supply: U256 = contract
            .query("totalSupply", (), None, web3::contract::Options::default(), None)
            .await?;

        Ok(TokenInfo {
            symbol,
            name,
            decimals,
            total_supply,
        })
    }
}
