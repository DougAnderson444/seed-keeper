//! Crate errors

#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// AES-KW Error
    #[error("AES-Key Wrapping (encrypt/decrypt) Error: {0}")]
    KeyWrap(#[from] aes_kw::Error),
}
