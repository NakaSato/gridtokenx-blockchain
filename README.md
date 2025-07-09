# GridTokenX - Blockchain-based P2P Energy Trading Platform

A comprehensive decentralized platform facilitating peer-to-peer energy trading for solar-powered smart grids using the Substrate framework. GridTokenX enables prosumers to trade energy tokens directly with consumers while maintaining transparency, security, and real-time settlement.

## 🚀 Features

- **Decentralized Energy Trading**: Peer-to-peer energy marketplace using blockchain technology
- **Energy Tokenization**: Real-time conversion of energy production into tradeable tokens
- **Smart Order Matching**: Automated matching of buyers and sellers with optimal pricing
- **IoT Integration**: Real-time energy measurement and validation using IoT devices
- **Multi-Payment Support**: Native tokens, fiat, stablecoins, and external cryptocurrencies
- **Role-Based Access Control**: Secure identity management for different user types
- **Price Discovery**: Dynamic pricing based on market conditions and grid metrics
- **Energy Transfer Validation**: Cryptographic verification of actual energy transfers

## 📁 Project Structure

```
gridtokenx-blockchain/
├── pallets/                    # Custom Substrate pallets
│   ├── energy-token/          # ⚡ Energy token management
│   ├── energy-trade/          # 🏪 P2P trading marketplace
│   ├── energy-transfer/       # 🔄 Energy transfer validation
│   ├── payment-settlement/    # 💰 Multi-payment processing
│   ├── price-discovery/       # 📊 Dynamic pricing engine
│   └── user-registry/         # 👥 Identity & device management
├── runtime/                   # 🏗️ Blockchain runtime configuration
├── node/                      # 🌐 Blockchain node implementation
└── docs/                      # 📚 Documentation
```

## 🛠️ Technology Stack

- **Blockchain Framework**: Substrate 4.0+
- **Programming Language**: Rust 1.88+
- **Consensus**: Aura (Block Production) + GRANDPA (Finality)
- **Database**: RocksDB
- **Networking**: libp2p
- **Frontend**: React.js with Polkadot.js
- **Testing**: Native Rust testing + Substrate test framework

## 📚 Documentation

| Document | Description |
|----------|-------------|
| [📖 DOCUMENTATION.md](./DOCUMENTATION.md) | Complete system documentation |
| [🏗️ ARCHITECTURE.md](./ARCHITECTURE.md) | System architecture and design |
| [🔧 SETUP_GUIDE.md](./SETUP_GUIDE.md) | Development environment setup |
| [🚀 API_DOCUMENTATION.md](./API_DOCUMENTATION.md) | Comprehensive API reference |
| [🧪 TESTING_GUIDE.md](./TESTING_GUIDE.md) | Testing strategies and guidelines |

## 🚦 Quick Start

### Prerequisites

- **Rust** 1.88+ and Cargo
- **Node.js** 14+ (for frontend development)
- **Git** for version control
- **Docker** (optional, for containerized development)

### Installation

1. **Clone the repository:**
```bash
git clone https://github.com/your-org/gridtokenx-blockchain.git
cd gridtokenx-blockchain
```

2. **Install Rust and Substrate dependencies:**
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup update nightly
rustup target add wasm32-unknown-unknown --toolchain nightly
```

3. **Build the project:**
```bash
cargo build --release
```

4. **Run the development node:**
```bash
./target/release/solar-grid-node --dev
```

### Development Environment

For detailed setup instructions, see [SETUP_GUIDE.md](./SETUP_GUIDE.md).

## 🧪 Testing

### Run Tests

```bash
# Run all tests
cargo test --all

# Run tests for specific pallet
cargo test -p pallet-energy-token

# Run with coverage
cargo tarpaulin --out html
```

### Benchmarking

```bash
# Build with benchmarking features
cargo build --release --features runtime-benchmarks

# Run benchmarks
./target/release/solar-grid-node benchmark pallet \
    --pallet "*" \
    --extrinsic "*"
