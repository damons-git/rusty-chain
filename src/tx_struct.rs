// Enum containing transaction type(s).
pub enum TxType {
    Data     = 0x00,
    Finance  = 0x01,
    Contract = 0x02
}

// Enum containing contract transaction states.
pub enum ContractState {
    Proposed = 0x00,
    Seconded = 0x01,
    Accepted = 0x02
}

/**
 * Data Transaction:
 * A transaction struct that allows
 * for 256-bytes of arbitrary data.
 */
pub struct DataTx {
    pub version: u8,                // u8 field for tx version
    pub id: [u8; 4],                // 32-bit field for unique transaction id
    pub tx_type: u8,                // 8-bit transaction type field
    pub owner: Vec<u8>,             // Public key of wallet making transaction (270 bytes - ASN.1 Public Key Format)
    pub data: Vec<u8>,              // 256-byte arbitrary data field
    pub reward: u32,                // u32 amount of tokens for mining reward (optional)
    pub signature: [u8; 256]        // 256-byte owner RSA signature field
}

/**
 * Finance Transaction:
 * A transaction struct that allows for
 * proposed financial transfer between
 * two wallets within the network.
 */
pub struct FinanceTx {
    pub version: u8,                // u8 field for tx version
    pub id: [u8; 4],                // 32-bit field for unique transaction id
    pub tx_type: u8,                // 8-bit transaction type field
    pub owner: Vec<u8>,             // 32-byte (256-bit) creator wallet reference
    pub receiver: Vec<u8>,          // 32-byte (256-bit) receiver wallet reference
    pub quantity: u32,              // u32 amount of tokens to be transfered
    pub reward: u32,                // u32 amount of tokens for mining reward
    pub signature: [u8; 256]        // 256-byte owner RSA signature field
}

/**
 * Contract Transaction:
 * A transaction struct that allows for
 * a digital contract to be creaated
 * and signed between two parties.
 */
pub struct ContractTx {
    pub version: u8,                // u8 field for tx version
    pub id: [u8; 4],                // 32-bit field for unique transaction id
    pub tx_type: u8,                // 8-bit transaction type field
    pub state: u8,                  // 8-bit contract state type
    pub proposer: Vec<u8>,          // 32-byte (256-bit) recevier wallet reference
    pub seconder: Vec<u8>,          // 32-byte (256-bit) seconder wallet reference
    pub contract: Vec<char>,        // 1024-bytes detailing the contract
    pub reward: u32,                // u32 amount of tokens for mining reward
    pub proposal_sig: [u8; 256],    // 256-byte proposer RSA signature field (creation state)
    pub seconded_sig: [u8; 256],    // 256-byte seconder RSA signature field (seconded state)
    pub accepted_sig: [u8; 256]     // 256-byte proposer RSA signature field (accepted state)
}



#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn construct_data_tx() {
        let version: u8 = 0x01;
        let id: [u8; 4] = [0, 0, 0, 0];
        let tx_type: u8 = TxType::Data as u8;
        let owner: Vec<u8> = vec![0, 0, 0, 0];
        let data: Vec<u8> = String::from("Hello World!").into_bytes();
        let reward: u32 = 0x00000010;
        let signature: [u8; 256] = [0x00; 256];

        let tx: DataTx = DataTx {
            version: version,
            id: id,
            tx_type: tx_type,
            owner: owner.clone(),
            data: data.clone(),
            reward: reward,
            signature: signature
        };

        assert_eq!(tx.version, version);
        assert_eq!(tx.id, id);
        assert_eq!(tx.tx_type, tx_type);
        assert_eq!(tx.owner, owner);
        assert_eq!(tx.data, data);
        assert_eq!(tx.reward, reward);
        assert!(tx.signature.iter().eq(signature.iter()));
    }

    #[test]
    fn construct_finance_tx() {
        let version: u8 = 0x01;
        let id: [u8; 4] = [0, 0, 0, 0];
        let tx_type: u8 = TxType::Finance as u8;
        let owner: Vec<u8> = vec![0, 0, 0, 0];
        let receiver: Vec<u8> = vec![0, 0, 0, 0];
        let quantity: u32 = 0x00001234;
        let reward: u32 = 0x00000010;
        let signature: [u8; 256] = [0x00; 256];

        let tx: FinanceTx = FinanceTx {
            version: version,
            id: id,
            tx_type: tx_type,
            owner: owner.clone(),
            receiver: receiver.clone(),
            quantity: quantity,
            reward: reward,
            signature: signature
        };

        assert_eq!(tx.version, version);
        assert_eq!(tx.id, id);
        assert_eq!(tx.tx_type, tx_type);
        assert_eq!(tx.owner, owner);
        assert_eq!(tx.receiver, receiver);
        assert_eq!(tx.quantity, quantity);
        assert_eq!(tx.reward, reward);
        assert!(tx.signature.iter().eq(signature.iter()));
    }

    #[test]
    fn construct_contract_tx() {
        let version: u8 = 0x01;
        let id: [u8; 4] = [0, 0, 0, 0];
        let tx_type: u8 = TxType::Contract as u8;
        let state: u8 = ContractState::Proposed as u8;
        let proposer: Vec<u8> = vec![0, 0, 0, 0];
        let seconder: Vec<u8> = vec![0, 0, 0, 0];
        let contract: Vec<char> = "This is an example contract between X and Y.".chars().collect();
        let reward: u32 = 0x00000010;
        let proposal_sig: [u8; 256] = [0x00; 256];
        let seconded_sig: [u8; 256] = [0x00; 256];
        let accepted_sig: [u8; 256] = [0x00; 256];

        let tx: ContractTx = ContractTx {
            version: version,
            id: id,
            tx_type: tx_type,
            state: state,
            proposer: proposer.clone(),
            seconder: seconder.clone(),
            contract: contract.clone(),
            reward: reward,
            proposal_sig: proposal_sig,
            seconded_sig: seconded_sig,
            accepted_sig: accepted_sig
        };

        assert_eq!(tx.version, version);
        assert_eq!(tx.id, id);
        assert_eq!(tx.tx_type, tx_type);
        assert_eq!(tx.state, state);
        assert_eq!(tx.proposer, proposer);
        assert_eq!(tx.seconder, seconder);
        assert_eq!(tx.contract, contract);
        assert_eq!(tx.reward, reward);
        assert!(tx.proposal_sig.iter().eq(proposal_sig.iter()));
        assert!(tx.seconded_sig.iter().eq(seconded_sig.iter()));
        assert!(tx.accepted_sig.iter().eq(accepted_sig.iter()));

    }
}