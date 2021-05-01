use web3::{
    contract::{Contract, Options},
    types::U256,
};

#[tokio::main]
async fn main() -> web3::contract::Result<()> {
    let _ = env_logger::try_init();
    let http = web3::transports::Http::new("http://localhost:8545")?;
    let web3 = web3::Web3::new(http);

    // Accessing existing contract
    let contract_address = "0xd028d24f16a8893bd078259d413372ac01580769".parse().unwrap();
    let contract = Contract::from_json(
        web3.eth(),
        contract_address,
        include_bytes!("./abi.json"),
    )?;

    let result = contract.query("balanceOf", None, None, Options::default(), None);
    let balance_of: U256 = result.await?;
    assert_eq!(balance_of, 1_000_000.into());

    Ok(())
}
