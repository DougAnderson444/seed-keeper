use seed_keeper_core::seed::{rand_seed, Seed};
use seed_keeper_core::wrap::{decrypt, encrypt};
use seed_keeper_core::{derive_key, ExposeSecret, Secret};

#[test]
pub fn integration_tests() {
    // Generate a secure random seed of 32 bytes:

    let seed: Seed = rand_seed();
    assert_eq!(seed.len(), 32);

    // Generate output key material from a username and password:

    let password = "some random words that you made up, for sure!".to_string();
    let salt = b"some@email.com"; // Salt should be unique per password

    let key: Secret<Seed> = derive_key(password, salt).unwrap();

    assert_eq!(
        **key.expose_secret(),
        [
            164, 103, 254, 113, 126, 241, 57, 240, 100, 56, 243, 125, 155, 224, 40, 242, 178, 136,
            222, 133, 220, 141, 127, 10, 88, 199, 181, 11, 241, 91, 149, 249
        ]
    );

    // Protect your new seed by encrypting it with the password and salt key:
    let encrypted = encrypt((**key.expose_secret()).try_into().unwrap(), &seed).unwrap();
    let decrypted = decrypt((**key.expose_secret()).try_into().unwrap(), &encrypted).unwrap();
    assert_eq!(*seed, *decrypted.as_slice());
}
