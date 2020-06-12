mod env;
mod tx_struct;
mod block_struct;
mod tx;
mod wallets;
mod key_parser;


fn main() {
    // println!("{:x?}", &bin);

    let tx = tx::generate_rand_data_tx();
    let bin = tx::tx_to_binary(tx);
    let key_data = wallets::create_keyfile();
    wallets::save_keyfile("wallet/keypair.der", &key_data);
    let wallet = wallets::load_keyfile(key_data);
    let sig: Vec<u8> = wallets::sign_binary_data(&wallet, &bin);
    let valid: bool = wallets::verify_binary_data(&wallet.public_key, &bin, &sig);
    println!("Signed/Verified: {}", valid);
}
