# Rust Web3 Ethereum block transactions information

This repository shows example code on how to get information from a block on the Ethereum block chain. It also shows an example on how to decipher transactions and get more information out of them. Specifically we get the name of the Token in the transaction if a smart contract was used in the transaction. Then we also look up the name of the function being invoked on the smart contract.

The code is explained on my blog: [Rust Web3 token transactions from blocks: how to](https://tms-dev-blog.com/rust-web3-token-transactions-from-blocks)

## Set up and running

To be able to run this project as is a `.env` file is needed in the root directory. This `.env` file should contain a line containing a valid Ethereum node connection endpoint (from Infura.io) like this: `INFURA_MAIN=wss://mainnet.infura.io/ws/v3/xxxxxx`.

Then the project can be run with `cargo run`.
