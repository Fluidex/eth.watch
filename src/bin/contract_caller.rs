extern crate web3;

use web3::contract::{Contract, Options};
use web3::types::Address;

fn main() {
    let http = web3::transports::Http::new("https://mainnet.infura.io").unwrap();
    let web3 = web3::Web3::new(http);

    // The contract address.
    let address: Address = "0x...".parse().unwrap();

    // Access the contract
    let contract = Contract::from_json(web3.eth(), address, include_bytes!("./abi.json")).unwrap();

    // Query the contract instance
    // And this is where I'm stuck!
    let result = contract.query("decimals", None, None, web3::contract::Options::default(), None);
}