extern crate ring;
extern crate rand;

use rand::{RngCore};

/**
 * Data Transaction:
 * A transaction struct that allows
 * for 256-bytes of arbitrary data.
 */
struct DataTx {
    version: u8,                // u8 field for tx version
    id: [u8; 4],                // 32-bit field for unique transaction id
    tx_type: u8,                // 8-bit transaction type field
    owner: Vec<char>,           // 32-byte (256-bit) creator wallet reference
    data: Vec<char>,            // 256-byte arbitrary data field
    reward: u32,                // u32 amount of tokens for mining reward
    signature: [u8; 256]        // 256-byte owner RSA signature field
}

/**
 * Finance Transaction:
 * A transaction struct that allows for
 * proposed financial transfer between
 * two wallets within the network.
 */
struct FinanceTx {
    version: u8,                // u8 field for tx version
    id: [u8; 4],                // 32-bit field for unique transaction id
    tx_type: u8,                // 8-bit transaction type field
    owner: Vec<char>,           // 32-byte (256-bit) creator wallet reference
    receiver: Vec<char>,        // 32-byte (256-bit) receiver wallet reference
    quantity: u32,              // u32 amount of tokens to be transfered
    reward: u32,                // u32 amount of tokens for mining reward
    signature: [u8; 256]        // 256-byte owner RSA signature field
}

/**
 * Contract Transaction:
 * A transaction struct that allows for
 * a digital contract to be creaated
 * and signed between two parties.
 */
struct ContractTx {
    version: u8,                // u8 field for tx version
    id: [u8; 4],                // 32-bit field for unique transaction id
    tx_type: u8,                // 8-bit transaction type field
    state: u8,                  // 8-bit contract state type
    proposer: Vec<char>,        // 32-byte (256-bit) recevier wallet reference
    seconder: Vec<char>,        // 32-byte (256-bit) seconder wallet reference
    contract: Vec<char>,        // 1024-bytes detailing the contract
    reward: u32,                // u32 amount of tokens for mining reward
    proposal_sig: [u8; 256],    // 256-byte proposer RSA signature field (creation state)
    seconded_sig: [u8; 256],    // 256-byte seconder RSA signature field (seconded state)
    accepted_sig: [u8; 256]     // 256-byte proposer RSA signature field (accepted state)
}

// Enum containing transaction type(s).
enum TxType {
    Data,
    Finance,
    Contract
}

// Resolve tx type to associated binary value.
fn resolve_tx_type(tx_type: TxType) -> u8 {
    match tx_type {
        TxType::Data => 0x00,
        TxType::Finance => 0x01,
        TxType::Contract => 0x02
    }
}

// Enum containing contract transaction states.
enum ContractState {
    Proposed,
    Seconded,
    Accepted
}

// Resolve a contract state enum to associated binary value.
fn resolve_contract_state(state: ContractState) -> u8 {
    match state {
        ContractState::Proposed => 0x00,
        ContractState::Seconded => 0x01,
        ContractState::Accepted => 0x02
    }
}

// Generate and return a random transaction id.
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
    fn resolve_data_tx_enum() {
        let tx_type: TxType = TxType::Data;
        let value: u8 = resolve_tx_type(tx_type);
        assert_eq!(0, value);
    }


    #[test]
    fn resolve_finance_tx_enum() {
        let tx_type: TxType = TxType::Finance;
        let value: u8 = resolve_tx_type(tx_type);
        assert_eq!(1, value);
    }

    #[test]
    fn resolve_contract_tx_enum() {
        let tx_type: TxType = TxType::Contract;
        let value: u8 = resolve_tx_type(tx_type);
        assert_eq!(2, value);
    }

    #[test]
    fn resolve_contract_state_proposed_enum() {
        let state: ContractState = ContractState::Proposed;
        let value: u8 = resolve_contract_state(state);
        assert_eq!(0, value);
    }

    #[test]
    fn resolve_contract_state_seconded_enum() {
        let state: ContractState = ContractState::Seconded;
        let value: u8 = resolve_contract_state(state);
        assert_eq!(1, value);
    }

    #[test]
    fn resolve_contract_state_accepted_enum() {
        let state: ContractState = ContractState::Accepted;
        let value: u8 = resolve_contract_state(state);
        assert_eq!(2, value);
    }

    #[test]
    fn construct_data_tx() {
        let version: u8 = 0x01;
        let id: [u8; 4] = [0, 0, 0, 0];
        let tx_type: u8 = resolve_tx_type(TxType::Data);
        let owner: Vec<char> = "7f83b1657ff1fc53b92dc18148a1d65dfc2d4b1fa3d677284addd200126d9069".chars().collect();
        let data: Vec<char> = "Hello World!".chars().collect();
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
        let tx_type: u8 = resolve_tx_type(TxType::Finance);
        let owner: Vec<char> = "7f83b1657ff1fc53b92dc18148a1d65dfc2d4b1fa3d677284addd200126d9069".chars().collect();
        let receiver: Vec<char> = "7f83b1657ff1fc53b92dc18148a1d65dfc2d4b1fa3d677284addd200126d9069".chars().collect();
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
        let tx_type: u8 = resolve_tx_type(TxType::Contract);
        let state: u8 = resolve_contract_state(ContractState::Proposed);
        let proposer: Vec<char> = "7f83b1657ff1fc53b92dc18148a1d65dfc2d4b1fa3d677284addd200126d9069".chars().collect();
        let seconder: Vec<char> = "7f83b1657ff1fc53b92dc18148a1d65dfc2d4b1fa3d677284addd200126d9069".chars().collect();
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