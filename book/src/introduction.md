# Introduction

CPZKp is a comprehensive implementation of Chaum-Pedersen Zero-Knowledge Proofs (ZKP) in Rust. This library provides a secure and efficient way to implement zero-knowledge authentication systems.

## What are Zero-Knowledge Proofs?

Zero-knowledge proofs are cryptographic protocols that allow one party (the prover) to prove to another party (the verifier) that they know a secret value without revealing any information about the secret itself.

The Chaum-Pedersen protocol is a specific type of zero-knowledge proof that allows proving knowledge of a discrete logarithm without revealing the logarithm itself.

## Features

- Support for scalar (multiplicative) group operations
- Support for secp256k1 elliptic curve operations
- Support for Curve25519 (optional feature)
- Multi-round session support
- WebAssembly support
- Python bindings
- CLI tool
- Comprehensive benchmarks

## Quick Start

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

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
cpzkp = { version = "0.1", features = ["curve25519"] }  # Optional features
```

## Security Considerations

This library is intended for educational purposes. For production use, please consult with a cryptographer and perform a security audit.

## License

This project is licensed under the MIT License - see the [LICENSE](https://github.com/doomhammerhell/CPZKp/blob/main/LICENSE) file for details. 