use std::process::Command;
use std::fs::{File, write, create_dir, read};
use std::io::{Error, ErrorKind};
use ring::{rand, signature};

// Create a PEM formatted RSA pub-priv key pair
// using openssl genpkey
pub fn create_wallet() {
    let output = Command::new("openssl")
        .arg("genpkey")
        .arg("-algorithm")
        .arg("RSA")
        .arg("-pkeyopt")
        .arg("rsa_keygen_bits:2048")
        .arg("-pkeyopt")
        .arg("rsa_keygen_pubexp:65537")
        .arg("-outform")
        .arg("DER")
        .output()
        .expect("Failed to execute process");

    let key_data = if output.status.success() { &output.stdout } else { &output.stderr };
    match output.status.success() {
        false => panic!("Failed running process to generate wallet."),
        true => ()
    }
    save_wallet("wallet/wallet.der", key_data);
}

// Save the wallet to the file system.
fn save_wallet(file_name: &str, data: &Vec<u8>) -> bool {
    match create_dir("wallet/") {
        Err(why) => if Error::last_os_error().kind() != ErrorKind::AlreadyExists {
                panic!("Failed to create wallet directory: {}", why);
            },
        Ok(_) => ()
    };

    match File::create(file_name) {
        Err(why) => panic!("Failed to create wallet file: {}", why),
        Ok(file) => file
    };

    match write(file_name, data) {
        Err(why) => panic!("Failed to write to created wallet file: {}", why),
        Ok(_) => ()
    };

    return true;
}

// Load a key file stored on disk into memory.
pub fn load_wallet() {
    let key_der = match read("wallet/wallet.der") {
        Err(why) => panic!("Failed to read the contents of the key file: {}", why),
        Ok(contents) => contents
    };

    let key_pair = signature::RsaKeyPair::from_der(&key_der);
    println!("{:?}", key_pair);
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn stub() {

    }

}