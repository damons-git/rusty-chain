use std::thread;
use std::sync::mpsc;
use std::sync::mpsc::{RecvTimeoutError};
use std::time::Duration;
use std::u128;
use crate::env::{MINER_PROCESS, DEBUG};
use crate::util::{hash};
use crate::log::{log, tlog, dlog};

// Commands accepted by mining workers
#[derive(Debug)]
enum MinerCommand {
    KILL
}

pub fn start_mining_server(chain_tx: mpsc::Sender<[u8; 4]>, diff: u8) {

    thread::spawn(move || {
        log("Starting mining service..".to_string());
        let mut miners: Vec<mpsc::Sender<MinerCommand>> = vec![];
        let (worker_tx, worker_rx) = mpsc::channel();
        let nonce_range: u128 = u128::MAX / MINER_PROCESS as u128;
        let data = vec![99, 55, 11];
        let diff_mask = parse_diff_to_mask(diff);

        log(format!("Mining server spawning {} worker thread(s).", MINER_PROCESS));
        for multiplier in 0..MINER_PROCESS {
            let (thread_tx, thread_rx) = mpsc::channel();
            mining_worker(worker_tx.clone(), thread_rx, nonce_range * multiplier as u128, data.clone(), diff_mask.clone());
            miners.push(thread_tx.clone());
        }

        loop {
            let available: bool = worker_rx.try_recv().is_err();
            if available {
                let (nonce, hash) = worker_rx.recv().unwrap();
                dlog(module_path!(), "Valid hash found", &[
                    format!("Hash (hex): {:x?}", hash),
                    format!("Nonce: {}", nonce),
                    format!("Difficulty: {}", diff),
                    format!("Difficulty Mask (hex): {:x?}", diff_mask)
                ]);
                for tx in miners.iter() {
                    tx.send(MinerCommand::KILL).unwrap();
                }
            }
        }
    });
}

// A simple miner worker process.
// Takes a transmitter to talk to managing process, a nonce interval to begin at,
// binary data to be worked on, and a difficulty level to meet.
fn mining_worker(tx: mpsc::Sender<(u128, [u8; 32])>, rx: mpsc::Receiver<MinerCommand>, mut nonce: u128, binary: Vec<u8>, diff_mask: Vec<u8>) {
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
                tx.send((nonce.clone(), hashed)).unwrap();
            }

            nonce += 1;

            let recv = rx.recv_timeout(Duration::new(0, 0));
            match recv {
                Ok(cmnd) => {
                    match cmnd {
                        MinerCommand::KILL => break
                    }
                },
                Err(e) => {
                    match e {
                        RecvTimeoutError::Timeout => (),
                        RecvTimeoutError::Disconnected => break
                    }
                }
            }

        }
        println!("Mining worker process killed.");
    });
}

// Given a difficulty level returns the equivalent Vec<u8>
// mask to apply and check against hash.
fn parse_diff_to_mask(diff: u8) -> Vec<u8> {
    let mut bin_str: Vec<char> = vec![];

    // Append difficulty mask and pad
    // to nearest byte multiple.
    for _ in 0..diff { bin_str.push('0'); }
    for _ in 0..(8 - (diff % 8)) { bin_str.push('1'); }

    // Separate char string into segments of 8 and convert
    // to equivalent u8 binary value.
    let split = &bin_str.chunks(8).map(|chunk| {
        let byte = chunk.iter().collect::<String>();
        u8::from_str_radix(&byte, 2).unwrap()
    }).collect::<Vec<u8>>();

    return split.clone();
}