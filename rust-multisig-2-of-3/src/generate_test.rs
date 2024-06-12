use bdk::{
    bitcoin::Network,
    database::MemoryDatabase,
    wallet::Wallet, KeychainKind,
    descriptor::{Descriptor, DescriptorPublicKey},
};
use regex::Regex;
use std::str::FromStr;


pub fn generate() -> Result<Descriptor<DescriptorPublicKey>, Box<dyn std::error::Error>> {
    let network = Network::Testnet;

    // External descriptors for each wallet
    let external_descriptor1 = "wpkh([c7c3c6e3/84'/0'/0']tpubD6NzVbkrYhZ4YULNz1CfmJzoKJeq4rAcQnMygnwAjFLZ8PYzGJzXEaENkBiHzTjtAfNyL2ebSHk7uP5kKbZUMvF4CVQRPnkg9kQXvKVSXZ7/0/*)";
    let external_descriptor2 = "wpkh([c7c3c6e3/84'/0'/1']tpubD6NzVbkrYhZ4YULNz1CfmJzoKJeq4rAcQnMygnwAjFLZ8PYzGJzXEaENkBiHzTjtAfNyL2ebSHk7uP5kKbZUMvF4CVQRPnkg9kQXvKVSXZ7/0/*)";
    let external_descriptor3 = "wpkh([c7c3c6e3/84'/0'/2']tpubD6NzVbkrYhZ4YULNz1CfmJzoKJeq4rAcQnMygnwAjFLZ8PYzGJzXEaENkBiHzTjtAfNyL2ebSHk7uP5kKbZUMvF4CVQRPnkg9kQXvKVSXZ7/0/*)";

    // Create wallets
    let wallet1: Wallet<MemoryDatabase> = Wallet::new(external_descriptor1, None, network, MemoryDatabase::new())?;
    let wallet2: Wallet<MemoryDatabase> = Wallet::new(external_descriptor2, None, network, MemoryDatabase::new())?;
    let wallet3: Wallet<MemoryDatabase> = Wallet::new(external_descriptor3, None, network, MemoryDatabase::new())?;

    // Get public descriptors
    let public_descriptor1 = wallet1.public_descriptor(KeychainKind::External)?.unwrap();
    let public_descriptor2 = wallet2.public_descriptor(KeychainKind::External)?.unwrap();
    let public_descriptor3 = wallet3.public_descriptor(KeychainKind::External)?.unwrap();

    println!("Public Descriptor 1: {:?}", public_descriptor1);
    println!("Public Descriptor 2: {:?}", public_descriptor2);
    println!("Public Descriptor 3: {:?}", public_descriptor3);

    // Extract public keys from descriptors
    let pubkey1 = extract_pubkey(&public_descriptor1)?;
    let pubkey2 = extract_pubkey(&public_descriptor2)?;
    let pubkey3 = extract_pubkey(&public_descriptor3)?;

    println!("Public Key 1: {}", pubkey1);
    println!("Public Key 2: {}", pubkey2);
    println!("Public Key 3: {}", pubkey3);

    // Create a 2-of-3 multisig descriptor
    let descriptor_str = format!("wsh(multi(2,{},{},{}))", pubkey1, pubkey2, pubkey3);
    println!("Descriptor String: {}", descriptor_str);

    let descriptor = Descriptor::<DescriptorPublicKey>::from_str(&descriptor_str)?;
    Ok(descriptor)
}

fn extract_pubkey(descriptor: &Descriptor<DescriptorPublicKey>) -> Result<String, Box<dyn std::error::Error>> {
    let descriptor_str = descriptor.to_string();
    println!("Descriptor String for Extraction: {}", descriptor_str);

    let re = Regex::new(r"\[.*?\](tpub[a-zA-Z0-9]+)")?;
    let captures = re.captures(&descriptor_str)
        .ok_or("Failed to extract public key from descriptor")?;

    let pubkey = captures.get(1).ok_or("Failed to extract public key from descriptor")?.as_str();
    Ok(pubkey.to_string())
}