extern crate byteorder;

use crate::tx_struct::{Tx, DataTx, FinancialTx};
use byteorder::ByteOrder;


/**
 * A struct defining a block within the chain.
 * Note: Txs field has to be 'boxed' as Tx type size unknown at compile time.
 */
pub struct Block<'a> {
    nonce: [u8; 16],
    previous_hash: [u8; 32],
    version: u8,
    difficulty: u8,
    height: [u8; 4],
    timestamp: [u8; 8],
    miner: Vec<u8>,
    reward_amount: [u8; 4],
    tx_count: u8,
    // tx_merkle: [u8; 32],
    txs: Vec<&'a Tx>,
    hash: [u8; 32]
}

impl Block<'_> {
    // Convert block fields into a binary used for generating hash.
    // Excludes nonce as used as Proof-of-work to meet set difficulty.
    fn to_hashable_bin(&self) -> Vec<u8> {

        // Generate binary of all txs.
        // TODO: Use merkle tree for static tx hash size.
        let mut txs_bin: Vec<u8> = vec![];
        for tx in self.txs.iter() {
            txs_bin.extend_from_slice(&tx.to_bin().clone());
        }

        let mut binary: Vec<u8> = vec![];
        binary.extend_from_slice(&self.previous_hash.clone());
        binary.push(self.version);
        binary.push(self.difficulty);
        binary.extend_from_slice(&self.height.clone());
        binary.extend_from_slice(&self.timestamp.clone());
        binary.extend_from_slice(&self.miner.clone());
        binary.extend_from_slice(&self.reward_amount.clone());
        binary.push(self.tx_count);
        binary.extend(&txs_bin.clone());

        return binary;
    }
}

