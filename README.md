# B58UUID for Rust

[![Crates.io](https://img.shields.io/crates/v/b58uuid.svg)](https://crates.io/crates/b58uuid)
[![Documentation](https://docs.rs/b58uuid/badge.svg)](https://docs.rs/b58uuid)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Build Status](https://github.com/b58uuid/b58uuid-rs/workflows/Tests/badge.svg)](https://github.com/b58uuid/b58uuid-rs/actions)
[![Downloads](https://img.shields.io/crates/d/b58uuid.svg)](https://crates.io/crates/b58uuid)

Base58-encoded UUID library for Rust with minimal dependencies.

🌐 **[Try the online converter at b58uuid.io](https://b58uuid.io)**

## Why This Library?

Convert standard 36-character UUIDs to compact 22-character Base58 format:

- **Significantly shorter** - From 36 to 22 characters: `550e8400-e29b-41d4-a716-446655440000` → `BWBeN28Vb7cMEx7Ym8AUzs`
- **URL-safe** - No special characters that need escaping
- **Unambiguous** - Uses Bitcoin's Base58 alphabet (excludes 0, O, I, l)
- **Fast** - Optimized encoding/decoding with lookup tables
- **Safe** - Memory-safe by design (Rust guarantees)
- **Cross-platform** - Works on Linux, macOS, Windows, iOS, Android, WASM

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
b58uuid = "1.0"
```

## Usage

```rust
use b58uuid;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Generate a new UUID
    let b58 = b58uuid::generate();
    println!("{}", b58); // Output: 3FfGK34vwMvVFDedyb2nkf

    // Encode existing UUID
    let encoded = b58uuid::encode_uuid("550e8400-e29b-41d4-a716-446655440000")?;
    println!("{}", encoded); // Output: BWBeN28Vb7cMEx7Ym8AUzs

    // Decode back to UUID
    let uuid = b58uuid::decode_to_uuid("BWBeN28Vb7cMEx7Ym8AUzs")?;
    println!("{}", uuid); // Output: 550e8400-e29b-41d4-a716-446655440000

    Ok(())
}
```

## API

### Functions

- `generate() -> String` - Generate a new random UUID and return Base58 encoding
- `encode_uuid(uuid_str: &str) -> Result<String, B58UUIDError>` - Encode UUID string to Base58
- `decode_to_uuid(b58_str: &str) -> Result<String, B58UUIDError>` - Decode Base58 string to UUID
- `encode(data: &[u8; 16]) -> String` - Encode 16-byte array to Base58
- `decode(b58_str: &str) -> Result<[u8; 16], B58UUIDError>` - Decode Base58 string to 16-byte array

### Error Type

`B58UUIDError` enum with variants:
- `InvalidUUID(String)` - Invalid UUID format
- `InvalidBase58(String)` - Invalid Base58 string
- `InvalidLength { expected: usize, got: usize }` - Invalid length
- `Overflow` - Arithmetic overflow during conversion

## Features

- **Minimal dependencies**: Only uses `getrandom` for secure random number generation
- **Always 22 characters**: Consistent, predictable output length
- **Bitcoin Base58 alphabet**: No confusing characters (0, O, I, l)
- **Memory safe**: Rust's compile-time guarantees prevent common bugs
- **Full error handling**: All operations return `Result` types
- **Cross-platform**: Works on all major platforms including embedded systems

## Dependencies

This library has only **one dependency**: [`getrandom`](https://crates.io/crates/getrandom)

### Why `getrandom`?

Rust's standard library intentionally does not provide cryptographically secure random number generation. The `getrandom` crate is the de facto standard solution in the Rust ecosystem for this purpose.

**Benefits of using `getrandom`**:
- ✅ **Cryptographically secure** on all platforms (Linux, macOS, Windows, iOS, Android, WASM)
- ✅ **Minimal overhead**: Small, well-audited crate with no transitive dependencies
- ✅ **Widely trusted**: Used by the `rand` crate and thousands of other projects
- ✅ **Platform abstraction**: Automatically uses the best random source for each platform:
  - Linux: `getrandom()` syscall
  - macOS/BSD: `/dev/urandom`
  - Windows: `BCryptGenRandom()`
  - WASM: `crypto.getRandomValues()`

Without `getrandom`, we would need to manually implement platform-specific code for each operating system, which would be error-prone and harder to maintain.

## Testing

```bash
cargo test
```

## Other Language Implementations

B58UUID is available in multiple languages:

- **Go**: [b58uuid-go](https://github.com/b58uuid/b58uuid-go) - Available on [pkg.go.dev](https://pkg.go.dev/github.com/b58uuid/b58uuid-go)
- **JavaScript/TypeScript**: [b58uuid-js](https://github.com/b58uuid/b58uuid-js) - Available on [npm](https://www.npmjs.com/package/b58uuid)
- **Python**: [b58uuid-py](https://github.com/b58uuid/b58uuid-py) - Available on [PyPI](https://pypi.org/project/b58uuid/)
- **Java**: [b58uuid-java](https://github.com/b58uuid/b58uuid-java) - Available on [Maven Central](https://central.sonatype.com/artifact/io.b58uuid/b58uuid)

## Resources

- 🌐 [Online Converter](https://b58uuid.io) - Try B58UUID in your browser
- 📚 [Documentation](https://docs.rs/b58uuid)
- 🐛 [Report Issues](https://github.com/b58uuid/b58uuid-rs/issues)

## License

MIT License - see LICENSE file for details.
