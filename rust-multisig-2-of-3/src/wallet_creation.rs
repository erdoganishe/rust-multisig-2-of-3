use bdk::bitcoin::bip32::ExtendedPrivKey;
use bdk::bitcoin::key::Secp256k1;
use bdk::bitcoin::{Network, PrivateKey};
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
    let (mut wallet, _, _) = get_funded_wallet("wsh(multi(2,039607dcec14960f5d7d9b3ee1baad8d09d1f06eefd726f6db06dbeaeb603b253d,029fa04e64a0cf9f05d9e14bd84f5f6e39e84fb8fb271da2d717dbbd8ec117f9c4,026c5988a312800d566305140c89b7697dd752abdb9eea75de37c8624c09f6ff4c))#qj7053gj");
    
    
    let addr = wallet.get_address(AddressIndex::New).unwrap();
    let balance = wallet.get_balance().unwrap();
    let mut builder = wallet.build_tx();
    builder
        .drain_to(addr.script_pubkey())
        .only_witness_utxo()
        .drain_wallet();
    let (mut psbt, _) = builder.finish().unwrap();

    let secp = Secp256k1::new();
    let private_keys = get_private_keys("keypairs.txt");
    println!("{:#?}", private_keys);
    
    for private_key in private_keys {
        let xpriv = PrivateKey::from_wif(&private_key).unwrap();
        println!("{}", xpriv);
        let signer: SignerWrapper<PrivateKey> = SignerWrapper::new(
            xpriv,
            SignerContext::Segwitv0
        );

        wallet.add_signer(
            KeychainKind::External,
            SignerOrdering::default(),
            Arc::new(signer) as Arc<dyn TransactionSigner>);
    }

    
    // let signer: std::sync::Arc<dyn TransactionSigner>;
    // wallet.add_signer(KeychainKind::External, bdk::signer::SignerOrdering::default(), signer);
    
    // let desc = descriptor!(sh(multi(
    //     2,
    //     private_keys[0].clone(),
    //     private_keys[1].clone(),
    //     private_keys[2].clone()
    // )))
    // .unwrap();

    // // println!("{:#?}", desc);

    // let (wallet_desc, keymap) = desc
    //     .into_wallet_descriptor(&secp, Network::Bitcoin)
    //     .unwrap();

    // let signers_container = SignersContainer::build(keymap, &wallet_desc, &secp);
    // println!("{:#?}", signers_container);
    // let signers = signers_container.signers();
    // let mut signatures = vec![];
    // let sign_option = SignOptions::default();
    // for signer in signers {
    //     let private_key = signer.clone();
    //     let signature = private_key.sign_transaction(&mut psbt, &sign_option, &secp);
    //     signatures.push(signature.unwrap());
    // }

    // println!("{:#?}", signatures);

    // let signed_psbt = wallet.sign(&mut psbt, sign_option).unwrap();

    // println!("Signed PSBT: {}", signed_psbt);
    // println!("Psbt: {}", psbt);

    println!("Addr: {}", addr);
    println!("Balance: {}", balance);

    
}

// #[test]
// fn test_create_tx_only_witness_utxo() {
//     let (wallet, _, _) =
//         get_funded_wallet("wsh(pk(cVpPVruEDdmutPzisEsYvtST1usBR3ntr8pXSyt6D2YYqXRyPcFW))");
//     let addr = wallet.get_address(New).unwrap();
//     let mut builder = wallet.build_tx();
//     builder
//         .drain_to(addr.script_pubkey())
//         .only_witness_utxo()
//         .drain_wallet();
//     let (psbt, _) = builder.finish().unwrap();

//     assert!(psbt.inputs[0].non_witness_utxo.is_none());
//     assert!(psbt.inputs[0].witness_utxo.is_some());
// }

// #[test]
// fn test_include_output_redeem_witness_script() {
//     let (wallet, _, _) = get_funded_wallet("sh(wsh(multi(1,cVpPVruEDdmutPzisEsYvtST1usBR3ntr8pXSyt6D2YYqXRyPcFW,cRjo6jqfVNP33HhSS76UhXETZsGTZYx8FMFvR9kpbtCSV1PmdZdu)))");
//     let addr = Address::from_str("2N1Ffz3WaNzbeLFBb51xyFMHYSEUXcbiSoX")
//         .unwrap()
//         .assume_checked();
//     let mut builder = wallet.build_tx();
//     builder
//         .add_recipient(addr.script_pubkey(), 45_000)
//         .include_output_redeem_witness_script();
//     let (psbt, _) = builder.finish().unwrap();

//     // p2sh-p2wsh transaction should contain both witness and redeem scripts
//     assert!(psbt
//         .outputs
//         .iter()
//         .any(|output| output.redeem_script.is_some() && output.witness_script.is_some()));
// }
