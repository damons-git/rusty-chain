use std::thread;
use std::sync::mpsc;
use std::time::Duration;
use std::u128;
use crate::env::{MINER_PROCESS};
use crate::util::{hash};

pub fn start_mining_server(chain_tx: mpsc::Sender<[u8; 4]>, diff: u8) {
    println!("Starting {} miner threads.", MINER_PROCESS);

    let (worker_tx, worker_rx) = mpsc::channel();
    let nonce_range: u128 = u128::MAX / MINER_PROCESS as u128;
    let data = vec![99, 55, 11];
    let diff_mask = parse_diff_to_mask(diff);

    for multiplier in 0..MINER_PROCESS {
        mining_worker(worker_tx.clone(), nonce_range * multiplier as u128, data.clone(), diff_mask.clone());
    }

    loop {
        let available: bool = worker_rx.try_recv().is_err();
        if available {
            println!("Received nonce at mining server: {}", worker_rx.recv().unwrap());
        }
    }
}

fn mining_worker(tx: mpsc::Sender<u128>, mut nonce: u128, binary: Vec<u8>, diff_mask: Vec<u8>) {
    thread::spawn(move || {
        loop {
            let mut data: Vec<u8> = vec![];
            data.extend_from_slice(&nonce.to_be_bytes().to_vec());
            data.extend_from_slice(&binary);

            let hashed = hash(&data);

            let mut temp: Vec<u8> = vec![];
            for i in 0..diff_mask.len() {
                temp.push(hashed[i] | diff_mask[i]);
            }

            if temp == diff_mask {
                println!("Hash {:x?} \n Nonce {}", hashed, nonce);
                tx.send(nonce.clone()).unwrap();
            }
            nonce += 1;
        }
    });
}

fn parse_diff_to_mask(diff: u8) -> Vec<u8> {
    let mut bin_str: Vec<char> = vec![];

    // Append difficulty mask.
    for _ in 0..diff {
        bin_str.push('0');
    }

    // Pad mask to nearest byte multiple.
    for _ in 0..(8 - (diff % 8)) {
        bin_str.push('1');
    }

    let split = &bin_str.chunks(8).map(|chunk| {
        let byte = chunk.iter().collect::<String>();
        u8::from_str_radix(&byte, 2).unwrap()
    }).collect::<Vec<u8>>();

    return split.clone();
}