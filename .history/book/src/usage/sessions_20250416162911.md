# Multi-Round Sessions

CPZKp supports multi-round sessions for more complex zero-knowledge proof protocols.

## Basic Usage

### 1. Creating a Session

```rust
use cpzkp::{Session, ScalarGroup};

let group = ScalarGroup;
let mut session = Session::new(&group)?;
```

### 2. Adding Rounds

```rust
// Add a round
session.add_round()?;

// Add multiple rounds
for _ in 0..3 {
    session.add_round()?;
}
```

### 3. Generating Proofs

```rust
// Generate proof for a specific round
let proof = session.generate_proof(0)?;

// Generate proofs for all rounds
let proofs = session.generate_all_proofs()?;
```

### 4. Verifying Proofs

```rust
// Verify a specific round
let valid = session.verify_proof(0, &proof)?;

// Verify all rounds
let all_valid = session.verify_all_proofs()?;
```

## Advanced Usage

### 1. Custom Round Configuration

```rust
use cpzkp::{Session, RoundConfig};

let config = RoundConfig {
    challenge_bits: 256,
    random_bits: 512,
    timeout: Some(Duration::from_secs(30)),
};

let mut session = Session::with_config(&group, config)?;
```

### 2. Parallel Processing

```rust
use rayon::prelude::*;

// Generate proofs in parallel
let proofs: Vec<_> = (0..session.round_count())
    .into_par_iter()
    .map(|i| session.generate_proof(i))
    .collect::<Result<_, _>>()?;

// Verify proofs in parallel
let results: Vec<_> = proofs.par_iter()
    .enumerate()
    .map(|(i, proof)| session.verify_proof(i, proof))
    .collect::<Result<_, _>>()?;
```

### 3. Error Handling

```rust
match session.generate_proof(0) {
    Ok(proof) => println!("Proof generated: {:?}", proof),
    Err(ZkpError::InvalidRound) => println!("Invalid round number"),
    Err(ZkpError::Timeout) => println!("Operation timed out"),
    Err(e) => println!("Other error: {}", e),
}
```

## Example Applications

### 1. Multi-Factor Authentication

```rust
struct MultiFactorAuth {
    session: Session,
    factors: Vec<BigUint>,
}

impl MultiFactorAuth {
    fn new(group: &ScalarGroup, factor_count: usize) -> Result<Self, Box<dyn std::error::Error>> {
        let mut session = Session::new(group)?;
        
        // Add rounds for each factor
        for _ in 0..factor_count {
            session.add_round()?;
        }
        
        // Generate factors
        let factors = (0..factor_count)
            .map(|_| group.generate_random(256))
            .collect::<Result<Vec<_>, _>>()?;
        
        Ok(Self { session, factors })
    }
    
    fn authenticate(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        // Generate proofs for each factor
        let proofs = self.session.generate_all_proofs()?;
        
        // Verify all proofs
        self.session.verify_all_proofs()
    }
}
```

### 2. Batch Processing

```rust
struct BatchProcessor {
    session: Session,
    batch_size: usize,
}

impl BatchProcessor {
    fn new(group: &ScalarGroup, batch_size: usize) -> Result<Self, Box<dyn std::error::Error>> {
        let mut session = Session::new(group)?;
        
        // Add rounds for batch processing
        for _ in 0..batch_size {
            session.add_round()?;
        }
        
        Ok(Self { session, batch_size })
    }
    
    fn process_batch(&mut self, inputs: &[BigUint]) -> Result<Vec<bool>, Box<dyn std::error::Error>> {
        assert_eq!(inputs.len(), self.batch_size);
        
        // Generate proofs in parallel
        let proofs: Vec<_> = (0..self.batch_size)
            .into_par_iter()
            .map(|i| self.session.generate_proof(i))
            .collect::<Result<_, _>>()?;
        
        // Verify proofs in parallel
        let results: Vec<_> = proofs.par_iter()
            .enumerate()
            .map(|(i, proof)| self.session.verify_proof(i, proof))
            .collect::<Result<_, _>>()?;
        
        Ok(results)
    }
}
```

## Performance Considerations

1. Use appropriate batch sizes
2. Consider parallel processing
3. Cache session state
4. Use timeouts for long-running operations

## Security Considerations

1. Validate all inputs
2. Use secure random number generation
3. Implement proper error handling
4. Consider timing attacks
5. Use appropriate group sizes

## Best Practices

1. Use appropriate error handling
2. Document session state
3. Implement proper cleanup
4. Consider resource usage
5. Test thoroughly 