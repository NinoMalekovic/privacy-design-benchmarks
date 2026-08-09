# Privacy Designs

## Overview

This sandbox provides plug-and-play implementations of four distinct privacy designs for blockchain and cryptographic systems. Each implementation demonstrates a different approach to transaction graph privacy, from obfuscation to complete elimination.

## Four Privacy Architectures

### 1. Monero-like Design: Obfuscation
**Goal**: Hide transaction relationships through cryptographic obfuscation

**Key Features**:
- Ring signatures (CLSAG-like) for input ambiguity
- Stealth addresses for one-time outputs
- Pedersen commitments for hidden amounts
- Key images for double-spend prevention

**What's Hidden**: 
- ✅ Actual sender (hidden among ring members)
- ✅ Actual recipient (stealth addresses)
- ✅ Transaction amounts (Pedersen commitments)
- ❌ Transaction graph still exists but is difficult to analyze

**Technology**: C++ with Crypto++ library

### 2. Zcash-like Design: Encryption
**Goal**: Shield transaction details within a private pool

**Key Features**:
- Shielded transaction pool
- Commitment trees for state verification
- Nullifier set for replay prevention
- Zero-knowledge proofs for validity

**What's Hidden**:
- ✅ Sender/receiver relationships
- ✅ Transaction amounts
- ✅ Complete transaction graph within shielded pool
- ❌ Some metadata may still be visible (e.g., pool size, timing)

**Technology**: Rust with SHA2, BLS12-381

### 3. Chaumian E-cash Design: Elimination
**Goal**: Remove the public transaction graph entirely

**Key Features**:
- Blind signatures for token issuance
- Off-chain token transfers
- No persistent public history
- Token redemption only on-chain

**What's Hidden**:
- ✅ Complete transaction graph (no public record)
- ✅ Transfer history (off-chain)
- ✅ Identity of token holders
- ❌ Token issuance and redemption are public

**Technology**: Rust with blind signature implementation

### 4. ZK General-purpose Privacy: Computation
**Goal**: Private arbitrary state transitions and computations

**Key Features**:
- General-purpose zero-knowledge proofs
- Private state transitions
- Verifiable computation without revealing inputs
- Multiple DSL support (Noir, Circom, ZoKrates)

**What's Hidden**:
- ✅ Private inputs to computations
- ✅ State transition details
- ✅ Identity and application-specific data
- ✅ Results can be verified without revealing computation details

**Technology**: Noir, Circom, ZoKrates with proving backends

## Quick Start

### Prerequisites

- Docker and Docker Compose
- Git
- 8GB+ RAM recommended
- 20GB+ disk space

### Clone and Build

```bash
# Clone the repository
git clone https://github.com/yourusername/privacy-sandbox.git
cd privacy-sandbox

# Build all containers
docker-compose build

# Run all demos
docker-compose up
```

### Run Individual Demos

```bash
# Monero-like implementation
docker-compose up monero-like

# Zcash-like implementation
docker-compose up zcash-like

# Chaumian E-cash implementation
docker-compose up chaumian-ecash

# Noir ZK implementation
docker-compose up noir

# Circom ZK implementation
docker-compose up circom

# ZoKrates ZK implementation
docker-compose up zokrates
```

## Implementation Details

### Directory Structure

```
privacy-sandbox/
├── monero-like/
│   ├── src/
│   │   └── main.cpp           # C++ implementation
│   ├── CMakeLists.txt
│   └── Dockerfile
├── zcash-like/
│   ├── src/
│   │   └── main.rs            # Rust implementation
│   ├── Cargo.toml
│   └── Dockerfile
├── chaumian-ecash/
│   ├── src/
│   │   └── main.rs            # Rust implementation
│   ├── Cargo.toml
│   └── Dockerfile
├── zk-general/
│   ├── noir/
│   │   ├── src/
│   │   │   └── main.nr        # Noir circuit
│   │   ├── Nargo.toml
│   │   └── Dockerfile
│   ├── circom/
│   │   ├── circuit.circom     # Circom circuit
│   │   ├── input.json
│   │   └── Dockerfile
│   └── zokrates/
│       ├── square.zok         # ZoKrates program
│       ├── prove.sh
│       └── Dockerfile
└── docker-compose.yml
```

### Technology Stack

| Component | Implementation | Language | Key Libraries |
|-----------|---------------|----------|---------------|
| Monero-like | Ring signatures, stealth addresses | C++ | Crypto++ |
| Zcash-like | Shielded transactions, commitment trees | Rust | SHA2, BLS12-381 |
| Chaumian E-cash | Blind signatures, off-chain transfers | Rust | Standard library |
| Noir | General ZK circuits | Noir | Nargo |
| Circom | ZK circuit design | Circom | SnarkJS |
| ZoKrates | ZK proof creation | ZoKrates DSL | ZoKrates |

## Core Concepts

### Privacy Models Comparison

```
Monero       → Obscure transaction graph
Zcash        → Encrypt transaction graph
Chaumian     → Eliminate transaction graph
ZK Systems   → Generalize private computation
```

### Architectural Evolution

1. **Obfuscation Layer** (Monero)
   - Makes relationships difficult to infer
   - Graph still exists publicly
   - Computational efficiency

