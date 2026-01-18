//! A trait extension for hashing files with SHA-2 algorithms.
//!
//! This crate provides the [`Sha2Hasher`] trait, which adds `sha224`, `sha256`, `sha384`,
//! and `sha512` methods to any type implementing [`AsRef<Path>`](std::path::Path).
//!
//! # Features
//!
//! One of the following features must be enabled:
//!
//! - `async` - Async implementation using tokio
//! - `sync` - Blocking implementation
//!
//! These features are mutually exclusive.
//!
//! # Example
#![cfg_attr(feature = "async", doc = "```no_run")]
#![cfg_attr(not(feature = "async"), doc = "```ignore")]
//! use sha2_hasher::Sha2Hasher;
//!
//! # #[tokio::main(flavor = "current_thread")]
//! # async fn main() {
//! let hash = std::path::Path::new("Cargo.toml").sha256().await.unwrap();
//! println!("{hash}");
//! # }
//! ```

#[cfg(all(feature = "async", feature = "sync"))]
compile_error!(
    "Features `async` and `sync` are mutually exclusive. Please enable only one of them."
);

#[cfg(not(any(feature = "async", feature = "sync")))]
compile_error!("Either `async` or `sync` feature must be enabled.");

#[cfg(feature = "async")]
mod r#async;
#[cfg(feature = "async")]
pub use r#async::Sha2Hasher;

#[cfg(feature = "sync")]
mod sync;
#[cfg(feature = "sync")]
pub use sync::Sha2Hasher;
