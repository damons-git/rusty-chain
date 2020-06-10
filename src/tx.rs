extern crate ring;
extern crate rand;

use crate::tx_struct::{DataTx, FinanceTx, ContractTx, TxType};
use crate::wallets::create_wallet;
use rand::{RngCore};


// Generate and return a random transaction id.
pub fn generate_rand_id() -> [u8; 4] {
    let mut rng = rand::thread_rng();
    let mut id: [u8; 4] = [0; 4];
    rng.fill_bytes(&mut id);

    return id;
}

//
fn generate_rand_data_tx() -> DataTx {
    let version = 0x00;
    let id = generate_rand_id();
    let tx_type = TxType::Data as u8;
    let owner = vec![0];
    let data: Vec<u8> = String::from("Hello World").into_bytes();
    let reward = 0x00000010;
    let sig = [0; 256];

    return DataTx {
        version: 0x01,
        id: id,
        tx_type: tx_type,
        owner: owner,
        data: data,
        reward: reward,
        signature: sig
    }
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