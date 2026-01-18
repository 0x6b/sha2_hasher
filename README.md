# sha2_hasher

A trait for hashing a file using the SHA-2 family of algorithms.

I found myself repeatedly writing code to hash files using the SHA256, so I wrote this tiny trait to make my life easier.

## Crate Features

- `async`: Enables the async implementation.
- `sync`: Enables the sync implementation.

Note: `async` and `sync` features are mutually exclusive. One must be enabled.

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
