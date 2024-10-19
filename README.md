# sha2_hasher

A trait for hashing a file using the SHA-2 family of algorithms.

I found myself repeatedly writing code to hash files using the SHA256, so I wrote this tiny trait to make my life easier.

## Crate Features

The following crate features are available:

- `async`: Enables the `async` feature. Default.
- `sync`: Enables the `sync` feature.
- `sha224`: Enables the `sha224` method.
- `sha256`: Enables the `sha256` method. Default.
- `sha384`: Enables the `sha384` method.
- `sha512`: Enables the `sha512` method.

## Usage

### Async

```rust
use sha2_hasher::Sha2Hasher;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let hash = std::path::Path::new(".gitignore").sha256().await.unwrap();
    println!("{hash}");
}
```

### Sync

```rust
// you have to enable `sync` feature
use sha2_hasher::sync::Sha2Hasher;

fn main() {
    let hash = std::path::Path::new(".gitignore").sha256().unwrap();
    println!("{hash}");
}
```

## License

MIT. See [LICENSE](LICENSE) for details.
