extern crate ring;
extern crate rand;
extern crate sha2;

use crate::tx_struct::{DataTx, FinancialTx, TxType};
use crate::wallet_struct::{Wallet};
use crate::env::DEBUG;
use rand::{Rng};
use rand::distributions::{Alphanumeric};


// Generate a random unsigned data transaction.
pub fn generate_rand_data_tx() -> DataTx {
    let wallet = Wallet::new();
    let version = 0x01;
    let tx_type = TxType::Data;
    let owner = wallet.public_key;
    let data: Vec<u8> = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(10)
        .collect::<String>().into_bytes();
    let reward = [0, 0, 0, 255];
    let prev_hash = [0; 32];
    let hash = [0; 32];
    let sig = [0; 256];

    let tx = DataTx {
        version: version,
        tx_type: tx_type,
        owner: owner,
        data: data,
        reward: reward,
        previous_hash: prev_hash,
        hash: hash,
        signature: sig
    };
    if DEBUG { println!("> Generated Random Data Tx: {}", tx.to_string())};

    return tx
}

// Generate a random unsigned financial transaction.
// TODO:
pub fn generate_rand_fin_tx() -> FinancialTx {
    unimplemented!();
}


#[cfg(test)]
mod test {
    use super::*;

}