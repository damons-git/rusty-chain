use crate::env;
use crate::block_struct;
use crate::tx_struct;
use crate::util;
use crate::miner::{start_mining_server};
use crate::env::{DEBUG};
use crate::log::{log, tlog, dlog};
use std::thread;
use std::sync::mpsc;
use std::net::SocketAddr;


pub fn start_server(mine_flag: bool, accept_tx_flag: bool, rest_api_flag: bool, spawn_chain_flag: bool) {
    let peers: Vec<SocketAddr> = env::PEERS_LIST.iter().map(|peer| util::parse_net_address(peer)).collect::<Vec<SocketAddr>>();

    tlog("Starting Rusty-chain node!", &[
        format!("Default peers: {:?}", peers),
        format!("Spawning chain: {}", spawn_chain_flag),
        format!("Mining node: {}", mine_flag),
        format!("Accepting txs: {}", accept_tx_flag),
        format!("Hosting REST API: {}", rest_api_flag)
    ]);

    let (tx, rx) = mpsc::channel();
    start_mining_server(tx, 18);

    loop {


    }
}