//! Encrypt by wrapping a key with AES Key Wrap Algorithm (RFC 3394)
//!
//! See: https://docs.rs/aes-kw/0.2.1/aes_kw/
use crate::error::Error;
use aes_kw::Kek;

/// Encrypt data with a key that derefs to a slice of 32 bytes
pub fn encrypt(key: [u8; 32], data: &[u8]) -> Result<Vec<u8>, Error> {
    let kek = Kek::from(key);
    Ok(kek.wrap_vec(data)?)
}

pub fn decrypt(key: [u8; 32], data: &[u8]) -> Result<Vec<u8>, Error> {
    let kek = Kek::from(key);
    Ok(kek.unwrap_vec(data)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() -> Result<(), Error> {
        let key = [1u8; 32];
        let data = vec![2u8; 32];
        let encrypted = encrypt(key, &data)?;
        let decrypted = decrypt(key, &encrypted)?;
        assert_eq!(data, decrypted);
        Ok(())
    }

    #[test]
    fn it_fails() -> Result<(), Error> {
        let key = [1u8; 32];
        let data = vec![2u8; 32];
        let encrypted = encrypt(key, &data)?;
        let decrypted = decrypt([2u8; 32], &encrypted);
        assert!(decrypted.is_err());
        Ok(())
    }
}
