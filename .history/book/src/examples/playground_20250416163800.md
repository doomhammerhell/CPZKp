# Interactive Playground

This chapter demonstrates how to create an interactive playground for experimenting with CPZKp.

## Overview

The playground provides a web-based interface for:
- Generating and verifying zero-knowledge proofs
- Experimenting with different parameters
- Visualizing cryptographic operations
- Learning about zero-knowledge proofs

## Project Structure

```
playground/
├── Cargo.toml
├── index.html
├── src/
│   └── lib.rs
└── style.css
```

## Implementation

### Cargo.toml

```toml
[package]
name = "cpzkp-playground"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
cpzkp = { path = "../../" }
wasm-bindgen = "0.2"
serde = { version = "1.0", features = ["derive"] }
serde-wasm-bindgen = "0.4"
console_error_panic_hook = "0.1"
```

### lib.rs

```rust
use wasm_bindgen::prelude::*;
use cpzkp::*;
use serde::{Serialize, Deserialize};

#[wasm_bindgen]
pub struct Playground {
    group: ScalarGroup,
}

#[wasm_bindgen]
impl Playground {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        console_error_panic_hook::set_once();
        Self {
            group: ScalarGroup::new(),
        }
    }

    pub fn generate_keys(&self) -> JsValue {
        let (secret, public) = self.group.generate_keys();
        serde_wasm_bindgen::to_value(&(secret, public)).unwrap()
    }

    pub fn create_proof(&self, secret: u64, public: u64) -> JsValue {
        let proof = self.group.create_proof(secret, public);
        serde_wasm_bindgen::to_value(&proof).unwrap()
    }

    pub fn verify_proof(&self, proof: JsValue) -> bool {
        let proof: Proof = serde_wasm_bindgen::from_value(proof).unwrap();
        self.group.verify(&proof)
    }
}
```

### index.html

```html
<!DOCTYPE html>
<html>
<head>
    <title>CPZKp Playground</title>
    <link rel="stylesheet" href="style.css">
</head>
<body>
    <div class="container">
        <h1>CPZKp Playground</h1>
        
        <div class="section">
            <h2>Key Generation</h2>
            <button id="generate-keys">Generate Keys</button>
            <div id="keys-output"></div>
        </div>

        <div class="section">
            <h2>Proof Generation</h2>
            <div class="input-group">
                <input type="number" id="secret" placeholder="Secret">
                <input type="number" id="public" placeholder="Public Key">
            </div>
            <button id="create-proof">Create Proof</button>
            <div id="proof-output"></div>
        </div>

        <div class="section">
            <h2>Proof Verification</h2>
            <button id="verify-proof">Verify Proof</button>
            <div id="verification-output"></div>
        </div>
    </div>

    <script type="module">
        import init, { Playground } from './pkg/cpzkp_playground.js';

        async function run() {
            await init();
            const playground = new Playground();

            // Key generation
            document.getElementById('generate-keys').addEventListener('click', () => {
                const keys = playground.generate_keys();
                document.getElementById('keys-output').textContent = JSON.stringify(keys, null, 2);
            });

            // Proof generation
            document.getElementById('create-proof').addEventListener('click', () => {
                const secret = parseInt(document.getElementById('secret').value);
                const public = parseInt(document.getElementById('public').value);
                const proof = playground.create_proof(secret, public);
                document.getElementById('proof-output').textContent = JSON.stringify(proof, null, 2);
            });

            // Proof verification
            document.getElementById('verify-proof').addEventListener('click', () => {
                const proof = JSON.parse(document.getElementById('proof-output').textContent);
                const result = playground.verify_proof(proof);
                document.getElementById('verification-output').textContent = 
                    result ? "Proof is valid!" : "Proof is invalid!";
            });
        }

        run();
    </script>
</body>
</html>
```

### style.css

```css
.container {
    max-width: 800px;
    margin: 0 auto;
    padding: 20px;
}

.section {
    margin-bottom: 30px;
    padding: 20px;
    border: 1px solid #ddd;
    border-radius: 5px;
}

.input-group {
    margin-bottom: 10px;
}

input {
    width: 100%;
    padding: 8px;
    margin-bottom: 10px;
}

button {
    padding: 10px 20px;
    background-color: #4CAF50;
    color: white;
    border: none;
    border-radius: 5px;
    cursor: pointer;
}

button:hover {
    background-color: #45a049;
}

#keys-output,
#proof-output,
#verification-output {
    margin-top: 10px;
    padding: 10px;
    background-color: #f5f5f5;
    border-radius: 5px;
    white-space: pre-wrap;
}
```

## Building and Running

1. Build the project:
```bash
wasm-pack build --target web
```

2. Serve the files:
```bash
python3 -m http.server
```

3. Open in browser:
```
http://localhost:8000
```

## Features

1. **Interactive Interface**
   - Generate keys with a single click
   - Create proofs with custom parameters
   - Verify proofs instantly

2. **Visual Feedback**
   - Clear output formatting
   - Immediate verification results
   - Error handling and display

3. **Educational Value**
   - Experiment with different parameters
   - Understand the proof process
   - Learn about zero-knowledge proofs

## Security Considerations

1. **Client-Side Security**
   - All operations run in the browser
   - No server-side processing
   - Secure random number generation

2. **Input Validation**
   - Validate all user inputs
   - Handle edge cases
   - Prevent invalid operations

## Best Practices

1. **User Experience**
   - Clear instructions
   - Intuitive interface
   - Responsive design

2. **Error Handling**
   - Graceful error recovery
   - Informative error messages
   - Input validation

3. **Performance**
   - Optimize WebAssembly loading
   - Minimize UI updates
   - Efficient proof generation

## Questions?

If you have questions about the playground or want to contribute, please open an issue or contact the maintainers. 