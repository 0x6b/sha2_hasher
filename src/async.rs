use std::{
    future::Future,
    io::{
        Error,
        ErrorKind::{InvalidInput, NotFound},
    },
    path::Path,
};

use const_hex::ToHexExt;
use sha2::{Digest, Sha224, Sha256, Sha384, Sha512};
use tokio::fs::read;

pub trait Sha2Hasher {
    /// Hashes with the SHA-224 algorithm.
    fn sha224(&self) -> impl Future<Output = Result<String, Error>> + Send;

    /// Hashes with the SHA-256 algorithm.
    fn sha256(&self) -> impl Future<Output = Result<String, Error>> + Send;

    /// Hashes with the SHA-384 algorithm.
    fn sha384(&self) -> impl Future<Output = Result<String, Error>> + Send;

    /// Hashes with the SHA-512 algorithm.
    fn sha512(&self) -> impl Future<Output = Result<String, Error>> + Send;
}

impl<P> Sha2Hasher for P
where
    P: AsRef<Path> + Sync,
{
    async fn sha224(&self) -> Result<String, Error> {
        hash_file::<Sha224, _>(self).await
    }

    async fn sha256(&self) -> Result<String, Error> {
        hash_file::<Sha256, _>(self).await
    }

    async fn sha384(&self) -> Result<String, Error> {
        hash_file::<Sha384, _>(self).await
    }

    async fn sha512(&self) -> Result<String, Error> {
        hash_file::<Sha512, _>(self).await
    }
}

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

#[cfg(test)]
mod tests {
    use std::path::Path;

    use crate::Sha2Hasher;

    #[tokio::test]
    async fn sha224() {
        let hash = Path::new(".gitignore").sha224().await.unwrap();
        assert_eq!(hash, "e7f68a0e088b02bded91142bb43538b0338ead063a1bdf1d158ef174");
    }

    #[tokio::test]
    async fn sha256() {
        let hash = Path::new(".gitignore").sha256().await.unwrap();
        assert_eq!(hash, "44c92e3a70ad3307b7056871c2bdb096d8bfa9373f5bf06a79bb6324a20ff2fb");
    }

    #[tokio::test]
    async fn sha384() {
        let hash = Path::new(".gitignore").sha384().await.unwrap();
        assert_eq!(hash, "16c6a6c5fb77fb778b0739b93005a54bf4d5d011ecfc151d1d28680df65829fb25e4f639d12ea5bd0d95fb15a02a9d46");
    }

    #[tokio::test]
    async fn sha512() {
        let hash = Path::new(".gitignore").sha512().await.unwrap();
        assert_eq!(hash, "cce95db66253cee0b4543434b0a93382fdd876996f0783709144d7317cc1686b97f907a4f18da2bdf95461b140129eb93242a842b3eee0878973ac139482db54");
    }
}
