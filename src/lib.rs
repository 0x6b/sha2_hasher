#![doc = include_str!("../README.md")]
mod sha2_hasher;
pub use sha2_hasher::Sha2Hasher;

#[cfg(test)]
mod tests {
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
