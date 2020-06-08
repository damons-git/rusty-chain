extern crate ring;
extern crate untrusted;

use std::process::Command;
use std::fs::{File, write, create_dir, read};
use std::io::{Error, ErrorKind};
use ring::{rand, signature};


pub fn create_wallet() -> () {
    let key_data: Vec<u8> = create_keyfile();
    save_keyfile("wallet/priv.der", &key_data);
}

// Create a DER formatted RSA pub-priv key pair
// using openssl genpkey
pub fn create_keyfile() -> Vec<u8> {
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

    let key_data = if output.status.success() { output.stdout } else { output.stderr };
    match output.status.success() {
        false => panic!("Failed running process to generate wallet."),
        true => ()
    }

    return key_data;
}

// Save the wallet to the file system.
fn save_keyfile(file_name: &str, data: &Vec<u8>) -> bool {
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
pub fn load_wallet() -> () {
    let priv_der = match read("wallet/priv.der") {
        Err(why) => panic!("Failed to read the contents of the key file: {}", why),
        Ok(contents) => contents
    };

    let output = Command::new("openssl")
        .arg("rsa")
        .arg("-in")
        .arg("wallet/priv.der")
        .arg("-inform")
        .arg("DER")
        .arg("-RSAPublicKey_out")
        .arg("-outform")
        .arg("DER")
        .output()
        .expect("Failed to execute process");
    let pub_der = if output.status.success() { output.stdout } else { output.stderr };

    let key_pair = match signature::RsaKeyPair::from_der(&priv_der) {
        Err(why) => panic!("Failed to parse key file: {}", why),
        Ok(res) => res
    };

    let pub_key = signature::UnparsedPublicKey::new(&signature::RSA_PKCS1_2048_8192_SHA256, pub_der);

    let msg: &[u8] = b"abc";
    let rng = rand::SystemRandom::new();
    let mut sig = vec![0; key_pair.public_modulus_len()];
    match key_pair.sign(&signature::RSA_PKCS1_SHA256, &rng, msg, &mut sig) {
        Err(why) => println!("{}", why),
        Ok(_) => ()
    };

    println!("{:?}", sig);
    // sig[0] = 0;
    let x = match pub_key.verify(&msg, &sig) {
        Err(why) => println!("Why: {}", why),
        Ok(res) => res
    };
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn stub() {

    }

}