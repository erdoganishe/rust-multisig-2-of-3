use bdk::descriptor;
use generate_test::generate;
//use generate_test_3::generate_desc;
use generate_wallet::generate_wallets;
use script_generation::{generate_script, get_keys};

// mod generate_test_2;

//mod generate_test_3;

mod generate_wallet;
mod script_generation;
mod generate_test;
fn main() {
    // let _ = generate_wallets();
    // let keys = get_keys();
    generate_script();
   
}
