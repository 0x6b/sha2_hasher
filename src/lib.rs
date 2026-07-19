//! A trait extension for hashing files with SHA-2 algorithms.
//!
//! This crate provides the [`Sha2Hasher`] trait, which adds `sha224`, `sha256`, `sha384`,
//! and `sha512` methods to any type implementing [`AsRef<Path>`](std::path::Path).
//!
//! # Features
//!
//! At least one of the following features must be enabled:
//!
//! - `async` - Async implementation using tokio
//! - `sync` - Blocking implementation
//!
//! When both are enabled, the async trait is exported at the crate root and the blocking trait is
//! available as `sync::Sha2Hasher`.
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

#[cfg(feature = "async")]
mod r#async;
#[cfg(feature = "async")]
pub use r#async::Sha2Hasher;

#[cfg(feature = "sync")]
pub mod sync;
#[cfg(all(feature = "sync", not(feature = "async")))]
pub use sync::Sha2Hasher;
