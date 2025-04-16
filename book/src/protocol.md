# The Chaum-Pedersen Protocol

The Chaum-Pedersen protocol is a zero-knowledge proof protocol that allows a prover to convince a verifier that they know a discrete logarithm without revealing the logarithm itself.

## Mathematical Background

The protocol is based on the discrete logarithm problem in a cyclic group. Given a group $G$ with generator $g$ and order $q$, and an element $h \in G$, the prover wants to prove knowledge of $x$ such that:

$$
h = g^x
$$

without revealing $x$.

## Protocol Steps

1. **Setup**:
   - Prover and verifier agree on a group $G$ with generator $g$
   - Prover has a secret $x$ and computes $h = g^x$

2. **Commitment**:
   - Prover chooses a random $k$ and computes:
     - $r_1 = g^k$
     - $r_2 = h^k$
   - Prover sends $(r_1, r_2)$ to verifier

3. **Challenge**:
   - Verifier chooses a random challenge $c$
   - Verifier sends $c$ to prover

4. **Response**:
   - Prover computes $s = k + c \cdot x \mod q$
   - Prover sends $s$ to verifier

5. **Verification**:
   - Verifier checks:
     - $g^s = r_1 \cdot h^c$
     - $h^s = r_2 \cdot (g^x)^c$

## Security Properties

The protocol satisfies three important properties:

1. **Completeness**: If the prover knows $x$, the verifier will always accept the proof.

2. **Soundness**: If the prover does not know $x$, they cannot convince the verifier except with negligible probability.

3. **Zero-Knowledge**: The verifier learns nothing about $x$ beyond the fact that the prover knows it.

## Implementation Details

In CPZKp, the protocol is implemented for both scalar groups and elliptic curves:

### Scalar Groups

For scalar groups, the operations are simple modular exponentiations:

```rust
// Commitment
let r1 = g.modpow(&k, &p);
let r2 = h.modpow(&k, &p);

// Response
let s = (k + c * x) % q;

// Verification
let lhs1 = g.modpow(&s, &p);
let rhs1 = (r1 * h.modpow(&c, &p)) % p;
let lhs2 = h.modpow(&s, &p);
let rhs2 = (r2 * g.modpow(&(c * x), &p)) % p;
```

### Elliptic Curves

For elliptic curves, the operations are point multiplications:

```rust
// Commitment
let r1 = g.scale(k);
let r2 = h.scale(k);

// Response
let s = (k + c * x) % q;

// Verification
let lhs1 = g.scale(s);
let rhs1 = r1.add(&h.scale(c));
let lhs2 = h.scale(s);
let rhs2 = r2.add(&g.scale(c * x));
```

## Security Considerations

- The random value $k$ must be chosen uniformly from $[0, q-1]$
- The challenge $c$ should be large enough to prevent brute-force attacks
- The group order $q$ should be a large prime
- The implementation should be constant-time to prevent timing attacks 