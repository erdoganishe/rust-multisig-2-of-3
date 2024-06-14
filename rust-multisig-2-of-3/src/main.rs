use bdk::descriptor;
use generate_key_pair::save_keypair;

use wallet_creation::{create_wallet, get_private_keys};
use wsh_descriptor_generation::{generate_descriptor, get_public_keys};
mod generate_key_pair;
mod wallet_creation;
mod wsh_descriptor_generation;

fn main() {
    // save_keypair(3);
    // generate_descriptor();
    create_wallet();
}
