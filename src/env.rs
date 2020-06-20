// Initial peer(s) list on join
// To add a new address increase array index and add to list.
pub const PEERS_LIST: [&str; 3] = [
    "127.0.0.1:55845",
    "0:0:0:0:0:0:0:1:55845",
    "2001:0db8:85a3:0000:0000:8a2e:0370:7334:55845"
];

pub const DEBUG: bool = true;                       // Flag to output debugging data to stdout
pub const SPAWN_CHAIN: bool = true;                 // Flag to state if node should start network at genesis block
pub const MINING_NODE: bool = true;                 // Flag to state if node should mine blocks
pub const MINING_THREADS: u8 = 1;                   // The number of mining threads to be used
pub const DEFAULT_PORT: u16 = 55845;                // Default port: chosen after the atomic weight of Iron (Fe) 55.845

pub const GENESIS_DIFF: u8 = 10;                    // Starting difficulty for the genesis block
pub const BLOCK_TIME: u32 = 120;                    // Expected block time in seconds

pub const KEY_ALGO: &str = "RSA";                   // Asymmetric encryption key protocol used for Wallets
pub const KEY_SIZE: u32 = 2048;                     // Key-size in bits
pub const KEY_PUB_EXP: u32 = 65537;                 // Public exponenent used for key generation

pub const MINER_PROCESS: u8 = 8;                    // Number of mining threads to be used