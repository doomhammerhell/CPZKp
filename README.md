# Zero-Knowledge Proof Authentication Library

A Rust library implementing a zero-knowledge proof authentication system based on the Chaum-Pedersen protocol. This library supports both scalar (multiplicative) groups and elliptic curve groups (secp256k1).

## Features

- Zero-knowledge proof authentication using Chaum-Pedersen protocol
- Support for both scalar and elliptic curve groups
- Efficient serialization for network transfer
- Comprehensive test suite including property-based tests
- Performance benchmarks
- Thread-safe and async-ready

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
zkp_auth = "0.1.0"
```

## Quick Start

```rust
use zkp_auth::{Group, Point, get_constants, solve_zk_challenge_s};
use num_bigint::BigUint;

// Select the group type (scalar or elliptic curve)
let group = Group::Scalar;

// Get system parameters
let (p, q, g, h) = get_constants(&group);

// Generate a secret
let x_secret = BigUint::from(1234u32);
let k = BigUint::from(5678u32);
let c = BigUint::from(910u32);

// Solve the ZK challenge
let s = solve_zk_challenge_s(&x_secret, &k, &c, &q);
```

## Usage Examples

### Scalar Group Authentication

```rust
use zkp_auth::{Group, Point, get_constants, exponentiates_points, solve_zk_challenge_s};

// Initialize system parameters
let group = Group::Scalar;
let (p, q, g, h) = get_constants(&group);

// Prover's secret
let x_secret = BigUint::from(300u32);

// Generate public values
let (y1, y2) = exponentiates_points(&x_secret, &g, &h, &p).unwrap();

// Generate proof
let k = BigUint::from(10u32);
let (r1, r2) = exponentiates_points(&k, &g, &h, &p).unwrap();

// Verifier generates challenge
let c = BigUint::from(894u32);

// Prover solves challenge
let s = solve_zk_challenge_s(&x_secret, &k, &c, &q);

// Verify the proof
let verification = verify(&r1, &r2, &y1, &y2, &g, &h, &c, &s, &p).unwrap();
assert!(verification);
```

### Elliptic Curve Group Authentication

```rust
use zkp_auth::{Group, Point, get_constants, exponentiates_points, solve_zk_challenge_s};

// Initialize system parameters with elliptic curve group
let group = Group::EllipticCurve;
let (p, q, g, h) = get_constants(&group);

// Rest of the process is similar to scalar group
// but uses elliptic curve operations internally
```

## Running Tests

Run the standard test suite:
```bash
cargo test
```

Run property-based tests:
```bash
cargo test --features="proptest"
```

Run benchmarks:
```bash
cargo bench
```

## Security Considerations

- This library is for educational purposes and should be thoroughly audited before production use
- The random number generation should be replaced with cryptographically secure alternatives in production
- The elliptic curve implementation uses the secp256k1 curve, commonly used in Bitcoin

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details.