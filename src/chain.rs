use crate::env;
use crate::block_struct::Block;
use crate::tx_struct::{Tx, DataTx, FinancialTx, TxType};
use crate::util::{parse_net_address, get_timestamp, hash};
use crate::miner::{start_mining_server};
use crate::env::{DEBUG, GENESIS_DIFF};
use crate::log::{log, tlog, dlog};
use crate::wallet_struct::Wallet;
use std::thread;
use std::sync::mpsc;
use std::net::SocketAddr;

// N.B. The lifetime of the state is equal to the shortest lifetime
// of either the previous_block, or tx in stored_txs.
struct State<'a, 'b> {
    previous_block: Block<'a>,
    height: [u8; 4],
    difficulty: u8,
    mining_service: (mpsc::Sender<[u8; 4]>, mpsc::Receiver<[u8; 4]>),
    network_service: (mpsc::Sender<[u8; 4]>, mpsc::Receiver<[u8; 4]>),
    stored_txs: Vec<&'b Tx>
}

pub fn start_server(mine_flag: bool, accept_tx_flag: bool, rest_api_flag: bool, spawn_chain_flag: bool) {
    let peers: Vec<SocketAddr> = env::PEERS_LIST.iter().map(|peer| parse_net_address(peer)).collect::<Vec<SocketAddr>>();

    tlog("Starting Rusty-chain node!", &[
        format!("Default peers: {:?}", peers),
        format!("Spawning chain: {}", spawn_chain_flag),
        format!("Joining chain: {}", !spawn_chain_flag),
        format!("Mining node: {}", mine_flag),
        format!("Accepting txs: {}", accept_tx_flag),
        format!("Hosting REST API: {}", rest_api_flag)
    ]);

    // Create Genesis block if spawn flag set.
    // Creates an independent mining server for genesis block.
    if spawn_chain_flag {
        let gen_wallet = Wallet::new();

        let mut genesis: Block = Block {
            nonce: [0; 16],
            previous_hash: [0; 32],
            version: 0,
            difficulty: GENESIS_DIFF,
            height: [0; 4],
            timestamp: get_timestamp(),
            miner: gen_wallet.public_key,
            reward_amount: [0; 4],
            tx_count: 0,
            txs: vec![],
            hash: [0; 32]
        };

        let gen_bin = genesis.to_hashable_bin();
        let (tx, rx) = mpsc::channel();
        start_mining_server(tx.clone(), 20, gen_bin.clone());
        let (nonce, hashv) = rx.recv().unwrap();
        genesis.nonce = nonce;
        genesis.hash = hashv;
        println!("{}", genesis.to_string());
    }

    // Load services.

    // start_net_interface(server_tx.clone());
    // start_rest_server(server_tx.clone());
    // start_fork_recovery(server_tx.clone());

    loop {}
}