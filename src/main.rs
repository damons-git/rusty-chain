mod env;
mod tx_struct;
mod block_struct;
mod tx;
mod wallets;


fn main() {
    let tx = tx::generate_rand_data_tx();
    let bin = tx::tx_to_binary(tx);
    println!("{:x?}", &bin);

    wallets::create_wallet();
    let w: wallets::Wallet = wallets::load_wallet();
    let sig: Vec<u8> = wallets::sign_binary_data(&w, &bin);
    let valid: bool = wallets::verify_binary_data(&w.public_key, &bin, &sig);

    println!("Signed/Verified: {}", valid);
}
