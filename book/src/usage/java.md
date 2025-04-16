# Java Bindings

CPZKp provides Java bindings through JNI (Java Native Interface) for integrating zero-knowledge proof functionality into Java applications.

## Setup

Add the following dependency to your `pom.xml`:

```xml
<dependency>
    <groupId>com.cpzkp</groupId>
    <artifactId>cpzkp</artifactId>
    <version>0.1.0</version>
</dependency>
```

Make sure the native library is available in your system's library path.

## Basic Usage

### Initialization

```java
import com.cpzkp.CPZKP;

public class Main {
    public static void main(String[] args) {
        try (CPZKP zkp = new CPZKP()) {
            // Use CPZKp here
        }
    }
}
```

### Basic Operations

```java
// Generate key pair
KeyPair keyPair = zkp.generateKey();
Point publicKey = keyPair.getPublicKey();
Point privateKey = keyPair.getPrivateKey();

// Create proof
Proof proof = zkp.createProof(privateKey);

// Verify proof
boolean isValid = zkp.verifyProof(publicKey, proof);
```

### Serialization

```java
// Serialize point
byte[] serialized = publicKey.serialize();

// Deserialize point
Point deserialized = Point.deserialize(serialized);
```

## Advanced Usage

### Error Handling

```java
try {
    KeyPair keyPair = zkp.generateKey();
    // Use key pair
} catch (RuntimeException e) {
    // Handle error
    System.err.println("Error: " + e.getMessage());
}
```

### Resource Management

The Java bindings use `AutoCloseable` and implement proper resource management:

```java
try (CPZKP zkp = new CPZKP()) {
    // Use CPZKp
    // Resources will be automatically freed
}
```

## Example Applications

### Authentication System

```java
public class AuthenticationSystem {
    private final CPZKP zkp;
    private final Map<String, Point> userPublicKeys;

    public AuthenticationSystem() {
        this.zkp = new CPZKP();
        this.userPublicKeys = new HashMap<>();
    }

    public void registerUser(String username, Point publicKey) {
        userPublicKeys.put(username, publicKey);
    }

    public boolean authenticate(String username, Proof proof) {
        Point publicKey = userPublicKeys.get(username);
        if (publicKey == null) {
            return false;
        }
        return zkp.verifyProof(publicKey, proof);
    }

    @Override
    public void close() throws Exception {
        zkp.close();
    }
}
```

### Digital Signatures

```java
public class DigitalSignature {
    private final CPZKP zkp;
    private final KeyPair keyPair;

    public DigitalSignature() {
        this.zkp = new CPZKP();
        this.keyPair = zkp.generateKey();
    }

    public Proof sign(byte[] message) {
        // In a real implementation, you would hash the message
        // and use it to generate the proof
        return zkp.createProof(keyPair.getPrivateKey());
    }

    public boolean verify(byte[] message, Proof proof) {
        return zkp.verifyProof(keyPair.getPublicKey(), proof);
    }

    @Override
    public void close() throws Exception {
        zkp.close();
    }
}
```

## Performance Considerations

1. **Resource Management**: Always use try-with-resources or explicitly close CPZKp instances.
2. **Native Calls**: Minimize the number of native calls by batching operations when possible.
3. **Memory Usage**: Be mindful of memory usage when serializing/deserializing points.

## Security Considerations

1. **Input Validation**: Always validate inputs before passing them to CPZKp methods.
2. **Error Handling**: Properly handle exceptions and errors.
3. **Resource Cleanup**: Ensure resources are properly cleaned up to prevent memory leaks.

## Best Practices

1. **Use AutoCloseable**: Always use try-with-resources with CPZKp instances.
2. **Error Handling**: Implement proper error handling for all operations.
3. **Input Validation**: Validate all inputs before passing them to CPZKp methods.
4. **Testing**: Test your implementation thoroughly, especially error cases.
5. **Documentation**: Document your usage of CPZKp in your codebase. 