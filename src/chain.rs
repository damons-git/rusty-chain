use crate::env;
use crate::block_struct;
use crate::tx_struct;
use crate::util;
use crate::miner::{start_mining_server};
use std::thread;
use std::sync::mpsc;


pub fn start_server(mine: bool, accept_tx: bool, host_rest_api: bool) {
    println!("Starting node..");

    for addr in env::PEERS_LIST.iter() {
        println!("{}", util::parse_net_address(addr));
    }

    let (tx, rx) = mpsc::channel();
    start_mining_server(tx, 10);

    loop {

    }
}