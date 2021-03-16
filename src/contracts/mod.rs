use ethabi::Contract;
use std::fs;
use std::io;
use std::str::FromStr;

// TODO:
const FLUIDEX_CONTRACT_FILE: &str = "fluidex.json";

fn read_file_to_json_value(path: &str) -> io::Result<serde_json::Value> {	
    let fluidex_home = std::env::var("FLUIDEX_HOME").unwrap_or_else(|_| ".".into());
    let path = std::path::Path::new(&fluidex_home).join(path);
    let contents = fs::read_to_string(path)?;
    let val = serde_json::Value::from_str(&contents)?;
    Ok(val)
}

pub fn fluidex_contract() -> Contract {
    let abi_string = read_file_to_json_value(FLUIDEX_CONTRACT_FILE)
        .expect("couldn't read FLUIDEX_CONTRACT_FILE")
        .get("abi")
        .expect("couldn't get abi from FLUIDEX_CONTRACT_FILE")
        .to_string();
    Contract::load(abi_string.as_bytes()).expect("fluidex contract abi")
}