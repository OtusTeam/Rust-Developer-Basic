use super::*;

pub struct Bls;

impl CryptoAlgorithm for Bls {
    const PRIVATE_KEY_SIZE: usize = 128;
    const PUBLIC_KEY_SIZE: usize = 128;
}

impl Sign for Bls {
    type PrivateKey = BlsPrivateKey;

    fn sign(payload: &[u8], _private_key: Self::PrivateKey) -> Vec<u8> {
        payload.to_vec()
    }
}

impl Verify for Bls {
    type PublicKey = BlsPublicKey;

    fn verify(_signature: &[u8], _public_key: Self::PublicKey) -> bool {
        true
    }
}

pub struct BlsPrivateKey;

pub struct BlsPublicKey;
