use bdk::{
    bitcoin::Network,
    database::MemoryDatabase,
    wallet::{AddressIndex, Wallet},
};
use std::fs::File;
use std::io::Write;

pub fn generate_wallets() -> Result<(), bdk::Error> {
    let network = Network::Testnet;

    let external_descriptor1 = "wpkh(tprv8ZgxMBicQKsPdy6LMhUtFHAgpocR8GC6QmwMSFpZs7h6Eziw3SpThFfczTDh5rW2krkqffa11UpX3XkeTTB2FvzZKWXqPY54Y6Rq4AQ5R8L/84'/0'/0'/0/*)";
    let internal_descriptor1 = "wpkh(tprv8ZgxMBicQKsPdy6LMhUtFHAgpocR8GC6QmwMSFpZs7h6Eziw3SpThFfczTDh5rW2krkqffa11UpX3XkeTTB2FvzZKWXqPY54Y6Rq4AQ5R8L/84'/0'/0'/1/*)";
    let wallet1: Wallet<MemoryDatabase> = Wallet::new(
        external_descriptor1,
        Some(internal_descriptor1),
        network,
        MemoryDatabase::new(),
    )?;

    let external_descriptor2 = "wpkh(tprv8ZgxMBicQKsPdy6LMhUtFHAgpocR8GC6QmwMSFpZs7h6Eziw3SpThFfczTDh5rW2krkqffa11UpX3XkeTTB2FvzZKWXqPY54Y6Rq4AQ5R8L/84'/0'/0'/2/*)";
    let internal_descriptor2 = "wpkh(tprv8ZgxMBicQKsPdy6LMhUtFHAgpocR8GC6QmwMSFpZs7h6Eziw3SpThFfczTDh5rW2krkqffa11UpX3XkeTTB2FvzZKWXqPY54Y6Rq4AQ5R8L/84'/0'/0'/3/*)";
    let wallet2: Wallet<MemoryDatabase> = Wallet::new(
        external_descriptor2,
        Some(internal_descriptor2),
        network,
        MemoryDatabase::new(),
    )?;

    let external_descriptor3 = "wpkh(tprv8ZgxMBicQKsPdy6LMhUtFHAgpocR8GC6QmwMSFpZs7h6Eziw3SpThFfczTDh5rW2krkqffa11UpX3XkeTTB2FvzZKWXqPY54Y6Rq4AQ5R8L/84'/0'/0'/4/*)";
    let internal_descriptor3 = "wpkh(tprv8ZgxMBicQKsPdy6LMhUtFHAgpocR8GC6QmwMSFpZs7h6Eziw3SpThFfczTDh5rW2krkqffa11UpX3XkeTTB2FvzZKWXqPY54Y6Rq4AQ5R8L/84'/0'/0'/5/*)";
    let wallet3: Wallet<MemoryDatabase> = Wallet::new(
        external_descriptor3,
        Some(internal_descriptor3),
        network,
        MemoryDatabase::new(),
    )?;

    let mut file = File::create("addresses.txt").expect("Error with file creation");

    writeln!(
        file,
        "{}",
        wallet1.get_address(AddressIndex::New)?.to_string()
    );
    writeln!(
        file,
        "{}",
        wallet2.get_address(AddressIndex::New)?.to_string()
    );
    writeln!(
        file,
        "{}",
        wallet3.get_address(AddressIndex::New)?.to_string()
    );

    Ok(())
}
