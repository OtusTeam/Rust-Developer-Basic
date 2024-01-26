pub mod bls;
pub mod ed25519;

pub trait CryptoAlgorithm: Sign + Verify {
    const PRIVATE_KEY_SIZE: usize;
    const PUBLIC_KEY_SIZE: usize;
}

pub trait Sign {
    type PrivateKey;

    fn sign(payload: &[u8], private_key: Self::PrivateKey) -> Vec<u8>;
}

pub trait Verify {
    type PublicKey;

    fn verify(signature: &[u8], public_key: Self::PublicKey) -> bool;
}
