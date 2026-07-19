use std::{
    future::Future,
    io::Error,
    path::Path,
};

use const_hex::ToHexExt;
use sha2::{Digest, Sha224, Sha256, Sha384, Sha512};
use tokio::{
    fs::File,
    io::{AsyncReadExt, BufReader},
};

const BUFFER_SIZE: usize = 64 * 1024;

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
    let mut reader = BufReader::new(File::open(path).await?);
    let mut hasher = D::new();
    let mut buffer = [0; BUFFER_SIZE];

    loop {
        let bytes_read = reader.read(&mut buffer).await?;
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }

    Ok(hasher.finalize().encode_hex())
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use crate::Sha2Hasher;
    use const_hex::ToHexExt;
    use sha2::{Digest, Sha256};

    const TEST_FILE: &str = "tests/data/test.txt";

    #[tokio::test]
    async fn sha224() {
        let hash = Path::new(TEST_FILE).sha224().await.unwrap();
        assert_eq!(hash, "c547cf5d6bf6b795abbe4c5cc7cac00f1d5ec17bcd74281ea89e6108");
    }

    #[tokio::test]
    async fn sha256() {
        let hash = Path::new(TEST_FILE).sha256().await.unwrap();
        assert_eq!(hash, "c98c24b677eff44860afea6f493bbaec5bb1c4cbb209c6fc2bbb47f66ff2ad31");
    }

    #[tokio::test]
    async fn sha384() {
        let hash = Path::new(TEST_FILE).sha384().await.unwrap();
        assert_eq!(
            hash,
            "d195483c9b554356ba50a855a605aaee134612dcfdd05988fc605181d93603f215a0d07812a0b333fc2ccc75025736f5"
        );
    }

    #[tokio::test]
    async fn sha512() {
        let hash = Path::new(TEST_FILE).sha512().await.unwrap();
        assert_eq!(
            hash,
            "921618bc6d9f8059437c5e0397b13f973ab7c7a7b81f0ca31b70bf448fd800a460b67efda0020088bc97bf7d9da97a9e2ce7b20d46e066462ec44cf60284f9a7"
        );
    }

    #[tokio::test]
    async fn hashes_files_larger_than_the_buffer() {
        let contents = vec![0xa5; 128 * 1024 + 17];
        let expected: String = Sha256::digest(&contents).encode_hex();
        let path = std::env::temp_dir().join(format!(
            "sha2_hasher_async_streaming_{}",
            std::process::id()
        ));
        tokio::fs::write(&path, contents).await.unwrap();

        let hash = path.sha256().await.unwrap();
        tokio::fs::remove_file(path).await.unwrap();

        assert_eq!(hash, expected);
    }
}
