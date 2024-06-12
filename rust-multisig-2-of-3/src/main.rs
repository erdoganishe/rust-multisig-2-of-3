use generate_wallet::generate_wallets;
use script_generation::get_keys;
mod generate_wallet;
mod script_generation;

fn main() {
    let _ = generate_wallets();
    get_keys();
}
