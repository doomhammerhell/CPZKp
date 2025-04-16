# Basic Usage

This chapter covers the basic usage of CPZKp for common operations.

## Installation

Add CPZKp to your project's dependencies:

```toml
[dependencies]
cpzkp = "0.1.0"
num-bigint = "0.4"
```

## Basic Operations

### 1. Initializing a Group

```rust
use cpzkp::ScalarGroup;

let group = ScalarGroup;
```

### 2. Getting Group Parameters

```rust
let (p, q, g, h) = group.get_constants()?;
```

### 3. Generating Random Numbers

```rust
let random = group.generate_random(256)?;
```

### 4. Point Operations

```rust
// Scale a point
let scaled = group.scale(&g, &random)?;

// Add points
let sum = group.add(&g, &h)?;

// Double a point
let doubled = group.double(&g)?;
```

### 5. Zero-Knowledge Proofs

```rust
// Generate a proof
let k = group.generate_random(256)?;
let r1 = group.scale(&g, &k)?;
let r2 = group.scale(&h, &k)?;

// Generate challenge
let c = group.generate_challenge()?;

// Generate response
let s = group.solve_zk_challenge_s(&k, &c, &x)?;

// Verify proof
let valid = group.verify_zk_proof(&g, &h, &y1, &y2, &c, &s)?;
```

## Error Handling

CPZKp uses a custom error type for consistent error handling:

```rust
use cpzkp::ZkpError;

match group.generate_random(0) {
    Ok(_) => println!("Success"),
    Err(ZkpError::InvalidInput(_)) => println!("Invalid input"),
    Err(e) => println!("Other error: {}", e),
}
```

## Serialization

Points can be serialized and deserialized:

```rust
// Serialize
let serialized = group.serialize_point(&point)?;

// Deserialize
let deserialized = group.deserialize_point(&serialized)?;
```

## Best Practices

1. Always use appropriate error handling
2. Validate inputs before processing
3. Use secure random number generation
4. Follow security best practices
5. Test your code thoroughly 