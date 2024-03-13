mod bindings;

use bindings::exports::seed_keeper::edwards_wit::operations::Guest;
use bindings::seed_keeper::wallet::config::get_seed;

use ed25519_dalek::SECRET_KEY_LENGTH;
use ed25519_dalek::{Signature, Signer, SigningKey};

struct Component;

bindings::export!(Component with_types_in bindings);

impl Guest for Component {
    /// Say hello!
    /// sign: func(message: list<u8>) -> list<u8>;
    fn sign(message: Vec<u8>) -> Result<Vec<u8>, String> {
        let seed = get_seed()?;
        let seed: [u8; SECRET_KEY_LENGTH] = seed
            .clone()
            .try_into()
            .map_err(|_| format!("Seed length is not 32 bytes, got {}", seed.len()).to_owned())?;
        let signer = SigningKey::from_bytes(&seed);
        let signature: Signature = signer.sign(&message);
        Ok(signature.to_bytes().to_vec())
    }

    /// Verify
    /// verify: func(message: list<u8>, signature: list<u8>) -> bool;
    fn verify(message: Vec<u8>, signature: Vec<u8>) -> Result<bool, String> {
        let seed = get_seed()?;
        let seed: [u8; SECRET_KEY_LENGTH] = seed
            .clone()
            .try_into()
            .map_err(|_| format!("Seed length is not 32 bytes, got {}", seed.len()).to_owned())?;
        let signer = SigningKey::from_bytes(&seed);
        let signature = Signature::from_bytes(
            &(signature.clone().try_into().map_err(|_| {
                format!("Signature length is not 64 bytes, got {}", signature.len()).to_owned()
            })?),
        );
        Ok(signer.verify(&message, &signature).is_ok())
    }
}

#[cfg(test)]
mod test_edwards_wit {
    use super::*;

    fn assert_keypair(keypair: &SigningKey) -> bool {
        let message: &[u8] = b"This is a test of the tsunami alert system.";
        let signature: Signature = keypair.sign(message);
        keypair.verify(message, &signature).is_ok()
    }

    #[test]
    fn test_keypair_from_seed() {
        let seed: [u8; 32] = [
            0x9d, 0x61, 0xb1, 0x9d, 0xef, 0xfd, 0x5a, 0x60, 0xba, 0x84, 0x4a, 0xf4, 0x92, 0xec,
            0x2c, 0xc4, 0x44, 0x49, 0xc5, 0x69, 0x34, 0x67, 0x2b, 0x6a, 0x2f, 0x8d, 0x1d, 0x7b,
            0x10, 0xac, 0x39, 0x23,
        ];
        let keypair = SigningKey::from_bytes(&seed);
        assert_keypair(&keypair);
    }
}
