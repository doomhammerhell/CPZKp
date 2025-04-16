# Python Bindings

CPZKp provides Python bindings through PyO3 for easy integration with Python applications.

## Installation

### 1. From PyPI

```bash
pip install cpzkp
```

### 2. From Source

```bash
git clone https://github.com/yourusername/cpzkp.git
cd cpzkp
pip install .
```

## Basic Usage

### 1. Initialization

```python
from cpzkp import ScalarGroup

# Initialize the group
group = ScalarGroup()
```

### 2. Basic Operations

```python
# Get group parameters
p, q, g, h = group.get_constants()

# Generate random number
random = group.generate_random(256)

# Scale a point
scaled = group.scale(g, random)

# Add points
sum = group.add(g, h)

# Double a point
doubled = group.double(g)
```

### 3. Zero-Knowledge Proofs

```python
# Generate a proof
k = group.generate_random(256)
r1 = group.scale(g, k)
r2 = group.scale(h, k)

# Generate challenge
c = group.generate_challenge()

# Generate response
s = group.solve_zk_challenge_s(k, c, x)

# Verify proof
valid = group.verify_zk_proof(g, h, y1, y2, c, s)
```

## Advanced Usage

### 1. Error Handling

```python
from cpzkp import ZkpError

try:
    random = group.generate_random(0)
except ZkpError as e:
    print(f"Error: {e}")
```

### 2. Serialization

```python
# Serialize
serialized = group.serialize_point(point)

# Deserialize
deserialized = group.deserialize_point(serialized)
```

### 3. Batch Operations

```python
# Batch verification
proofs = [(g, h, y1, y2, c, s) for _ in range(10)]
results = group.batch_verify(proofs)
```

## Example Applications

### 1. Authentication System

```python
from cpzkp import ScalarGroup
from typing import Dict, Tuple

class AuthenticationSystem:
    def __init__(self):
        self.group = ScalarGroup()
        self.user_secrets: Dict[str, int] = {}
        self.user_public_keys: Dict[str, Tuple[int, int]] = {}
    
    def register_user(self, username: str) -> Tuple[int, int]:
        x = self.group.generate_random(256)
        self.user_secrets[username] = x
        
        p, q, g, h = self.group.get_constants()
        y1 = self.group.scale(g, x)
        y2 = self.group.scale(h, x)
        
        self.user_public_keys[username] = (y1, y2)
        return (y1, y2)
    
    def authenticate(self, username: str) -> bool:
        if username not in self.user_secrets:
            return False
            
        x = self.user_secrets[username]
        y1, y2 = self.user_public_keys[username]
        
        # Generate proof
        k = self.group.generate_random(256)
        r1 = self.group.scale(g, k)
        r2 = self.group.scale(h, k)
        
        # Generate challenge
        c = self.group.generate_challenge()
        
        # Generate response
        s = self.group.solve_zk_challenge_s(k, c, x)
        
        # Verify proof
        return self.group.verify_zk_proof(g, h, y1, y2, c, s)
```

### 2. Digital Signatures

```python
from cpzkp import ScalarGroup
import hashlib

class DigitalSignature:
    def __init__(self):
        self.group = ScalarGroup()
        self.private_key = self.group.generate_random(256)
        p, q, g, h = self.group.get_constants()
        self.public_key = self.group.scale(g, self.private_key)
    
    def sign(self, message: bytes) -> Tuple[int, int]:
        # Hash the message
        m = int.from_bytes(hashlib.sha256(message).digest(), 'big')
        
        # Generate proof
        k = self.group.generate_random(256)
        r1 = self.group.scale(g, k)
        r2 = self.group.scale(h, k)
        
        # Generate challenge
        c = self.group.generate_challenge()
        
        # Generate response
        s = self.group.solve_zk_challenge_s(k, c, self.private_key)
        
        return (c, s)
    
    def verify(self, message: bytes, signature: Tuple[int, int]) -> bool:
        c, s = signature
        
        # Hash the message
        m = int.from_bytes(hashlib.sha256(message).digest(), 'big')
        
        # Verify proof
        return self.group.verify_zk_proof(g, h, self.public_key, m, c, s)
```

## Performance Considerations

1. Use appropriate group sizes
2. Consider using batch operations
3. Cache group parameters
4. Use parallel processing for large computations

## Security Considerations

1. Always validate inputs
2. Use secure random number generation
3. Handle errors appropriately
4. Follow security best practices

## Best Practices

1. Use type hints for better code clarity
2. Implement proper error handling
3. Document your code
4. Write unit tests
5. Consider performance implications 