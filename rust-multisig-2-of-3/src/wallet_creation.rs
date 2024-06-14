use bdk::bitcoin::bip32::ExtendedPrivKey;
use bdk::bitcoin::key::Secp256k1;
use bdk::bitcoin::consensus::encode::serialize;
use bdk::bitcoin::{base64, Network, PrivateKey};
use bdk::descriptor::IntoWalletDescriptor;
use bdk::signer::{SignerContext, SignerOrdering, SignerWrapper, SignersContainer, TransactionSigner};
use bdk::wallet::AddressIndex;
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

pub fn create_wallet() {
    let descriptor = "wsh(multi(2,039607dcec14960f5d7d9b3ee1baad8d09d1f06eefd726f6db06dbeaeb603b253d,029fa04e64a0cf9f05d9e14bd84f5f6e39e84fb8fb271da2d717dbbd8ec117f9c4,026c5988a312800d566305140c89b7697dd752abdb9eea75de37c8624c09f6ff4c))#qj7053gj";
    let (mut wallet, _, _) = get_funded_wallet(descriptor);

    let addr = wallet.get_address(AddressIndex::New).unwrap();
    let balance = wallet.get_balance().unwrap();
    println!("Addr: {}", addr);
    println!("Balance: {}", balance);

    // Ensure that you have UTXOs
    // let utxos = wallet.list_unspent().unwrap();

    let mut builder = wallet.build_tx();
    builder
        .drain_to(addr.script_pubkey())
        .only_witness_utxo()
        .drain_wallet();
    let (mut psbt, _) = builder.finish().unwrap();

    let private_keys = get_private_keys("keypairs.txt");
    
    for private_key in private_keys {
        let xpriv = PrivateKey::from_wif(&private_key).unwrap();
        let signer: SignerWrapper<PrivateKey> = SignerWrapper::new(
            xpriv,
            SignerContext::Segwitv0
        );

        wallet.add_signer(
            KeychainKind::External,
            SignerOrdering::default(),
            Arc::new(signer) as Arc<dyn TransactionSigner>
        );
    }

    // Sign the PSBT with the added signers
    let sign_options = SignOptions {
        trust_witness_utxo: true,    
        ..Default::default()    
    };
    println!("{:#?}", sign_options.trust_witness_utxo);
    wallet.sign(&mut psbt, sign_options).unwrap();

    // Print the final PSBT
    println!("Final PSBT: {}", psbt);

}