//! Encrypt by wrapping a key with AES Key Wrap Algorithm (RFC 3394)
//!
//! See: https://docs.rs/aes-kw/0.2.1/aes_kw/
use std::ops::Deref;

use crate::error::Error;
use aes_kw::Kek;
pub use zeroize::Zeroize;
use zeroize::Zeroizing;

/// Encrypt data with a key. The key can be any type that derefs to a slice of 32 bytes.
/// Types should impl [Zeroize] so that the memory is zeroized after use.
///
/// # Example
/// ```
/// use seed_keeper_core::wrap::{encrypt, decrypt};
/// use zeroize::Zeroizing;
///
/// let key = Zeroizing::new([1u8; 32]);
/// let data = Zeroizing::new(vec![2u8; 32]);
/// let encrypted = encrypt(key.clone(), data.clone()).unwrap();
/// let decrypted = decrypt(key, &encrypted).unwrap();
/// assert_eq!(data, decrypted.into());
/// ```
pub fn encrypt(
    key: impl Deref<Target = [u8; 32]> + Zeroize,
    data: impl Deref<Target = impl AsRef<[u8]>> + Zeroize,
) -> Result<Vec<u8>, Error> {
    let kek = Kek::from(*key);
    Ok(kek.wrap_vec(&data.as_ref())?)
}

/// Decrypt data using a key. The decryption key should impl [Zeroize].
///
/// # Example
/// ```
/// use seed_keeper_core::wrap::{encrypt, decrypt};
/// use zeroize::Zeroizing;
///
/// let key = Zeroizing::new([1u8; 32]);
/// let data = Zeroizing::new(vec![2u8; 32]);
/// let encrypted = encrypt(key.clone(), data.clone()).unwrap();
/// let decrypted = decrypt(key, &encrypted).unwrap();
/// assert_eq!(data, decrypted.into());
/// ```
pub fn decrypt(
    key: impl Deref<Target = [u8; 32]> + Zeroize,
    data: impl AsRef<[u8]>,
) -> Result<Zeroizing<Vec<u8>>, Error> {
    let kek = Kek::from(*key);
    Ok(Zeroizing::new(kek.unwrap_vec(data.as_ref())?))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() -> Result<(), Error> {
        let key = Zeroizing::new([1u8; 32]);
        let data = Zeroizing::new(vec![2u8; 32]);
        let encrypted = encrypt(key.clone(), data.clone())?;
        let decrypted = decrypt(key, &encrypted)?;
        assert_eq!(data, decrypted.into());
        Ok(())
    }
}
