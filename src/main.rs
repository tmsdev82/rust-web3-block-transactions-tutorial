use chrono::prelude::*;
use std::env;
use web3::types::{BlockId, BlockNumber};

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

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
}
