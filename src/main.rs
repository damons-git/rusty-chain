mod env;
mod util;
mod tx_struct;
mod block_struct;
mod tx;
mod wallet_struct;
mod key_parser;
mod wallet;
mod difficulty;
mod chain;
mod miner;


fn main() {
    // Node entrypoint
    // TODO: Add command line arg control
    let mine = true;
    let txs  = false;
    let rest = false;

    // Start node service
    chain::start_server(mine, txs, rest);
}