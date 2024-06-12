use bdk::bitcoin::Address;
use bdk::bitcoin::secp256k1::Secp256k1;
use bdk::bitcoin::Script;
use std::fs::File;
use std::io::{BufRead, BufReader};
use bdk::keys::{DescriptorPublicKey, IntoDescriptorKey};
use bdk::miniscript::descriptor::Descriptor;
use bdk::miniscript::TranslatePk;
use std::str::FromStr;


pub fn get_keys() -> Vec<String> {

    let file = File::open("public_keys.txt").expect("Error with opening file");
    let reader = BufReader::new(file);

    let mut public_keys = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        let parts: Vec<_> = line.split("PublicKey(").collect();
        let parts2: Vec<_> = parts.get(1).unwrap().split(")").collect();
        let public_key = parts2.get(0).unwrap();
        public_keys.push(public_key.to_string());
    }

    public_keys
}

pub fn generate_script()-> Descriptor<DescriptorPublicKey> {
    let public_keys = get_keys();
  
    let pk1 = DescriptorPublicKey::from_str(&public_keys[0]).unwrap();
    let pk2 = DescriptorPublicKey::from_str(&public_keys[1]).unwrap();
    let pk3 = DescriptorPublicKey::from_str(&public_keys[2]).unwrap();

    let descriptor_str = format!("wsh(multi(2,{},{},{}))", pk1, pk2, pk3);
    let descriptor = Descriptor::<DescriptorPublicKey>::from_str(&descriptor_str).unwrap();
    
    println!("Descriptor: {}", descriptor);
    
    descriptor
}

