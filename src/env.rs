pub const DEBUG: bool = true;               // Debug flag to output debugging data to stdout
pub const KEY_ALGO: &str = "RSA";           // Asymmetric encryption key protocol used for Wallets
pub const KEY_SIZE: u32 = 2048;             // Key-size in bits
pub const KEY_PUB_EXP: u32 = 65537;         // Public exponenent used for key generation
pub const GENESIS_DIFF: u8 = 10;            // Starting difficulty for the genesis block
pub const BLOCK_TIME: u32 = 120;            // Expected block time in seconds