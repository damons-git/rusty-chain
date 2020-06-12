extern crate ring;

use crate::env::{KEY_ALGO, KEY_SIZE, KEY_PUB_EXP};
use crate::key_parser;
use std::process::Command;
use std::fs::{File, write, create_dir, read};
use std::io::{Error, ErrorKind};
use ring::{rand, signature};


pub struct Wallet {
    pub public_key: signature::UnparsedPublicKey<Vec<u8>>,
    pub private_key: signature::RsaKeyPair
}

enum SignatureState {
    SignatureInvalid,
    SignatureValid
}

// Create a DER (Distinguished Encoding Rules) formatted
// RSA public-private key file using openssl genpkey
pub fn create_keyfile() -> Vec<u8> {

    let output = Command::new("openssl")
        .arg("genpkey")
        .arg("-algorithm")
        .arg(format!("{}", KEY_ALGO))
        .arg("-pkeyopt")
        .arg(format!("rsa_keygen_bits:{}", KEY_SIZE))
        .arg("-pkeyopt")
        .arg(format!("rsa_keygen_pubexp:{}", KEY_PUB_EXP))
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

// Save the DER keypair to the file system.
pub fn save_keyfile(file_name: &str, data: &Vec<u8>) -> bool {
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

    match write(file_name, data) {
        Err(why) => panic!("Failed to write to created key file: {}", why),
        Ok(_) => ()
    };

    return true;
}

// Load a key file stored on disk.
pub fn load_keyfile_from_disk() -> Wallet {

    let key_data = match read("wallet/keyfile.der") {
        Err(why) => panic!("Failed to read the contents of the key file: {}", why),
        Ok(contents) => contents
    };

    return load_keyfile(key_data);
}

// Load a raw binary key file and return as a wallet struct.
pub fn load_keyfile(key_data: Vec<u8>) -> Wallet {
    // Note: ASN.1 DER Encoded RSA key pair, defined by RSA foundation.
    let key_pair = match signature::RsaKeyPair::from_der(&key_data) {
        Err(why) => panic!("Failed to parse key file: {}", why),
        Ok(res) => res
    };

    // Note: 270 byte ASN.1 Public Key Encoding, defined by RSA foundation.
    let pub_der = key_parser::get_public_der(&key_data);
    let pub_key = signature::UnparsedPublicKey::new(&signature::RSA_PKCS1_2048_8192_SHA256, pub_der);

    return Wallet {
        public_key: pub_key,
        private_key: key_pair
    };
}

// Produces a signature for for arbitrary binary data using the
// given wallet.
pub fn sign_binary_data(wallet: &Wallet, data: &Vec<u8>) -> Vec<u8> {
    let data_bin: &[u8] = &data;
    let private = &wallet.private_key;
    let rng = rand::SystemRandom::new();
    let mut data_signature = vec![0; private.public_modulus_len()];

    match private.sign(&signature::RSA_PKCS1_SHA256, &rng, data_bin, &mut data_signature) {
        Err(why) => println!("{}", why),
        Ok(_) => ()
    };

    return data_signature;
}

// Verifies the signature of the given data was signed by
// the private key related to the public key provided.
pub fn verify_binary_data(public_key: &signature::UnparsedPublicKey<Vec<u8>>, data: &Vec<u8>, signature: &Vec<u8>) -> bool {
    let verification_res = public_key.verify(&data, &signature)
        .map(|_| SignatureState::SignatureValid)
        .map_err(|_| SignatureState::SignatureInvalid);

    let signature_valid = match verification_res {
        Err(_) => false,
        Ok(_) => true
    };

    return signature_valid;
}



#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn create_keyfile_test() {
        let keyfile_der = create_keyfile();
        let keyfile_len_raw = keyfile_der.len() as u16;
        let slice = &keyfile_der[2..4];
        let keyfile_len_stated = ((slice[0] as u16) << 8) | slice[1] as u16 + 0x04;

        assert!(&keyfile_der[0..2] == [48, 130]);           // Ensure data is ASN.1 DER sequence (0x3082).
        assert_eq!(keyfile_len_raw, keyfile_len_stated);    // Compare raw length to that stated keyfile bytes 3/4 (N.B. +4 for sequence bytes).
    }

}