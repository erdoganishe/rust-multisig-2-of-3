use bdk::bitcoin::PublicKey;
use bdk::miniscript::descriptor::Descriptor;
use std::fs::File;
use std::io::Write;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

//get keys, generated in file
pub fn get_public_keys(path: &str) -> Vec<String> {
    let file = File::open(path).expect("Error with opening file");
    let reader = BufReader::new(file);

    let mut public_keys = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        let parts: Vec<_> = line.split("|").collect();
        let public_key = parts.get(1).unwrap();
        public_keys.push(public_key.to_string());
    }

    public_keys
}

//generate descriptor for p2wsh 2-of-3 multisig
pub fn generate_descriptor() -> Descriptor<PublicKey> {
    let public_keys = get_public_keys("keypairs.txt");
    let pk1 = public_keys[0].clone();
    let pk2 = public_keys[1].clone();
    let pk3 = public_keys[2].clone();

    let descriptor_str = format!("wsh(multi(2,{},{},{}))", pk1, pk2, pk3);
    let descriptor = Descriptor::<PublicKey>::from_str(&descriptor_str).unwrap();

    let mut file = File::create("descriptors.txt").expect("Error with file creation");

    writeln!(file, "Descriptor: {}", descriptor).expect("Error writing in file");

    descriptor
}
