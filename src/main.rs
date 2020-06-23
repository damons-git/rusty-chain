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
use crate::tx_struct::{Tx, DataTx, TxType};
use crate::wallet_struct::Wallet;

lazy_static! {
    static ref LOGFILE: String = format!("{}-logfile.txt", util::get_datetime());
}

fn main() {
    // Node entrypoint
    // TODO: Add command line arg control
    let mine_chain = true;
    let accept_txs  = false;
    let host_rest = false;
    let spawn_chain = true;

    // Start node service
    chain::start_server(mine_chain, accept_txs, host_rest, spawn_chain);

    loop { }
}