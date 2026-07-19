use std::{
    fs::File,
    io::{BufReader, Error, Read},
    path::Path,
};

use const_hex::ToHexExt;
use sha2::{Digest, Sha224, Sha256, Sha384, Sha512};

const BUFFER_SIZE: usize = 64 * 1024;

pub trait Sha2Hasher {
    /// Hashes with the SHA-224 algorithm.
    fn sha224(&self) -> Result<String, Error>;

    /// Hashes with the SHA-256 algorithm.
    fn sha256(&self) -> Result<String, Error>;

    /// Hashes with the SHA-384 algorithm.
    fn sha384(&self) -> Result<String, Error>;

    /// Hashes with the SHA-512 algorithm.
    fn sha512(&self) -> Result<String, Error>;
}

impl<P> Sha2Hasher for P
where
    P: AsRef<Path>,
{
    fn sha224(&self) -> Result<String, Error> {
        hash_file::<Sha224, _>(self)
    }

    fn sha256(&self) -> Result<String, Error> {
        hash_file::<Sha256, _>(self)
    }

    fn sha384(&self) -> Result<String, Error> {
        hash_file::<Sha384, _>(self)
    }

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
    let mut reader = BufReader::new(File::open(path)?);
    let mut hasher = D::new();
    let mut buffer = [0; BUFFER_SIZE];

    loop {
        let bytes_read = reader.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }

    Ok(hasher.finalize().encode_hex())
}

#[cfg(test)]
mod tests {
    #[cfg(test)]
    use std::env::temp_dir;
    #[cfg(test)]
    use std::fs::remove_file;
    #[cfg(test)]
    use std::fs::write;
    use std::path::Path;
    #[cfg(test)]
    use std::process::id;

    use const_hex::ToHexExt;
    use sha2::{Digest, Sha256};

    use super::Sha2Hasher;

    const TEST_FILE: &str = "tests/data/test.txt";

    #[test]
    fn sha224() {
        let hash = Path::new(TEST_FILE).sha224().unwrap();
        assert_eq!(hash, "c547cf5d6bf6b795abbe4c5cc7cac00f1d5ec17bcd74281ea89e6108");
    }

    #[test]
    fn sha256() {
        let hash = Path::new(TEST_FILE).sha256().unwrap();
        assert_eq!(hash, "c98c24b677eff44860afea6f493bbaec5bb1c4cbb209c6fc2bbb47f66ff2ad31");
    }

    #[test]
    fn sha384() {
        let hash = Path::new(TEST_FILE).sha384().unwrap();
        assert_eq!(
            hash,
            "d195483c9b554356ba50a855a605aaee134612dcfdd05988fc605181d93603f215a0d07812a0b333fc2ccc75025736f5"
        );
    }

    #[test]
    fn sha512() {
        let hash = Path::new(TEST_FILE).sha512().unwrap();
        assert_eq!(
            hash,
            "921618bc6d9f8059437c5e0397b13f973ab7c7a7b81f0ca31b70bf448fd800a460b67efda0020088bc97bf7d9da97a9e2ce7b20d46e066462ec44cf60284f9a7"
        );
    }

    #[test]
    fn hashes_files_larger_than_the_buffer() {
        let contents = vec![0xa5; 128 * 1024 + 17];
        let expected: String = Sha256::digest(&contents).encode_hex();
        let path = temp_dir().join(format!("sha2_hasher_sync_streaming_{}", id()));
        write(&path, contents).unwrap();

        let hash = path.sha256().unwrap();
        remove_file(path).unwrap();

        assert_eq!(hash, expected);
    }
}
