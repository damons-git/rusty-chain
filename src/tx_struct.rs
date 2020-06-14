use crate::util::{type_of, hash};
use crate::wallet_struct::{Wallet};

// Enum containing transaction type(s).
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TxType {
    Data = 0x00,
    Financial = 0x01
}

// Generalised interface for Tx structs.
pub trait Tx {
    // Convert tx to hashable binary.
    fn to_hashable_bin(&self) -> Vec<u8>;

    // Convert tx to signable binary.
    fn to_signable_bin(&self) -> Vec<u8>;

    // Generate and set hash of transaction.
    fn generate_hash(&mut self) -> ();

    // Generate and set signature of transaction.
    fn generate_signature(&mut self, wallet: &Wallet) -> ();
}

/**
 * Data Transaction:
 * A transaction struct that allows
 * for 256-bytes of arbitrary data.
 */
pub struct DataTx {
    pub version: u8,                // u8 field for tx version
    pub tx_type: TxType,            // 8-bit transaction type field represented as TxType enum
    pub owner: Vec<u8>,             // Public key of wallet making transaction (270 bytes - ASN.1 Public Key Format)
    pub data: Vec<u8>,              // 256-byte arbitrary data field
    pub reward: [u8; 4],            // u32 amount of tokens for mining reward (optional)
    pub previous_hash: [u8; 32],    // 32-byte field for previous tx hash from owner wallet
    pub hash: [u8; 32],             // 32-byte field for unique transaction hash
    pub signature: [u8; 256]        // 256-byte owner RSA signature field
}

impl DataTx {
    pub fn new() -> DataTx {
        return DataTx {
            version: 0x00,
            tx_type: TxType::Data,
            owner: vec![0; 32],
            data: vec![],
            reward: [0, 0, 0, 0],
            previous_hash: [0; 32],
            hash: [0; 32],
            signature: [0; 256]
        }
    }
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
                \tsignature: {:x?},
            }}",
            self.version,
            self.tx_type as u8,
            self.owner,
            self.data,
            self.reward,
            self.previous_hash,
            self.hash,
            &self.signature[..]
        );
    }
}

impl Tx for DataTx {
    // Convert transaction fields into a
    // binary used for generating hash.
    fn to_hashable_bin(&self) -> Vec<u8> {
        let mut binary: Vec<u8> = vec![];
        binary.push(self.version);
        binary.push(self.tx_type as u8);
        binary.extend_from_slice(&self.owner.clone());
        binary.extend_from_slice(&self.data.clone());
        binary.extend_from_slice(&self.reward.clone());
        binary.extend_from_slice(&self.previous_hash.clone());

        return binary;
    }

    // Convert transaction fields into a binary
    // used for signing.
    fn to_signable_bin(&self) -> Vec<u8> {
        let mut binary: Vec<u8> = vec![];
        binary.push(self.version);
        binary.push(self.tx_type as u8);
        binary.extend_from_slice(&self.owner.clone());
        binary.extend_from_slice(&self.data.clone());
        binary.extend_from_slice(&self.reward.clone());
        binary.extend_from_slice(&self.previous_hash.clone());
        binary.extend_from_slice(&self.hash.clone());

        return binary;
    }

    // Generate and set hash of transaction.
    fn generate_hash(&mut self) -> () {
        let bin: Vec<u8> = self.to_hashable_bin();
        let hash: [u8; 32] = hash(&bin);
        self.hash = hash;
    }

    // Generate and set signature of transaction.
    fn generate_signature(&mut self, wallet: &Wallet) -> () {
        let bin: Vec<u8> = self.to_signable_bin();
        let sig: [u8; 256] = wallet.sign(&bin);
        self.signature = sig;
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
    pub tx_type: TxType,            // 8-bit transaction type field represented as TxType enum
    pub owner: Vec<u8>,             // 32-byte (256-bit) creator wallet reference
    pub receiver: Vec<u8>,          // 32-byte (256-bit) receiver wallet reference
    pub quantity: [u8; 4],          // u32 amount of tokens to be transfered
    pub reward: [u8; 4],            // u32 amount of tokens for mining reward
    pub previous_hash: [u8; 32],    // 32-byte field for previous tx hash from owner wallet
    pub hash: [u8; 32],             // 32-byte field for unique transaction hash
    pub signature: [u8; 256]        // 256-byte owner RSA signature field
}

impl FinancialTx {
    pub fn new() -> FinancialTx {
        return FinancialTx {
            version: 0x00,
            tx_type: TxType::Financial,
            owner: vec![0; 32],
            receiver: vec![0; 32],
            quantity: [0, 0, 0, 0],
            reward: [0, 0, 0, 0],
            previous_hash: [0; 32],
            hash: [0; 32],
            signature: [0; 256]
        }
    }
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
                \tsignature: {:x?},
            }}",
            self.version,
            self.tx_type as u8,
            self.owner,
            self.receiver,
            self.quantity,
            self.reward,
            self.previous_hash,
            self.hash, &self.signature[..]
        );
    }
}

