extern crate ring;
extern crate rand;
extern crate sha2;

use crate::tx_struct::{DataTx, FinancialTx, TxType};
use crate::wallets::{create_keyfile, load_keyfile};
use crate::env::DEBUG;
use std::convert::TryInto;
use rand::{Rng, RngCore};
use rand::distributions::{Alphanumeric};
use sha2::{Sha256, Digest};


// Generate a random unsigned data transaction.
pub fn generate_rand_data_tx() -> DataTx {
    let wallet = load_keyfile(create_keyfile());
    let version = 0x01;
    let tx_type = TxType::Data as u8;
    let owner = wallet.public_key;
    let data: Vec<u8> = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(10)
        .collect::<String>().into_bytes();
    let reward = [0, 0, 0, 255];
    let prev_hash = [0; 32];
    let hash = generate_data_tx_hash(&version, &tx_type, &owner, &reward, &prev_hash);
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
    if DEBUG { println!("Generated Random Data Tx: {}", tx.to_string())};

    return tx
}

// Generate a random unsigned financial transaction.
// TODO:
pub fn generate_random_fin_tx() -> FinancialTx {
    unimplemented!();
}

// Construct a SHA256 hash of a data transaction from occupied tx fields.
// The fields used to create the hashed binary are: version | tx_type | owner | reward | prev_hash
pub fn generate_data_tx_hash(version: &u8, tx_type: &u8, owner: &Vec<u8>, reward: &[u8; 4], prev_hash: &[u8; 32]) -> [u8; 32] {
    let mut binary: Vec<u8> = vec![];
    binary.push(*version);
    binary.push(*tx_type);
    binary.extend_from_slice(&owner.clone());
    binary.extend_from_slice(&reward.clone());
    binary.extend_from_slice(&prev_hash.clone());

    return hash(binary);
}

// Construct a SHA256 hash of a financial transaction from occupied tx fields.
// The fields used to create the hashed binary are:
// TODO:
pub fn generate_fin_tx_hash(version: &u8, tx_type: &u8, owner: &Vec<u8>, reward: &[u8; 4], prev_hash: &[u8; 32]) -> [u8; 32] {
    unimplemented!();
}

// Perform SHA256 hash of arbitrary binary data.
pub fn hash(binary: Vec<u8>) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(binary);
    let result = hasher.finalize();

    let mut hash: [u8; 32] = Default::default();
    hash.copy_from_slice(&result[0..32]);
    return hash;
}

// Convert a transaction struct into a signable binary blob.
pub fn tx_to_binary(tx: DataTx) -> Vec<u8> {
    let mut binary: Vec<u8> = vec![];
    binary.push(tx.version);
    binary.push(tx.tx_type);
    binary.extend_from_slice(&tx.owner.clone());
    binary.extend_from_slice(&tx.data.clone());
    binary.extend_from_slice(&tx.reward.clone());
    binary.extend_from_slice(&tx.previous_hash.clone());
    binary.extend_from_slice(&tx.hash.clone());

    return binary;
}


// Generate and random 32-bit value.
pub fn generate_rand_id() -> [u8; 4] {
    let mut rng = rand::thread_rng();
    let mut id: [u8; 4] = [0; 4];
    rng.fill_bytes(&mut id);

    return id;
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn generate_rand_id_test() {
        let mut id: [u8; 4] = [0, 0, 0, 0];
        id = generate_rand_id();

        assert_eq!(id.len(), 4);
        assert_ne!(id, [0, 0, 0,0]);
    }

}