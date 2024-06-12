use bdk::bitcoin::Address;
use bdk::bitcoin::secp256k1::Secp256k1;
use bdk::bitcoin::Script;
use bdk::keys::DescriptorPublicKey;
use std::fs::File;
use std::io::{BufRead, BufReader};

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

pub fn generate_script(){
    let public_keys = get_keys();
  
    // let mut script = Script::new();
    // script.push_opcode(bdk::bitcoin::Opcode::OP_2); 
    // for pub_key in pub_keys {
    //     script.push_data(hex::decode(pub_key).unwrap());
    // }
    // script.push_opcode(bdk::bitcoin::Opcode::OP_3); // 3
    // script.push_opcode(bdk::bitcoin::Opcode::OP_CHECKMULTISIG);

}