impl Tx for FinancialTx {
    // Convert transaction fields into a
    // binary used for generating hash.
    fn to_hashable_bin(&self) -> Vec<u8> {
        let mut binary: Vec<u8> = vec![];
        binary.push(self.version);
        binary.push(self.tx_type as u8);
        binary.extend_from_slice(&self.owner.clone());
        binary.extend_from_slice(&self.receiver.clone());
        binary.extend_from_slice(&self.quantity.clone());
        binary.extend_from_slice(&self.reward.clone());
        binary.extend_from_slice(&self.previous_hash.clone());

        return binary;
    }

    // Convert transaction fields into a binary
    // used for signing.
    fn to_signable_bin(&self) -> Vec<u8> {
        let mut binary: Vec<u8> = vec![];
        binary.push(self.version);
        binary.push(self.tx_type as u8);
        binary.extend_from_slice(&self.owner.clone());
        binary.extend_from_slice(&self.receiver.clone());
        binary.extend_from_slice(&self.quantity.clone());
        binary.extend_from_slice(&self.reward.clone());
        binary.extend_from_slice(&self.previous_hash.clone());
        binary.extend_from_slice(&self.hash.clone());

        return binary;
    }

    // Generate and set hash of transaction.
    fn generate_hash(&mut self) -> () {
        let bin: Vec<u8> = self.to_hashable_bin();
        let hash: [u8; 32] = hash(&bin);
        self.hash = hash;
    }

    // Generate and set signature of transaction.
    fn generate_signature(&mut self, wallet: &Wallet) -> () {
        let bin: Vec<u8> = self.to_signable_bin();
        let sig: [u8; 256] = wallet.sign(&bin);
        self.signature = sig;
    }
}



#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn construct_data_tx() {
        let version: u8 = 0x01;
        let tx_type: TxType = TxType::Data;
        let owner: Vec<u8> = vec![0];
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

    #[test]
    fn construct_financial_tx() {
        let version: u8 = 0x01;
        let tx_type: TxType = TxType::Financial;
        let owner: Vec<u8> = vec![0];
        let receiver: Vec<u8> = vec![0];
        let quantity: [u8; 4] = [0, 0, 0, 1];
        let reward: [u8; 4] = [0, 0, 0, 1];
        let previous_hash: [u8; 32] = [0; 32];
        let hash: [u8; 32] = [0; 32];
        let signature: [u8; 256] = [0x00; 256];

        let tx: FinancialTx = FinancialTx {
            version: version,
            tx_type: tx_type,
            owner: owner.clone(),
            receiver: receiver.clone(),
            quantity: quantity,
            reward: reward,
            previous_hash: previous_hash,
            hash: hash,
            signature: signature
        };

        assert_eq!(tx.version, version);
        assert_eq!(tx.tx_type, tx_type);
        assert_eq!(tx.owner, owner);
        assert_eq!(tx.receiver, receiver);
        assert_eq!(tx.quantity, quantity);
        assert_eq!(tx.reward, reward);
        assert_eq!(tx.previous_hash, previous_hash);
        assert_eq!(tx.hash, hash);
        assert!(tx.signature.iter().eq(signature.iter()));
    }

    #[test]
    fn print_data_tx() {
        let tx: DataTx = DataTx::new();
        assert!(type_of(&tx.to_string()) == "alloc::string::String");
    }

    #[test]
    fn print_financial_tx() {
        let tx: FinancialTx = FinancialTx::new();
        assert!(type_of(&tx.to_string()) == "alloc::string::String");
    }

    #[test]
    fn hash_data_tx() {
        let mut tx: DataTx = DataTx::new();
        tx.generate_hash();
        let expected = [130, 252, 253, 82, 21, 23, 93, 169, 230, 92, 167, 196, 251, 146, 122, 31, 176, 230, 31, 9, 213, 73, 135, 195, 104, 232, 225, 110, 189, 156, 41, 105];

        assert_eq!(tx.hash, expected);
    }

    #[test]
    fn hash_financial_tx() {
        let mut tx: FinancialTx = FinancialTx::new();
        tx.generate_hash();
        let expected = [45, 101, 99, 194, 67, 155, 231, 68, 251, 192, 152, 146, 214, 0, 139, 246, 169, 98, 110, 198, 96, 181, 206, 117, 124, 213, 61, 85, 41, 238, 117, 250];

        assert_eq!(tx.hash, expected);
    }

    #[test]
    fn sign_verify_data_tx() {
        let mut tx: DataTx = DataTx::new();
        let wallet = Wallet::new();
        let binary = tx.to_signable_bin();
        tx.generate_signature(&wallet);

        assert!(Wallet::verify(&wallet.public_key, &binary, &tx.signature));
    }

    #[test]
    fn sign_verify_financial_tx() {
        let mut tx: FinancialTx = FinancialTx::new();
        let wallet = Wallet::new();
        let binary = tx.to_signable_bin();
        tx.generate_signature(&wallet);

        assert!(Wallet::verify(&wallet.public_key, &binary, &tx.signature));
    }
}