# Test Suite Documentation

This directory contains the test suite for CPZKp, organized into three main categories:

## Directory Structure

```
tests/
├── unit/           # Unit tests for individual modules
├── integration/    # Integration tests for component interaction
├── property/       # Property-based tests using proptest
└── README.md       # This documentation
```

## Test Categories

### 1. Unit Tests (`unit/`)

Unit tests focus on testing individual components in isolation:
- `scalar_tests.rs`: Tests for scalar group operations
- `ecc_tests.rs`: Tests for elliptic curve operations
- `session_tests.rs`: Tests for session management

### 2. Integration Tests (`integration/`)

Integration tests verify the interaction between components:
- `session_tests.rs`: Tests for multi-round sessions
- `serialization_tests.rs`: Tests for data serialization
- `group_tests.rs`: Tests for group operations

### 3. Property Tests (`property/`)

Property-based tests using proptest to verify mathematical properties:
- `scalar_proptest.rs`: Properties of scalar operations
- `ecc_proptest.rs`: Properties of elliptic curve operations
- `zkp_proptest.rs`: Properties of zero-knowledge proofs

## Running Tests

### All Tests

```bash
cargo test
```

### Specific Test Categories

```bash
# Unit tests only
cargo test --test unit

# Integration tests only
cargo test --test integration

# Property tests only
cargo test --test property
```

### With Verbose Output

```bash
cargo test -- --nocapture
```

## Test Coverage

To generate test coverage reports:

```bash
cargo tarpaulin --all-features
```

## Adding New Tests

When adding new tests:
1. Place them in the appropriate category directory
2. Follow the existing test patterns
3. Include proper documentation
4. Add #[should_panic] tests for error cases
5. Use proptest for property-based testing when appropriate

## Best Practices

1. **Documentation**
   - Include module-level documentation
   - Document test cases
   - Explain complex assertions

2. **Error Testing**
   - Test error cases with #[should_panic]
   - Verify error messages
   - Test edge cases

3. **Property Testing**
   - Test mathematical properties
   - Use proptest for random inputs
   - Verify invariants

4. **Integration Testing**
   - Test component interaction
   - Verify serialization
   - Test error propagation

## Questions?

If you have questions about the test suite, please open an issue or contact the maintainers. 