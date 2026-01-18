#![doc = include_str!("../README.md")]

#[cfg(all(feature = "async", feature = "sync"))]
compile_error!(
    "Features `async` and `sync` are mutually exclusive. Please enable only one of them."
);

#[cfg(not(any(feature = "async", feature = "sync")))]
compile_error!("Either `async` or `sync` feature must be enabled.");

#[cfg(feature = "async")]
use std::future::Future;
use std::{
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

#[cfg(feature = "sync")]
use std::fs::read;
#[cfg(feature = "async")]
use tokio::fs::read;

#[cfg(feature = "sync")]
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

#[cfg(feature = "async")]
pub trait Sha2Hasher {
    /// Hashes with the SHA-224 algorithm.
    #[cfg(feature = "sha224")]
    fn sha224(&self) -> impl Future<Output = Result<String, Error>> + Send;

    /// Hashes with the SHA-256 algorithm.
    #[cfg(feature = "sha256")]
    fn sha256(&self) -> impl Future<Output = Result<String, Error>> + Send;

    /// Hashes with the SHA-384 algorithm.
    #[cfg(feature = "sha384")]
    fn sha384(&self) -> impl Future<Output = Result<String, Error>> + Send;

    /// Hashes with the SHA-512 algorithm.
    #[cfg(feature = "sha512")]
    fn sha512(&self) -> impl Future<Output = Result<String, Error>> + Send;
}

/// Implement the `Sha2Hasher` trait for any type that can be converted to a `Path`.
#[cfg(feature = "sync")]
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

/// Implement the `Sha2Hasher` trait for any type that can be converted to a `Path`.
#[cfg(feature = "async")]
impl<P> Sha2Hasher for P
where
    P: AsRef<Path> + Sync,
{
    #[cfg(feature = "sha224")]
    async fn sha224(&self) -> Result<String, Error> {
        hash_file::<Sha224, _>(self).await
    }

    #[cfg(feature = "sha256")]
    async fn sha256(&self) -> Result<String, Error> {
        hash_file::<Sha256, _>(self).await
    }

    #[cfg(feature = "sha384")]
    async fn sha384(&self) -> Result<String, Error> {
        hash_file::<Sha384, _>(self).await
    }

    #[cfg(feature = "sha512")]
    async fn sha512(&self) -> Result<String, Error> {
        hash_file::<Sha512, _>(self).await
    }
}

#[cfg(feature = "sync")]
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

#[cfg(feature = "async")]
#[inline]
async fn hash_file<D, P>(path: P) -> Result<String, Error>
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
    hasher.update(read(path).await?);
    Ok(hasher.finalize().encode_hex())
}

#[cfg(all(test, feature = "sync"))]
mod sync_tests {
    use std::path::Path;

    use crate::Sha2Hasher;

    macro_rules! test {
        ($hash:ident, $expected:expr) => {
            #[test]
            fn $hash() {
                let hash = Path::new(".gitignore").$hash().unwrap();
                assert_eq!(hash, $expected);
            }
        };
    }

    #[cfg(feature = "sha224")]
    test!(sha224, "e7f68a0e088b02bded91142bb43538b0338ead063a1bdf1d158ef174");
    #[cfg(feature = "sha256")]
    test!(sha256, "44c92e3a70ad3307b7056871c2bdb096d8bfa9373f5bf06a79bb6324a20ff2fb");
    #[cfg(feature = "sha384")]
    test!(sha384, "16c6a6c5fb77fb778b0739b93005a54bf4d5d011ecfc151d1d28680df65829fb25e4f639d12ea5bd0d95fb15a02a9d46");
    #[cfg(feature = "sha512")]
    test!(sha512, "cce95db66253cee0b4543434b0a93382fdd876996f0783709144d7317cc1686b97f907a4f18da2bdf95461b140129eb93242a842b3eee0878973ac139482db54");
}

#[cfg(all(test, feature = "async"))]
mod async_tests {
    use std::path::Path;

    use crate::Sha2Hasher;

    macro_rules! test {
        ($hash:ident, $expected:expr) => {
            #[tokio::test]
            async fn $hash() {
                let hash = Path::new(".gitignore").$hash().await.unwrap();
                assert_eq!(hash, $expected);
            }
        };
    }

    #[cfg(feature = "sha224")]
    test!(sha224, "e7f68a0e088b02bded91142bb43538b0338ead063a1bdf1d158ef174");
    #[cfg(feature = "sha256")]
    test!(sha256, "44c92e3a70ad3307b7056871c2bdb096d8bfa9373f5bf06a79bb6324a20ff2fb");
    #[cfg(feature = "sha384")]
    test!(sha384, "16c6a6c5fb77fb778b0739b93005a54bf4d5d011ecfc151d1d28680df65829fb25e4f639d12ea5bd0d95fb15a02a9d46");
    #[cfg(feature = "sha512")]
    test!(sha512, "cce95db66253cee0b4543434b0a93382fdd876996f0783709144d7317cc1686b97f907a4f18da2bdf95461b140129eb93242a842b3eee0878973ac139482db54");
}
