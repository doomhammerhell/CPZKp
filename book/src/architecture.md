# Architecture

CPZKp is designed with a modular architecture that allows for easy extension and customization. The library is organized into several key components:

## Core Components

### 1. Group Operations (`GroupOps`)

The `GroupOps` trait defines the basic operations that can be performed on group elements:

```rust
pub trait GroupOps {
    type Point;
    type Scalar;

    fn prime(&self) -> BigUint;
    fn order(&self) -> BigUint;
    fn generator(&self) -> Self::Point;
    fn second_generator(&self) -> Self::Point;
}
```

### 2. Point Operations (`PointOps`)

The `PointOps` trait defines operations specific to group points:

```rust
pub trait PointOps {
    fn is_on_curve(&self) -> bool;
    fn double(&self) -> Self;
    fn scale(&self, scalar: &BigUint) -> Self;
    fn add(&self, other: &Self) -> Self;
}
```

### 3. ZKP Operations (`ZkpOps`)

The `ZkpOps` trait defines the zero-knowledge proof operations:

```rust
pub trait ZkpOps {
    fn generate_challenge(&self) -> Result<BigUint>;
    fn solve_zk_challenge_s(&self, k: &BigUint, c: &BigUint, x: &BigUint) -> Result<BigUint>;
    fn verify_zk_proof(
        &self,
        g: &Self::Point,
        h: &Self::Point,
        y1: &Self::Point,
        y2: &Self::Point,
        c: &BigUint,
        s: &BigUint,
    ) -> Result<bool>;
}
```

## Implementations

### 1. Scalar Group

The `ScalarGroup` implementation provides operations for scalar groups:

```rust
pub struct ScalarGroup;

impl GroupOps for ScalarGroup {
    type Point = BigUint;
    type Scalar = BigUint;
    // ...
}
```

### 2. Elliptic Curve

The `EllipticCurve` implementation provides operations for elliptic curves:

```rust
pub struct EllipticCurve;

impl GroupOps for EllipticCurve {
    type Point = Point;
    type Scalar = BigUint;
    // ...
}
```

## Key Features

### 1. Serialization

The library provides serialization and deserialization for points:

```rust
pub trait PointSerialization {
    fn serialize_point(&self, point: &Self::Point) -> Result<Vec<u8>>;
    fn deserialize_point(&self, data: &[u8]) -> Result<Self::Point>;
}
```

### 2. Error Handling

The library uses a custom error type for consistent error handling:

```rust
#[derive(Debug, Error)]
pub enum ZkpError {
    #[error("Invalid point: {0}")]
    InvalidPoint(String),
    #[error("Serialization error: {0}")]
    SerializationError(String),
    // ...
}
```

### 3. Random Number Generation

Secure random number generation is provided through the `generate_random` function:

```rust
pub fn generate_random(bits: usize) -> Result<BigUint> {
    // ...
}
```

## Extensions

The library includes several extensions:

1. **CLI Tool**: A command-line interface for generating and verifying proofs
2. **WebAssembly Support**: WASM bindings for browser-based applications
3. **Python Bindings**: Python integration through PyO3
4. **Ethereum Integration**: Support for Ethereum-based applications
5. **Interactive Playground**: A web-based playground for experimenting with proofs

## Security Considerations

The architecture is designed with security in mind:

1. **Constant-time Operations**: All operations are implemented to be constant-time
2. **Secure Randomness**: Uses cryptographically secure random number generation
3. **Input Validation**: All inputs are validated before processing
4. **Error Handling**: Comprehensive error handling to prevent information leakage 