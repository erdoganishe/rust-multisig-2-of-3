//test for creating wsh descriptor

use bdk::descriptor::{Descriptor};
use bdk::bitcoin::{opcodes, Script};
use bdk::miniscript::descriptor::Wsh;
use miniscript::descriptor::{Descriptor, Wsh};


use std::fs::File;
use std::io::Write;

pub fn generate_desc(){
    let alice_xpub = "xpub..."; // Alice's extended public key
    let bob_xpub = "xpub..."; // Bob's extended public key
    let carol_xpub = "xpub..."; // Carol's extended public key
    let wsh_descriptor = Descriptor::Wsh(
        Wsh<>
    );
    // let descriptor = Descriptors::Multi(Multi {
    //     threshold: 2, // 2-of-3 multisig
    //     pubkeys: vec![alice_xpub.parse().unwrap(), bob_xpub.parse().unwrap(), carol_xpub.parse().unwrap()],
    // });
    let multisig_script = Script::builder()
    .push_int(2)
    .push_slice(&pubkey1.key.serialize())
    .push_slice(&pubkey2.key.serialize())
    .push_slice(&pubkey3.key.serialize())
    .push_int(3)
    .push_opcode(opcodes::all::OP_CHECKMULTISIG)
    .into_script();

    let wsh_descriptor = Descriptor::Wsh(Wsh::new(multisig_script));

    let mut file = File::create("multisig_descriptor.txt").expect("msg");
    writeln!(file, "{}", descriptor);
}
