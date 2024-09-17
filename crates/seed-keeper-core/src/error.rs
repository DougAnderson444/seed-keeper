//! Crate errors

#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// AES-KW Error
    #[error("AES-Key Wrapping (encrypt/decrypt) Error: {0}")]
    KeyWrap(#[from] aes_kw::Error),

    /// Argon2 error
    #[error("Argon2 Error")]
    Argon2(#[from] argon2::Error),

    /// String Message
    #[error("{0}")]
    Message(String),

    /// Failed to create Wallet
    #[error("Failed to create Wallet")]
    WalletCreation,

    /// Seed too short "Seed must be 32 bytes long"
    #[error("Seed must be 32 bytes long")]
    SeedLength,

    /// Value too short, "Must be at least {} characters long"
    #[error("Must be at least {0} characters long")]
    ValueTooShort(usize),
}

impl From<String> for Error {
    fn from(e: String) -> Self {
        Error::Message(e)
    }
}
