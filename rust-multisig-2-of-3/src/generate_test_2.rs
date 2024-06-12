use bdk::{
    blockchain::ElectrumBlockchain,
    bitcoin::{Network, Address},
    database::MemoryDatabase,
    electrum_client::Client,
    wallet::{Wallet, AddressIndex}, 
    SyncOptions,   
    SignOptions,
};
use bdk::bitcoin::{Script};

use std::str::FromStr;

pub fn create_wallet() -> Result<(), Box<dyn std::error::Error>> {
    let external_descriptor = "wpkh(tprv8ZgxMBicQKsPdy6LMhUtFHAgpocR8GC6QmwMSFpZs7h6Eziw3SpThFfczTDh5rW2krkqffa11UpX3XkeTTB2FvzZKWXqPY54Y6Rq4AQ5R8L/84'/0'/0'/0/*)";
    let internal_descriptor = "wpkh(tprv8ZgxMBicQKsPdy6LMhUtFHAgpocR8GC6QmwMSFpZs7h6Eziw3SpThFfczTDh5rW2krkqffa11UpX3XkeTTB2FvzZKWXqPY54Y6Rq4AQ5R8L/84'/0'/0'/1/*)";

    let wallet: Wallet<MemoryDatabase> = Wallet::new(
        external_descriptor,
        Some(internal_descriptor),
        Network::Testnet,
        MemoryDatabase::new(),
        
    )?;
  

    let client = Client::new("ssl://electrum.blockstream.info:60002")?;
    let blockchain = ElectrumBlockchain::from(client);
    wallet.sync(&blockchain, SyncOptions::default())?;

    let address = wallet.get_address(AddressIndex::New)?;
    let script_pubkey = address.script_pubkey();
   
    println!("Public key: {}", script_pubkey);
    
    let balance = wallet.get_balance()?;
    println!("Wallet balance in SAT: {}", balance);

    let faucet_address = Address::from_str("mkHS9ne12qx9pS9VojpwU5xtRd4T7X7ZUt")?;

    // let mut tx_builder = wallet.build_tx();
    // tx_builder
    //     .add_recipient(faucet_address.payload.script_pubkey(), 
    //     (balance.trusted_pending + balance.confirmed) / 2)
    //     .enable_rbf();
    // let (mut psbt, tx_details) = tx_builder.finish()?;
    // println!("Transaction details: {:#?}", tx_details);
    println!("Generated Address: {}", address);


    Ok(())
}