2. **Encryption Layer** (Zcash)
   - Hides relationships behind ZK proofs
   - Graph exists but is shielded
   - Strong privacy guarantees

3. **Elimination Layer** (Chaumian)
   - No public graph to analyze
   - Off-chain transfers
   - Perfect graph privacy

4. **Generalization Layer** (ZK)
   - Privacy for any computation
   - Arbitrary state transitions
   - Application-agnostic

## Security Considerations

### Monero-like
- ✅ Ring size affects anonymity set
- ✅ Key images prevent double-spending
- ⚠️ Timing analysis may leak information
- ⚠️ Ring signature size grows with anonymity

### Zcash-like
- ✅ Strong cryptographic guarantees
- ✅ Trusted setup (if using Groth16)
- ⚠️ Shielded pool adoption affects privacy
- ⚠️ Metadata leakage possible

### Chaumian E-cash
- ✅ Perfect graph privacy
- ✅ No on-chain transfer history
- ⚠️ Requires trust in issuer
- ⚠️ Double-spend detection challenges

### ZK General-purpose
- ✅ Arbitrary privacy guarantees
- ✅ Verifiable computation
- ⚠️ Proving time and complexity
- ⚠️ Trusted setup requirements

## Performance Characteristics

| Design | Proof Size | Verification Time | Proving Time | Memory Usage |
|--------|------------|-------------------|--------------|--------------|
| Monero-like | Medium | Fast | Fast | Low |
| Zcash-like | Small | Fast | Medium | Medium |
| Chaumian E-cash | Very Small | Very Fast | Very Fast | Low |
| ZK General | Varies | Fast | Slow | High |

## Extending the Sandbox

### Adding New Features

1. **Monero-like Extensions**:
```cpp
// Add new ring signature schemes
class CLSAGSignature : public RingSignature {
    // Implement CLSAG
};

// Add Bulletproofs for range proofs
class BulletproofRangeProof {
    // Implement range proofs
};
```

2. **Zcash-like Extensions**:
```rust
// Implement Sapling or Orchard circuits
struct SaplingShieldedTransaction {
    // Add Sapling-specific fields
}
```

3. **Chaumian E-cash Extensions**:
```rust
// Implement threshold issuance
struct ThresholdBlindSignature {
    // Add threshold blind signatures
}
```

4. **ZK General Extensions**:
```circom
// Create custom circuits
template CustomPrivacyCircuit() {
    // Add your privacy logic
}
```

## Testing

```bash
# Run all tests
cargo test --all
make test  # For C++ components

# Run specific implementation tests
cargo test -p zcash-like
cd monero-like && make test

# Verify ZK proofs
cd zk-general/noir && nargo test
cd zk-general/circom && snarkjs groth16 verify
cd zk-general/zokrates && zokrates verify
```

## Common Issues and Solutions

### Docker Build Failures
```bash
# Clear Docker cache
docker system prune -a

# Increase Docker resources
# Docker Desktop -> Settings -> Resources
# Memory: 8GB, Swap: 2GB
```

### ZK Proving Performance
```bash
# Use optimized proving keys
# For Groth16: Use compiled circuits
circom circuit.circom --r1cs --wasm --sym --c

# For PLONK: Use Barretenberg
# See Barretenberg documentation
```

### Memory Issues
```bash
# Reduce circuit complexity
# Use smaller ring sizes for Monero
# Use smaller pool sizes for Zcash
# Use simpler circuits for ZK
```

## Contributing

1. Fork the repository
2. Create a feature branch
3. Add your implementation
4. Test thoroughly
5. Submit a pull request

### Contribution Guidelines
- Maintain coding standards
- Add comprehensive comments
- Include test coverage
- Document new features
- Update README accordingly

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## References

### Academic Papers
- Monero: "Ring Confidential Transactions" (Shen Noether, 2015)
- Zcash: "Zerocash: Decentralized Anonymous Payments" (Sasson et al., 2014)
- Chaumian E-cash: "Blind Signatures for Untraceable Payments" (Chaum, 1982)
- ZK Systems: "How to Prove a Theorem So No One Else Can Claim It" (Goldwasser et al., 1985)

### Implementations
- [Monero](https://github.com/monero-project/monero)
- [Zcash](https://github.com/zcash/zcash)
- [Noir](https://github.com/noir-lang/noir)
- [Circom](https://github.com/iden3/circom)
- [ZoKrates](https://github.com/Zokrates/ZoKrates)

### Documentation
- [Crypto++ Reference](https://www.cryptopp.com/docs/ref/)
- [Rust Cryptography](https://docs.rs/rust-crypto/)
- [ZKProof Standards](https://zkproof.org/)

## Acknowledgments

- Monero Research Lab
- Zcash Foundation
- Ethereum Foundation (ZK research)
- Privacy & Scaling Explorations

## Support

For questions and support:
- Open an issue in the GitHub repository
- Contact the maintainers
- Join the community discussions

## Roadmap

- [x] Basic implementations of all four designs
- [x] Docker containerization
- [x] Documentation
- [ ] Production-ready optimizations
- [ ] Benchmarks and comparisons
- [ ] Web interface for testing
- [ ] Integration with other privacy technologies

---

*This sandbox is intended for educational and research purposes. For production use, please refer to the official implementations and security audits of each technology.*
