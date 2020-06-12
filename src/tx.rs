extern crate ring;
extern crate rand;

use crate::tx_struct::{DataTx, FinanceTx, ContractTx, TxType};
use crate::wallets::{create_keyfile, load_keyfile};
use rand::{Rng, RngCore};
use rand::distributions::{Alphanumeric};


// Generate and return a random transaction id.
pub fn generate_rand_id() -> [u8; 4] {
    let mut rng = rand::thread_rng();
    let mut id: [u8; 4] = [0; 4];
    rng.fill_bytes(&mut id);

    return id;
}

// Generate a random unsigned data transaction.
pub fn generate_rand_data_tx() -> DataTx {
    let version = 0x01;
    let id = generate_rand_id();
    let tx_type = TxType::Data as u8;
    let owner = vec![0];
    let reward = [0, 0, 0, 1];
    let sig = [0; 256];
    let data: Vec<u8> = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(10)
        .collect::<String>().into_bytes();

    println!("ver:    {:x?}", version);
    println!("id:     {:x?}", id);
    println!("type:   {:x?}", tx_type);
    println!("owner:  {:x?}", owner);
    println!("data:   {:x?}", data);
    println!("reward: {:x?}", reward);

    return DataTx {
        version: version,
        id: id,
        tx_type: tx_type,
        owner: owner,
        data: data,
        reward: reward,
        signature: sig
    }
}

pub fn tx_to_binary(tx: DataTx) -> Vec<u8> {
    let mut binary: Vec<u8> = vec![];
    binary.push(tx.version);
    binary.extend_from_slice(&tx.id.clone());
    binary.push(tx.tx_type);
    binary.extend_from_slice(&tx.owner.clone());
    binary.extend_from_slice(&tx.data.clone());
    binary.extend_from_slice(&tx.reward.clone());

    return binary;
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