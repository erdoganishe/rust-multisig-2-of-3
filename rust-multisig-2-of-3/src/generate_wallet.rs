// use bdk::{
//     bitcoin::Network,
//     database::MemoryDatabase,
//     wallet::{AddressIndex, Wallet},
//     KeychainKind,
// };
// use std::fs::File;
// use std::io::Write;

// pub fn generate_wallets() -> Result<(), bdk::Error> {
//     let network = Network::Testnet;

//     let external_descriptor1 = "wpkh(tprv8ZgxMBicQKsPdy6LMhUtFHAgpocR8GC6QmwMSFpZs7h6Eziw3SpThFfczTDh5rW2krkqffa11UpX3XkeTTB2FvzZKWXqPY54Y6Rq4AQ5R8L/84'/0'/0'/0/*)";
//     let internal_descriptor1 = "wpkh(tprv8ZgxMBicQKsPdy6LMhUtFHAgpocR8GC6QmwMSFpZs7h6Eziw3SpThFfczTDh5rW2krkqffa11UpX3XkeTTB2FvzZKWXqPY54Y6Rq4AQ5R8L/84'/0'/0'/1/*)";
//     let wallet1: Wallet<MemoryDatabase> = Wallet::new(
//         external_descriptor1,
//         Some(internal_descriptor1),
//         network,
//         MemoryDatabase::new(),
//     )?;

//     let external_descriptor2 = "wpkh(tprv8ZgxMBicQKsPdy6LMhUtFHAgpocR8GC6QmwMSFpZs7h6Eziw3SpThFfczTDh5rW2krkqffa11UpX3XkeTTB2FvzZKWXqPY54Y6Rq4AQ5R8L/84'/0'/1'/0/*)";
//     let internal_descriptor2 = "wpkh(tprv8ZgxMBicQKsPdy6LMhUtFHAgpocR8GC6QmwMSFpZs7h6Eziw3SpThFfczTDh5rW2krkqffa11UpX3XkeTTB2FvzZKWXqPY54Y6Rq4AQ5R8L/84'/0'/1'/1/*)";
//     let wallet2: Wallet<MemoryDatabase> = Wallet::new(
//         external_descriptor2,
//         Some(internal_descriptor2),
//         network,
//         MemoryDatabase::new(),
//     )?;

//     let external_descriptor3 = "wpkh(tprv8ZgxMBicQKsPdy6LMhUtFHAgpocR8GC6QmwMSFpZs7h6Eziw3SpThFfczTDh5rW2krkqffa11UpX3XkeTTB2FvzZKWXqPY54Y6Rq4AQ5R8L/84'/0'/2'/0/*)";
//     let internal_descriptor3 = "wpkh(tprv8ZgxMBicQKsPdy6LMhUtFHAgpocR8GC6QmwMSFpZs7h6Eziw3SpThFfczTDh5rW2krkqffa11UpX3XkeTTB2FvzZKWXqPY54Y6Rq4AQ5R8L/84'/0'/2'/1/*)";
//     let wallet3: Wallet<MemoryDatabase> = Wallet::new(
//         external_descriptor3,
//         Some(internal_descriptor3),
//         network,
//         MemoryDatabase::new(),
//     )?;

//     let public_descriptor1 = wallet1.public_descriptor(KeychainKind::External)?.unwrap();
//     let public_descriptor2 = wallet2.public_descriptor(KeychainKind::External)?.unwrap();
//     let public_descriptor3 = wallet3.public_descriptor(KeychainKind::External)?.unwrap();

//     let mut file = File::create("public_keys.txt").expect("Error with file creation");

//     writeln!(file, "{:?}", public_descriptor1);
//     writeln!(file, "{:?}", public_descriptor2);
//     writeln!(file, "{:?}", public_descriptor3);

//     Ok(())
// }
