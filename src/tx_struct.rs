// Enum containing transaction type(s).
pub enum TxType {
    Data     = 0x00,
    Financial  = 0x01
}

/**
 * Data Transaction:
 * A transaction struct that allows
 * for 256-bytes of arbitrary data.
 */
pub struct DataTx {
    pub version: u8,                // u8 field for tx version
    pub tx_type: u8,                // 8-bit transaction type field
    pub owner: Vec<u8>,             // Public key of wallet making transaction (270 bytes - ASN.1 Public Key Format)
    pub data: Vec<u8>,              // 256-byte arbitrary data field
    pub reward: [u8; 4],            // u32 amount of tokens for mining reward (optional)
    pub previous_hash: [u8; 32],    // 32-byte field for previous tx hash from owner wallet
    pub hash: [u8; 32],             // 32-byte field for unique transaction hash
    pub signature: [u8; 256]        // 256-byte owner RSA signature field
}

impl ToString for DataTx {
    fn to_string(&self) -> String {
        return format!("
            DataTx {{
                \tversion: {:x?},
                \ttx_type: {:x?},
                \towner: {:x?},
                \tdata: {:x?},
                \treward: {:x?},
                \tprevious_hash: {:x?},
                \thash: {:x?},
                \tsignature: {:?},
            }}
        ",
        self.version,
        self.tx_type,
        self.owner,
        self.data,
        self.reward,
        self.previous_hash,
        self.hash,
        &self.signature[..]);
    }
}

/**
 * Finance Transaction:
 * A transaction struct that allows for
 * proposed financial transfer between
 * two wallets within the network.
 */
pub struct FinancialTx {
    pub version: u8,                // u8 field for tx version
    pub tx_type: u8,                // 8-bit transaction type field
    pub owner: Vec<u8>,             // 32-byte (256-bit) creator wallet reference
    pub receiver: Vec<u8>,          // 32-byte (256-bit) receiver wallet reference
    pub quantity: u32,              // u32 amount of tokens to be transfered
    pub reward: u32,                // u32 amount of tokens for mining reward
    pub previous_hash: [u8; 32],    // 32-byte field for previous tx hash from owner wallet
    pub hash: [u8; 32],             // 32-byte field for unique transaction hash
    pub signature: [u8; 256]        // 256-byte owner RSA signature field
}

impl ToString for FinancialTx {
    fn to_string(&self) -> String {
        return format!("
            FinancialTx {{
                \tversion: {:x?},
                \ttx_type: {:x?},
                \towner: {:x?},
                \treceiver: {:x?},
                \tquantity: {:x?},
                \treward: {:x?},
                \tprevious_hash: {:x?},
                \thash: {:x?},
                \tsignature: {:?},
            }}
        ",
        self.version,
        self.tx_type,
        self.owner,
        self.receiver,
        self.quantity,
        self.reward,
        self.previous_hash,
        self.hash, &self.signature[..]);
    }
}



#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn construct_data_tx() {
        let version: u8 = 0x01;
        let tx_type: u8 = TxType::Data as u8;
        let owner: Vec<u8> = vec![0, 0, 0, 0];
        let data: Vec<u8> = String::from("Hello World!").into_bytes();
        let reward: [u8; 4] = [0, 0, 0, 1];
        let previous_hash: [u8; 32] = [0; 32];
        let hash: [u8; 32] = [0; 32];
        let signature: [u8; 256] = [0x00; 256];

        let tx: DataTx = DataTx {
            version: version,
            tx_type: tx_type,
            owner: owner.clone(),
            data: data.clone(),
            reward: reward,
            previous_hash: previous_hash,
            hash: hash,
            signature: signature
        };

        assert_eq!(tx.version, version);
        assert_eq!(tx.tx_type, tx_type);
        assert_eq!(tx.owner, owner);
        assert_eq!(tx.data, data);
        assert_eq!(tx.reward, reward);
        assert_eq!(tx.previous_hash, previous_hash);
        assert_eq!(tx.hash, hash);
        assert!(tx.signature.iter().eq(signature.iter()));
    }
}