//! Encrypt by wrapping a key with AES Key Wrap Algorithm (RFC 3394)
//!
//! See: https://docs.rs/aes-kw/0.2.1/aes_kw/
use aes_kw::Kek;

pub fn encrypt(key: [u8; 32], data: &[u8]) -> Vec<u8> {
    let kek = Kek::from(key);
    kek.wrap_vec(data).unwrap()
}

pub fn decrypt(key: [u8; 32], data: &[u8]) -> Vec<u8> {
    let kek = Kek::from(key);
    kek.unwrap_vec(data).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let key = [1u8; 32];
        let data = vec![2u8; 32];
        let encrypted = encrypt(key, &data);
        let decrypted = decrypt(key, &encrypted);
        assert_eq!(data, decrypted);
    }
}
