extern crate byteorder;

use crate::util::hash;
use crate::tx_struct::{Tx, DataTx, FinancialTx};
use byteorder::ByteOrder;


/**
 * A struct defining a block within the chain.
 * Note: Txs field has to be 'boxed' as Tx type size unknown at compile time.
 */
pub struct Block<'a> {
    pub nonce: [u8; 16],
    pub previous_hash: [u8; 32],
    pub version: u8,
    pub difficulty: u8,
    pub height: [u8; 4],
    pub timestamp: [u8; 8],
    pub miner: Vec<u8>,
    pub reward_amount: [u8; 4],
    pub tx_count: u8,
    // pub tx_merkle: [u8; 32],
    pub txs: Vec<&'a Tx>,
    pub hash: [u8; 32]
}

impl ToString for Block<'_> {
    fn to_string(&self) -> String {
        return format!("Block {{
            nonce: {:?},
            previous_hash (hex): {:x?},
            version: {},
            difficulty: {},
            height: {:?},
            timestamp: {:?},
            miner (hex): {:x?},
            reward_amount: {:?},
            tx_count: {:?},
            txs: [..],
            hash (hex): {:x?},
        }}",
            self.nonce,
            self.previous_hash,
            self.version,
            self.difficulty,
            self.height,
            self.timestamp,
            self.miner,
            self.reward_amount,
            self.tx_count,
            // self.txs,
            self.hash
        );
    }
}

impl Block<'_> {
    // Convert block fields into a binary used for generating hash.
    // Excludes nonce as used as Proof-of-work to meet set difficulty.
    pub fn to_hashable_bin(&self) -> Vec<u8> {

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

    // Generate and set hash of block.
    pub fn generate_hash(&mut self) -> () {
        let bin: Vec<u8> = self.to_hashable_bin();
        let hash: [u8; 32] = hash(&bin);
        self.hash = hash;
    }
}

