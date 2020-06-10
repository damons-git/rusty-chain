mod env;
mod tx_struct;
mod block_struct;
mod tx;
mod wallets;


fn main() {
    wallets::create_wallet();
    let w: wallets::Wallet = wallets::load_wallet();
    let data: Vec<u8> = vec![1, 2, 3, 4, 5];
    let sig: Vec<u8> = wallets::sign_binary_data(&w, &data);
    let valid: bool = wallets::verify_binary_data(&w.public_key, &data, &sig);

    println!("Signed/Verified: {}", valid);
}
