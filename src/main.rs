mod env;
mod util;
mod tx_struct;
mod block_struct;
mod tx;
mod wallet_struct;
mod key_parser;
mod wallet;

use crate::tx_struct::{DataTx, FinancialTx, TxType, Tx};
use crate::wallet_struct::{Wallet};
use crate::wallet::{load_from_disk, save_to_disk};

fn main() {
    let mut tx = tx::generate_rand_data_tx();
    let wallet = Wallet::new();
    tx.generate_hash();
    tx.generate_signature(&wallet);
    let valid: bool = Wallet::verify(&wallet.public_key, &tx.to_signable_bin(), &tx.signature);
    println!("Signed & Verified: {}", valid);
}
