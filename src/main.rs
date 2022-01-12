use chrono::prelude::*;
use std::collections::BTreeMap;
use std::env;
use std::fs::File;
use std::io::BufReader;
use web3::contract::{Contract, Options};
use web3::helpers as w3h;
use web3::types::{BlockId, BlockNumber, TransactionId, H160, U256, U64};

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let file = File::open("src/signatures.json").unwrap();
    let reader = BufReader::new(file);
    let code_sig_lookup: BTreeMap<String, Vec<String>> = serde_json::from_reader(reader).unwrap();

    let websocket = web3::transports::WebSocket::new(&env::var("INFURA_MAIN").unwrap())
        .await
        .unwrap();
    let web3s = web3::Web3::new(websocket);

    let latest_block = web3s
        .eth()
        .block(BlockId::Number(BlockNumber::Latest))
        .await
        .unwrap()
        .unwrap();

    let timestamp = latest_block.timestamp.as_u64() as i64;
    let naive = NaiveDateTime::from_timestamp(timestamp, 0);
    let utc_dt: DateTime<Utc> = DateTime::from_utc(naive, Utc);

    println!(
        "[{}] block num {}, parent {}, transactions: {}, gas used {}, gas limit {}, base fee {}, difficulty {}, total difficulty {}",
        utc_dt.format("%Y-%m-%d %H:%M:%S"),
        latest_block.number.unwrap(),
        latest_block.parent_hash,
        latest_block.transactions.len(),
        latest_block.gas_used,
        latest_block.gas_limit,
        latest_block.base_fee_per_gas.unwrap(),
        latest_block.difficulty,
        latest_block.total_difficulty.unwrap()
    );

    for transaction_hash in latest_block.transactions {
        let tx = match web3s
            .eth()
            .transaction(TransactionId::Hash(transaction_hash))
            .await
        {
            Ok(Some(tx)) => tx,
            _ => {
                println!("An error occurred.");
                continue;
            }
        };

        let smart_contract_addr = match tx.to {
            Some(addr) => match web3s.eth().code(addr, None).await {
                Ok(code) => {
                    if code == web3::types::Bytes::from([]) {
                        println!("Empty code, skipping.");
                        continue;
                    } else {
                        println!("Non empty code, returning address.");
                        addr
                    }
                }
                _ => {
                    println!("Unable to retrieve code, skipping.");
                    continue;
                }
            },
            _ => {
                println!("To address is not a valid address, skipping.");
                continue;
            }
        };

        let smart_contract = match Contract::from_json(
            web3s.eth(),
            smart_contract_addr,
            include_bytes!("erc20_abi.json"),
        ) {
            Ok(contract) => contract,
            _ => {
                println!("Failed to init contract, skipping.");
                continue;
            }
        };

        let token_name: String = match smart_contract
            .query("name", (), None, Options::default(), None)
            .await
        {
            Ok(result) => result,
            _ => {
                println!("Could not get name, skipping.");
                continue;
            }
        };

        let input_str: String = w3h::to_string(&tx.input);
        if input_str.len() < 12 {
            continue;
        }
        let func_code = input_str[3..11].to_string();
        let func_signature: String = match code_sig_lookup.get(&func_code) {
            Some(func_sig) => format!("{:?}", func_sig),
            _ => {
                println!("Function not found.");
                "[unknown]".to_string()
            }
        };

        let from_addr = tx.from.unwrap_or(H160::zero());
        let to_addr = tx.to.unwrap_or(H160::zero());

        let eth_value = wei_to_eth(tx.value);
        println!(
            "[{}] ({} -> {}) from {}, to {}, value {}, gas {}, gas price {}",
            tx.transaction_index.unwrap_or(U64::from(0)),
            &token_name,
            &func_signature,
            w3h::to_string(&from_addr),
            w3h::to_string(&to_addr),
            eth_value,
            tx.gas,
            tx.gas_price,
        );
    }
}

fn wei_to_eth(wei_val: U256) -> f64 {
    let res = wei_val.as_u128() as f64;
    let res = res / 1_000_000_000_000_000_000.0;
    res
}
