mod tx_struct;
mod block_struct;
mod wallets;

fn main() {
    wallets::create_wallet();
    wallets::load_wallet();
}
