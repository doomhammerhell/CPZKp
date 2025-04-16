# CLI Tool

CPZKp provides a command-line interface for common operations.

## Installation

Install the CLI tool using Cargo:

```bash
cargo install cpzkp-cli
```

## Basic Commands

### 1. Generate Keys

```bash
cpzkp generate-keys --group scalar
```

Options:
- `--group`: Group type (scalar or elliptic)
- `--output`: Output file path (optional)

### 2. Generate Proof

```bash
cpzkp generate-proof --key-file keys.json --message "Hello, World!"
```

Options:
- `--key-file`: Path to key file
- `--message`: Message to sign
- `--output`: Output file path (optional)

### 3. Verify Proof

```bash
cpzkp verify-proof --proof-file proof.json
```

Options:
- `--proof-file`: Path to proof file
- `--key-file`: Path to key file (optional)

### 4. Batch Operations

```bash
cpzkp batch-verify --proofs-dir proofs/
```

Options:
- `--proofs-dir`: Directory containing proof files
- `--parallel`: Use parallel processing (default: true)

## Configuration

The CLI tool can be configured using a config file:

```toml
[default]
group = "scalar"
output_dir = "output"
parallel = true

[security]
random_bits = 256
challenge_bits = 128
```

## Examples

### 1. Complete Workflow

```bash
# Generate keys
cpzkp generate-keys --group scalar --output keys.json

# Generate proof
cpzkp generate-proof --key-file keys.json --message "Hello, World!" --output proof.json

# Verify proof
cpzkp verify-proof --proof-file proof.json
```

### 2. Batch Processing

```bash
# Generate multiple proofs
for i in {1..10}; do
    cpzkp generate-proof --key-file keys.json --message "Message $i" --output "proofs/proof_$i.json"
done

# Verify all proofs
cpzkp batch-verify --proofs-dir proofs/
```

## Error Handling

The CLI tool provides detailed error messages:

```bash
$ cpzkp generate-keys --group invalid
Error: Invalid group type 'invalid'. Valid options are: scalar, elliptic
```

## Exit Codes

- `0`: Success
- `1`: General error
- `2`: Invalid input
- `3`: File I/O error
- `4`: Verification failed

## Best Practices

1. Always verify proofs after generation
2. Use appropriate group sizes
3. Store keys securely
4. Use batch operations for efficiency
5. Check exit codes in scripts 