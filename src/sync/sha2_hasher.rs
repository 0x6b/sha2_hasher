use std::{
    fs::read,
    io::{
        Error,
        ErrorKind::{InvalidInput, NotFound},
    },
    path::Path,
};

use const_hex::ToHexExt;
use sha2::Digest;
#[cfg(feature = "sha224")]
use sha2::Sha224;
#[cfg(feature = "sha256")]
use sha2::Sha256;
#[cfg(feature = "sha384")]
use sha2::Sha384;
#[cfg(feature = "sha512")]
use sha2::Sha512;

pub trait Sha2Hasher {
    /// Hashes with the SHA-224 algorithm.
    #[cfg(feature = "sha224")]
    fn sha224(&self) -> Result<String, Error>;

    /// Hashes with the SHA-256 algorithm.
    #[cfg(feature = "sha256")]
    fn sha256(&self) -> Result<String, Error>;

    /// Hashes with the SHA-384 algorithm.
    #[cfg(feature = "sha384")]
    fn sha384(&self) -> Result<String, Error>;

    /// Hashes with the SHA-512 algorithm.
    #[cfg(feature = "sha512")]
    fn sha512(&self) -> Result<String, Error>;
}

/// Implement the `Sha2Hasher` trait for any type that can be converted to a `Path`.
impl<P> Sha2Hasher for P
where
    P: AsRef<Path>,
{
    #[cfg(feature = "sha224")]
    fn sha224(&self) -> Result<String, Error> {
        hash_file::<Sha224, _>(self)
    }

    #[cfg(feature = "sha256")]
    fn sha256(&self) -> Result<String, Error> {
        hash_file::<Sha256, _>(self)
    }

    #[cfg(feature = "sha384")]
    fn sha384(&self) -> Result<String, Error> {
        hash_file::<Sha384, _>(self)
    }

    #[cfg(feature = "sha512")]
    fn sha512(&self) -> Result<String, Error> {
        hash_file::<Sha512, _>(self)
    }
}

#[inline]
fn hash_file<D, P>(path: P) -> Result<String, Error>
where
    D: Digest,
    P: AsRef<Path>,
{
    let path = path.as_ref();
    if !path.is_file() {
        return Err(Error::new(
            if path.exists() { InvalidInput } else { NotFound },
            "Invalid path: must be an existing and accessible file",
        ));
    }

    let mut hasher = D::new();
    hasher.update(read(path)?);
    Ok(hasher.finalize().encode_hex())
}
