//generate a key pair

use bdk::bitcoin::secp256k1::{self, rand, Secp256k1, SecretKey};
use bdk::bitcoin::{Network, PrivateKey};

pub fn generate_keypair() -> (PrivateKey, secp256k1::PublicKey) {
    
    let secp = Secp256k1::new();
    let mut rng = rand::thread_rng();
    let secret_key = SecretKey::new(&mut rng);

    let public_key = secp256k1::PublicKey::from_secret_key(&secp, &secret_key);

    let private_key = PrivateKey {
        compressed: true,
        network: Network::Bitcoin,
        inner: secret_key,
    };

    (private_key, public_key)
}