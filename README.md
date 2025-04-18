# CPZKp - Chaum-Pedersen Zero-Knowledge Proofs

[![CI/CD](https://github.com/doomhammerhell/CPZKp/actions/workflows/ci.yml/badge.svg)](https://github.com/doomhammerhell/CPZKp/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/cpzkp.svg)](https://crates.io/crates/cpzkp)
[![Documentation](https://docs.rs/cpzkp/badge.svg)](https://docs.rs/cpzkp)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A comprehensive implementation of Chaum-Pedersen Zero-Knowledge Proofs with multiple interfaces and practical applications.

## Features

- Support for scalar (multiplicative) group operations
- Support for secp256k1 elliptic curve operations
- Support for Curve25519 (optional feature)
- Multi-round session support
- WebAssembly support
- Python bindings
- CLI tool
- Comprehensive benchmarks
- Web demo application
- Ethereum integration example
- Interactive documentation

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
cpzkp = { version = "0.1", features = ["curve25519"] }  # Optional features
```

## Usage

### Basic Usage

```rust
use cpzkp::{Group, Point, get_constants, solve_zk_challenge_s};
use num_bigint::BigUint;

// Select the group type
let group = Group::Scalar;

// Get the system parameters
let (p, q, g, h) = get_constants(&group).unwrap();

// Generate a random secret
let x_secret = BigUint::from(1234u32);
let k = BigUint::from(5678u32);
let c = BigUint::from(910u32);

// Solve the ZK challenge
let s = solve_zk_challenge_s(&x_secret, &k, &c, &q);
```

### CLI Tool

```bash
# Generate a key pair
cpzkp gen-key --group scalar --output keypair.json

# Generate a proof
cpzkp prove --msg "Hello, World!" --keypair keypair.json --output proof.json

# Verify a proof
cpzkp verify --proof proof.json --keypair keypair.json
```

### WebAssembly

```javascript
import { KeyPair, Proof } from 'cpzkp';

// Generate a key pair
const keypair = await KeyPair.new('scalar');

// Generate a proof
const proof = await Proof.generate(keypair, 'Hello, World!');

// Verify the proof
const isValid = await proof.verify();
```

### Python Bindings

```python
from cpzkp import KeyPair, Proof

# Generate a key pair
keypair = KeyPair('scalar')

# Generate a proof
proof = Proof.generate(keypair, 'Hello, World!')

# Verify the proof
is_valid = proof.verify()
```

### Multi-Round Sessions

```rust
use cpzkp::{Group, Session};

// Create a new session
let mut session = Session::new(Group::Scalar).unwrap();

// Start a new round
let (r1, r2) = session.next_round().unwrap();

// Solve the challenge
let s = session.solve_challenge(0, &challenge).unwrap();

// Verify the round
let is_valid = session.verify_round(0).unwrap();

// Finalize the session
session.finalize().unwrap();
```

### Ethereum Integration

```rust
use cpzkp::ethereum_integration;

// Generate an Ethereum wallet and ZKP
let (wallet, proof) = ethereum_integration::generate_proof().await?;

// Verify the proof
let is_valid = proof.verify()?;
```

## Web Demo

Try the interactive web demo:

```bash
cd examples/webapp
docker build -t cpzkp-webapp .
docker run -p 8080:80 cpzkp-webapp
```

Then open http://localhost:8080 in your browser.

## Features

### Optional Features

- `scalar`: Enable scalar group operations (default)
- `ecc`: Enable elliptic curve operations
- `curve25519`: Enable Curve25519 support
- `wasm`: Enable WebAssembly support
- `python`: Enable Python bindings
- `ethereum`: Enable Ethereum integration

### Benchmarks

Run the benchmarks with:

```bash
cargo bench
```

The benchmarks measure:
- Scalar operations
- ECC operations
- Verification time
- Serialization/deserialization

## Documentation

The complete documentation is available at:
- [API Documentation](https://docs.rs/cpzkp)
- [Book](https://doomhammerhell.github.io/CPZKp)

## Security

This library is intended for educational purposes. For production use, please consult with a cryptographer and perform a security audit.

## License

MIT