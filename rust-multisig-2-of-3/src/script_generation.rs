use bdk::bitcoin::Address;
use bdk::bitcoin::secp256k1::Secp256k1;
use bdk::bitcoin::Script;
use bdk::keys::DescriptorPublicKey;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

pub fn get_keys() -> Result<(), Box<dyn std::error::Error>> {

    let file = File::open("public_keys.txt")?;
    let reader = BufReader::new(file);

    let mut public_keys = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<_> = line.split("PublicKey(").collect();
        let parts2: Vec<_> = parts.get(1).unwrap().split(")").collect();
        let public_key = parts2.get(0).unwrap();
        public_keys.push(public_key.to_string());
    }

    println!("{:#?}", public_keys);


    Ok(())
}