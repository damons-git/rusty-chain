mod env;
mod util;
mod tx_struct;
mod block_struct;
mod tx;
mod wallets;
mod key_parser;

use crate::tx_struct::{DataTx, FinancialTx, TxType, Tx};


fn main() {
    let mut tx = tx::generate_rand_data_tx();
    let key_data = wallets::create_keyfile();
    let wallet = wallets::load_keyfile(key_data);
    tx.generate_hash();
    tx.generate_signature(&wallet);
    let valid: bool = wallets::verify(&wallet.public_key, &tx.to_signable_bin(), &tx.signature);
    println!("Signed & Verified: {}", valid);
}
