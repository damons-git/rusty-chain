use std::process::Command;

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
    println!("{:?}", s);
}