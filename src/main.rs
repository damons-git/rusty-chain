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
mod log;

use lazy_static::lazy_static;

lazy_static! {
    static ref LOGFILE: String = format!("{}-logfile.txt", util::get_datetime());
}

fn main() {
    // Node entrypoint
    // TODO: Add command line arg control
    let mine = true;
    let txs  = false;
    let rest = false;
    let spawn = true;

    // Start node service
    chain::start_server(mine, txs, rest, spawn);
}