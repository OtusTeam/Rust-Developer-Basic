use crypto_algorithms::*;

fn main() {
    let payload = b"some_secret";

    {
        let private_key = bls::BlsPrivateKey;
        let public_key = bls::BlsPublicKey;

        assert!(sign_and_verify::<bls::Bls>(
            payload,
            private_key,
            public_key
        ));

        let _arr = [0; bls::Bls::PRIVATE_KEY_SIZE];
        let _arr = [0; bls::Bls::PUBLIC_KEY_SIZE];
    }
    {
        let private_key = ed25519::Ed25519PrivateKey;
        let public_key = ed25519::Ed25519PublicKey;

        assert!(sign_and_verify::<ed25519::Ed25519>(
            payload,
            private_key,
            public_key
        ));
    }
}

fn sign_and_verify<T: CryptoAlgorithm>(
    payload: &[u8],
    private_key: T::PrivateKey,
    public_key: T::PublicKey,
) -> bool {
    let signature = T::sign(payload, private_key);
    T::verify(&signature, public_key)
}
