use bdk::descriptor;
// use generate_test::generate;
//use generate_test_3::generate_desc;
use generate_key_pair::save_keypair;
// use generate_wallet::generate_wallets;
use wallet_creation::{create_wallet, get_private_keys};
use wsh_descriptor_generation::{generate_descriptor, get_public_keys};
// mod generate_test_2;

//mod generate_test_3;

mod generate_key_pair;
mod generate_wallet;
mod wallet_creation;
mod wsh_descriptor_generation;

fn main() {
    // let _ = generate_wallets();
    // let keys = get_keys();
    // let wsh_descriptor = generate_descriptor();
    //save_keypair(3);
    // let keys = get_private_keys("keypairs.txt");
    // println!("{:#?}",keys);
    create_wallet();
}
