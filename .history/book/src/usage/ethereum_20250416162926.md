# Ethereum Integration

CPZKp can be integrated with Ethereum smart contracts for decentralized zero-knowledge proof verification.

## Setup

### 1. Install Dependencies

```bash
cargo add cpzkp-ethereum
```

### 2. Add to Cargo.toml

```toml
[dependencies]
cpzkp-ethereum = "0.1.0"
```

## Basic Usage

### 1. Smart Contract Integration

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "cpzkp/contracts/ZkpVerifier.sol";

contract MyContract is ZkpVerifier {
    function verifyProof(
        uint256[2] memory a,
        uint256[2][2] memory b,
        uint256[2] memory c,
        uint256[2] memory input
    ) public view returns (bool) {
        return verify(a, b, c, input);
    }
}
```

### 2. Rust Integration

```rust
use cpzkp_ethereum::{EthereumGroup, Proof};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let group = EthereumGroup;
    
    // Generate proof
    let proof = group.generate_proof()?;
    
    // Serialize for Ethereum
    let serialized = proof.serialize_for_ethereum()?;
    
    // Verify on Ethereum
    let valid = group.verify_on_ethereum(&serialized)?;
    
    Ok(())
}
```

## Advanced Usage

### 1. Custom Verification

```rust
use cpzkp_ethereum::{EthereumGroup, CustomVerifier};

struct MyVerifier {
    group: EthereumGroup,
    contract_address: Address,
}

impl MyVerifier {
    fn new(contract_address: Address) -> Self {
        Self {
            group: EthereumGroup,
            contract_address,
        }
    }
    
    fn verify_custom(&self, proof: &Proof) -> Result<bool, Box<dyn std::error::Error>> {
        let serialized = proof.serialize_for_ethereum()?;
        
        // Call custom verification function
        self.group.verify_custom(&serialized, self.contract_address)
    }
}
```

### 2. Batch Verification

```rust
use cpzkp_ethereum::{EthereumGroup, BatchVerifier};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let group = EthereumGroup;
    let verifier = BatchVerifier::new(group);
    
    // Generate multiple proofs
    let proofs = (0..10)
        .map(|_| group.generate_proof())
        .collect::<Result<Vec<_>, _>>()?;
    
    // Verify in batch
    let results = verifier.verify_batch(&proofs)?;
    
    Ok(())
}
```

## Example Applications

### 1. Private Token Transfers

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "cpzkp/contracts/ZkpVerifier.sol";
import "@openzeppelin/contracts/token/ERC20/ERC20.sol";

contract PrivateToken is ERC20, ZkpVerifier {
    mapping(bytes32 => bool) public spentNullifiers;
    
    function transferPrivate(
        uint256[2] memory a,
        uint256[2][2] memory b,
        uint256[2] memory c,
        uint256[2] memory input,
        bytes32 nullifier
    ) public {
        require(!spentNullifiers[nullifier], "Nullifier already spent");
        require(verify(a, b, c, input), "Invalid proof");
        
        spentNullifiers[nullifier] = true;
        // Transfer logic
    }
}
```

### 2. Voting System

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "cpzkp/contracts/ZkpVerifier.sol";

contract Voting is ZkpVerifier {
    mapping(bytes32 => bool) public voted;
    mapping(uint256 => uint256) public votes;
    
    function vote(
        uint256[2] memory a,
        uint256[2][2] memory b,
        uint256[2] memory c,
        uint256[2] memory input,
        bytes32 nullifier,
        uint256 choice
    ) public {
        require(!voted[nullifier], "Already voted");
        require(verify(a, b, c, input), "Invalid proof");
        
        voted[nullifier] = true;
        votes[choice] += 1;
    }
}
```

## Performance Considerations

1. Gas optimization
2. Batch processing
3. Efficient proof generation
4. Smart contract optimization

## Security Considerations

1. Nullifier management
2. Input validation
3. Replay protection
4. Front-running protection
5. Gas limit considerations

## Best Practices

1. Use appropriate gas limits
2. Implement proper error handling
3. Consider gas costs
4. Test thoroughly
5. Document contract interfaces 