```

## 🏛️ System Architecture

### Core Components

1. **Energy Token Pallet** - Manages energy tokens and balances
2. **Energy Trade Pallet** - Handles order creation and matching
3. **User Registry Pallet** - Manages user identities and devices
4. **Energy Transfer Pallet** - Validates physical energy transfers
5. **Payment Settlement Pallet** - Processes payments across multiple methods
6. **Price Discovery Pallet** - Optimizes pricing based on market conditions

### User Roles

- **Consumer**: Energy buyers
- **Prosumer**: Energy producers and consumers
- **Grid Operator**: Grid infrastructure management
- **Admin**: System administration

### Trading Flow

```
User Registration → Device Registration → Energy Production → 
Token Minting → Order Creation → Order Matching → 
Energy Transfer → Transfer Validation → Payment Settlement → 
Trade Completion
```

## 🔗 API Reference

### Energy Token Pallet

```rust
// Mint energy tokens
mint_tokens(amount: TokenBalance) -> DispatchResult

// Transfer tokens between accounts
transfer(to: AccountId, amount: TokenBalance) -> DispatchResult
```

### Energy Trade Pallet

```rust
// Create sell order
create_ask_order(energy_amount: TokenBalance, price_per_unit: TokenBalance, location: Vec<u8>) -> DispatchResult

// Create buy order
create_bid_order(energy_amount: TokenBalance, price_per_unit: TokenBalance, location: Vec<u8>) -> DispatchResult

// Match compatible orders
match_orders(ask_id: Hash, bid_id: Hash) -> DispatchResult
```

For complete API documentation, see [API_DOCUMENTATION.md](./API_DOCUMENTATION.md).

## 🔐 Security Features

- **Role-based Access Control**: Granular permissions for different user types
- **Cryptographic Verification**: Secure proof of energy transfers
- **Input Validation**: Comprehensive validation of all user inputs
- **Economic Security**: Stake-based incentive mechanisms
- **Overflow Protection**: Safe arithmetic operations

## 📊 Performance Metrics

- **Block Time**: 6 seconds
- **Transaction Finality**: ~12 seconds (2 blocks)
- **Throughput**: ~1000 TPS (theoretical)
- **Storage**: Optimized for large-scale operations

## 🛣️ Roadmap

### Phase 1: Core Infrastructure ✅
- [x] Basic pallet structure
- [x] Energy token management
- [x] User registry system
- [x] Order book implementation

### Phase 2: Advanced Features 🚧
- [x] Energy transfer validation
- [x] Payment settlement system
- [x] Price discovery engine
- [ ] Frontend interface

### Phase 3: Production Ready 📅
- [ ] Security audit
- [ ] Performance optimization
- [ ] Mainnet deployment
- [ ] Mobile application

### Phase 4: Ecosystem Expansion 🔮
- [ ] Cross-chain integration
- [ ] Smart contract support
- [ ] DeFi integrations
- [ ] Governance system

## 🤝 Contributing

We welcome contributions from the community! Please see our contributing guidelines:

1. **Fork** the repository
2. **Create** a feature branch (`git checkout -b feature/amazing-feature`)
3. **Commit** your changes (`git commit -m 'Add amazing feature'`)
4. **Push** to the branch (`git push origin feature/amazing-feature`)
5. **Open** a Pull Request

### Development Guidelines

- Follow Rust best practices and conventions
- Write comprehensive tests for new features
- Update documentation for API changes
- Ensure all tests pass before submitting PR

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🆘 Support

- **Documentation**: Check our comprehensive docs above
- **Issues**: Report bugs via [GitHub Issues](https://github.com/your-org/gridtokenx-blockchain/issues)
- **Discussions**: Join our [GitHub Discussions](https://github.com/your-org/gridtokenx-blockchain/discussions)
- **Community**: Join our Discord server [link]

## 🙏 Acknowledgments

- **Substrate Team** for the excellent blockchain framework
- **Polkadot Ecosystem** for the development tools
- **Energy Trading Community** for domain expertise
- **Open Source Contributors** for their valuable contributions

---

**GridTokenX** - Powering the future of decentralized energy trading 🌱⚡🔗

*Built with ❤️ by the GridTokenX Team*
