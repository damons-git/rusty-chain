use std::process::Command;
use std::fs::{File, write, create_dir};

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
        .output()
        .expect("Failed to execute process");

    let s = if output.status.success() { String::from_utf8_lossy(&output.stdout) } else { String::from_utf8_lossy(&output.stderr) };

    match output.status.success() {
        false => panic!("Failed running process to generate wallet."),
        true => ()
    }
    save_wallet("wallet/wallet.pem", &s);
}

// Save the wallet to the file system.
fn save_wallet(file_name: &str, data: &str) -> bool {
    match create_dir("wallet/") {
        Err(why) => println!("{}", why),
        Ok(_) => ()
    };

    let mut file = match File::create(file_name) {
        Err(why) => panic!("Couldn't create wallet file: {}", why),
        Ok(file) => file
    };

    match write(file_name, data.as_bytes()) {
        Err(why) => panic!("Couldn't write to created wallet file: {}", why),
        Ok(_) => ()
    };

    return true;
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn stub() {

    }

}