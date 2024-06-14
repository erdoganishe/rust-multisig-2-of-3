use bdk::bitcoin::bip32::ExtendedPrivKey;
use bdk::bitcoin::key::Secp256k1;
use bdk::bitcoin::consensus::encode::serialize;
use bdk::bitcoin::{base64, Network, PrivateKey};
use bdk::blockchain::{Blockchain, ElectrumBlockchain};
use bdk::database::MemoryDatabase;
use bdk::descriptor::IntoWalletDescriptor;
use bdk::electrum_client::Client;
use bdk::signer::{SignerContext, SignerOrdering, SignerWrapper, SignersContainer, TransactionSigner};
use bdk::wallet::{self, AddressIndex};
use bdk::wallet::{get_funded_wallet, Wallet};
use bdk::{descriptor, KeychainKind, SignOptions};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;
use std::sync::Arc;


//get private keys from file
pub fn get_private_keys(path: &str) -> Vec<String> {
    let file = File::open(path).expect("Error with opening file");
    let reader = BufReader::new(file);

    let mut private_keys = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        let parts: Vec<_> = line.split("|").collect();
        let private_key = parts.get(0).unwrap();
        private_keys.push(private_key.to_string());
    }

    private_keys
}
pub fn get_descriptor(path: &str) -> String{
    let file = File::open(path).expect("Error with opening file");
    let reader = BufReader::new(file);

    let mut descriptor:String = "".to_string();

    for line in reader.lines() {
        let line = line.unwrap();
        descriptor = line;
    }

    descriptor
}
pub fn create_wallet() {
    let descriptor = &get_descriptor("descriptors.txt");
    // let (mut wallet, _, _) = get_funded_wallet(descriptor);

    let mut wallet = Wallet::new(descriptor, None, 
        Network::Testnet,
        MemoryDatabase::new()).unwrap();
 
    println!("Network: {}", wallet.network());

    let addr = wallet.get_address(AddressIndex::New).unwrap();
    let balance = wallet.get_balance().unwrap();
    println!("Addr: {}", addr);
    println!("Balance: {}", balance);
    let client = Client::new("ssl://electrum.blockstream.info:60002").expect("failed to connect");
    let blockchain = ElectrumBlockchain::from(client);
    wallet.sync(&blockchain, Default::default()).unwrap();

    // Get all UTXOs
    let utxos = wallet.list_unspent().unwrap();

    // Print the UTXOs
    for utxo in utxos {
        println!("{:?}", utxo);
    }

    let mut builder = wallet.build_tx();
    builder
        .drain_to(addr.script_pubkey())
        .fee_rate(bdk::FeeRate::from_sat_per_vb(35.0))
        .only_witness_utxo()
        .drain_wallet();
    let (mut psbt, _) = builder.finish().unwrap();

    let private_keys = get_private_keys("keypairs.txt");
    println!("{:#?}", private_keys);
    for private_key in private_keys {
        let xpriv = PrivateKey::from_wif(&private_key).unwrap();
        let signer: SignerWrapper<PrivateKey> = SignerWrapper::new(
            xpriv,
            SignerContext::Segwitv0
        );

        wallet.add_signer(
            KeychainKind::External,
            SignerOrdering::default(),
            Arc::new(signer) 
        );
    }

    // Sign the PSBT with the added signers
    let sign_options = SignOptions {
        trust_witness_utxo: true,    
        ..Default::default()    
    };
    println!("{:#?}", sign_options.trust_witness_utxo);
    println!("Non-signed PSBT: {}", psbt);
    wallet.sign(&mut psbt, sign_options).unwrap();

    // Print the final PSBT
    println!("Final PSBT: {}", psbt);
 
    let final_tx = psbt.extract_tx();
    println!("{:#?}", final_tx);
    let txid = blockchain.broadcast(&final_tx).unwrap();
    println!("Transaction broadcasted with txid: {:#?}", txid);
}