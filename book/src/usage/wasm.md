# WebAssembly Support

CPZKp can be compiled to WebAssembly for use in web applications.

## Setup

### 1. Install Dependencies

```bash
cargo install wasm-pack
```

### 2. Add WebAssembly Support

Add the following to your `Cargo.toml`:

```toml
[lib]
crate-type = ["cdylib", "rlib"]

[dependencies]
wasm-bindgen = "0.2"
js-sys = "0.3"
web-sys = { version = "0.3", features = ["console"] }
```

## Basic Usage

### 1. JavaScript Integration

```javascript
import init, { ZkpClient } from 'cpzkp';

async function main() {
    await init();
    const client = new ZkpClient();
    
    // Generate keys
    const keys = await client.generate_keys();
    console.log('Public keys:', keys);
    
    // Generate proof
    const proof = await client.generate_proof('Hello, World!');
    console.log('Proof:', proof);
    
    // Verify proof
    const valid = await client.verify_proof(proof);
    console.log('Valid:', valid);
}
```

### 2. TypeScript Support

```typescript
import init, { ZkpClient } from 'cpzkp';

interface Keys {
    y1: string;
    y2: string;
}

interface Proof {
    c: string;
    s: string;
}

async function main(): Promise<void> {
    await init();
    const client = new ZkpClient();
    
    const keys: Keys = await client.generate_keys();
    const proof: Proof = await client.generate_proof('Hello, World!');
    const valid: boolean = await client.verify_proof(proof);
}
```

## Advanced Usage

### 1. Error Handling

```javascript
try {
    const proof = await client.generate_proof('Hello, World!');
} catch (error) {
    console.error('Error:', error);
}
```

### 2. Custom Configuration

```javascript
const client = new ZkpClient({
    group: 'scalar',
    randomBits: 256,
    challengeBits: 128
});
```

### 3. Batch Operations

```javascript
const proofs = await Promise.all(
    messages.map(msg => client.generate_proof(msg))
);

const results = await client.batch_verify(proofs);
```

## Performance Considerations

1. Use `wasm-pack build --release` for production builds
2. Consider using Web Workers for heavy computations
3. Cache results when possible
4. Use batch operations for multiple proofs

## Security Considerations

1. Always validate inputs on both JavaScript and Rust sides
2. Use appropriate group sizes
3. Implement proper error handling
4. Consider using Web Crypto API for additional security

## Example Application

### 1. HTML Setup

```html
<!DOCTYPE html>
<html>
<head>
    <title>CPZKp Web Demo</title>
</head>
<body>
    <script type="module">
        import init, { ZkpClient } from './pkg/cpzkp.js';
        
        async function main() {
            await init();
            const client = new ZkpClient();
            
            // Your application code here
        }
        
        main().catch(console.error);
    </script>
</body>
</html>
```

### 2. React Integration

```jsx
import React, { useState, useEffect } from 'react';
import init, { ZkpClient } from 'cpzkp';

function App() {
    const [client, setClient] = useState(null);
    const [proof, setProof] = useState(null);
    
    useEffect(() => {
        async function setup() {
            await init();
            setClient(new ZkpClient());
        }
        setup();
    }, []);
    
    const handleGenerateProof = async () => {
        try {
            const newProof = await client.generate_proof('Hello, World!');
            setProof(newProof);
        } catch (error) {
            console.error('Error:', error);
        }
    };
    
    return (
        <div>
            <button onClick={handleGenerateProof}>
                Generate Proof
            </button>
            {proof && <pre>{JSON.stringify(proof, null, 2)}</pre>}
        </div>
    );
}
```

## Best Practices

1. Always initialize the WebAssembly module before use
2. Handle errors appropriately
3. Use TypeScript for better type safety
4. Consider performance implications
5. Test thoroughly in different browsers 