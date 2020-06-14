extern crate ring;

use crate::env::{KEY_ALGO, KEY_SIZE, KEY_PUB_EXP};
use crate::key_parser;
use std::process;
use std::process::Command;
use ring::{rand, signature};

#[derive(Debug, Copy, Clone, PartialEq)]
enum SignatureState {
    SignatureInvalid,
    SignatureValid
}

/**
 * A struct to hold a public and private key pair.
 */
pub struct Wallet {
    pub public_key: Vec<u8>,
    pub private_key: Vec<u8>
}

impl Wallet {
    // Create a new 2048-bit RSA wallet.
    pub fn new() -> Wallet {
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
            false => {
                panic!("Unable to generate wallet.");
                process::exit(1);
            }
            true => ()
        }

        let public = key_parser::get_public_der(&key_data);
        let private = key_data.clone();

        return Wallet {
            public_key: public,
            private_key: private
        }
    }

    // Load an already generated wallet.
    pub fn load(key_data: Vec<u8>) -> Wallet {
        let public = key_parser::get_public_der(&key_data);
        let private = key_data.clone();

        return Wallet {
            public_key: public,
            private_key: private
        }
    }

    // Sign arbitrary binary data using wallet private key.
    // Note: Wallet holds ASN.1 DER encoded RSA key pair, defined by RSA foundation.
    pub fn sign(&self, data: &Vec<u8>) -> [u8; 256] {

        let public = signature::UnparsedPublicKey::new(&signature::RSA_PKCS1_2048_8192_SHA256, &self.public_key);
        let private = match signature::RsaKeyPair::from_der(&self.private_key) {
            Err(why) => panic!("Failed to parse key file: {}", why),
            Ok(res) => res
        };

        let binary: &[u8] = &data;
        let rng = rand::SystemRandom::new();
        let mut sig_vec = vec![0; private.public_modulus_len()];

        match private.sign(&signature::RSA_PKCS1_SHA256, &rng, binary, &mut sig_vec) {
            Err(why) => println!("{}", why),
            Ok(_) => ()
        };

        let mut sig: [u8; 256] = [0; 256];
        sig.copy_from_slice(&sig_vec[0..256]);
        return sig;
    }

    // Verify signature against binary data and public key.
    pub fn verify(public_signer: &Vec<u8>, data: &Vec<u8>, signature: &[u8; 256]) -> bool {
        let public_key = signature::UnparsedPublicKey::new(&signature::RSA_PKCS1_2048_8192_SHA256, public_signer);
        let verification_res = public_key.verify(&data, signature)
            .map(|_| SignatureState::SignatureValid)
            .map_err(|_| SignatureState::SignatureInvalid);

        let signature_valid = match verification_res {
            Err(_) => false,
            Ok(_) => true
        };

        return signature_valid;
    }
}
