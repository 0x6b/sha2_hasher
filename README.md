# sha2_hasher

A trait for hashing a file using the SHA-2 family of algorithms.

I found myself repeatedly writing code to hash files using the SHA256, so I wrote this tiny trait to make my life easier.

## Crate Features

- `async`: Enables the async implementation.
- `sync`: Enables the sync implementation.

At least one implementation feature must be enabled. When both are enabled, the async trait is
available at the crate root and the sync trait is available as `sha2_hasher::sync::Sha2Hasher`.

## Usage

### Async

```rust,ignore
// Enable with: --features async
use sha2_hasher::Sha2Hasher;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let hash = std::path::Path::new(".gitignore").sha256().await.unwrap();
    println!("{hash}");
}
```

### Sync

```rust,ignore
// Enable with: --features sync
use sha2_hasher::Sha2Hasher;

fn main() {
    let hash = std::path::Path::new(".gitignore").sha256().unwrap();
    println!("{hash}");
}
```

## License

MIT. See [LICENSE](LICENSE) for details.
