extern crate ring;

use crate::wallet_struct::Wallet;
use std::fs::{File, write, create_dir, read};
use std::io::{Error, ErrorKind};


// Save the DER keypair to the file system.
pub fn save_to_disk(file_name: &str, wallet: Wallet) -> bool {
    match create_dir("wallet/") {
        Err(why) => if Error::last_os_error().kind() != ErrorKind::AlreadyExists {
                panic!("Failed to create wallet directory: {}", why);
            },
        Ok(_) => ()
    };
    match File::create(file_name) {
        Err(why) => panic!("Failed to create key file file: {}", why),
        Ok(file) => file
    };
    match write(file_name, wallet.private_key) {
        Err(why) => panic!("Failed to write to created key file: {}", why),
        Ok(_) => ()
    };

    return true;
}

// Load a key file stored on disk.
pub fn load_from_disk() -> Wallet {
    let key_data = match read("wallet/keypair.der") {
        Err(why) => panic!("Failed to read the contents of the key file: {}", why),
        Ok(contents) => contents
    };

    let wallet = Wallet::load(key_data);
    return wallet;
}



#[cfg(test)]
mod test {
    use super::*;
}