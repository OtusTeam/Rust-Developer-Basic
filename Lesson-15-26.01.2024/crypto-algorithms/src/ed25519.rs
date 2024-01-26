use super::*;

pub struct Ed25519;

impl CryptoAlgorithm for Ed25519 {
    const PRIVATE_KEY_SIZE: usize = 256;
    const PUBLIC_KEY_SIZE: usize = 256;
}

impl Sign for Ed25519 {
    type PrivateKey = Ed25519PrivateKey;

    fn sign(payload: &[u8], _private_key: Self::PrivateKey) -> Vec<u8> {
        payload.to_vec()
    }
}

impl Verify for Ed25519 {
    type PublicKey = Ed25519PublicKey;

    fn verify(_signature: &[u8], _public_key: Self::PublicKey) -> bool {
        true
    }
}

pub struct Ed25519PrivateKey;

pub struct Ed25519PublicKey;
