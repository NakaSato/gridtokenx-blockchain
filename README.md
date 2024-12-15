# SolarGridX - Blockchain-based P2P Energy Trading Platform

A decentralized platform facilitating peer-to-peer energy trading for solar-powered smart grids using Substrate framework.

## Project Structure

```
blockchain/
├── pallets/            # Substrate pallets (smart contracts)
│   ├── energy-token/   # Energy token management
│   ├── energy-trade/   # P2P trading logic
│   └── user-registry/  # Identity management
├── runtime/            # Chain runtime configuration
└── node/              # Blockchain node implementation
```

## Prerequisites

- Rust 1.70+ and Cargo
- Substrate development environment
- Node.js 14+ (for frontend)
- Docker (optional)

## Features

- Decentralized energy trading using Substrate pallets
- Real-time energy measurement and tokenization
- Automated trade settlements
- Secure identity management
- Integration with IoT devices

## Smart Contract Pallets

1. Energy Token Pallet

   - Mint tokens for energy production
   - Transfer tokens between accounts
   - Query token balances

2. Energy Trade Pallet (WIP)

   - Create and manage trade orders
   - Match buyers with sellers
   - Settlement mechanism

3. User Registry Pallet (WIP)
   - Identity management
   - Role-based access control
   - Device registration

## Development Setup

1. Install Rust:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2. Install Substrate prerequisites:

```bash
rustup update nightly
rustup target add wasm32-unknown-unknown --toolchain nightly
```

3. Build the project:

```bash
cargo build --release
```

## Testing

Run the test suite:

```bash
cargo test --all
```

## Contributing

1. Fork the repository
2. Create a feature branch
3. Commit your changes
4. Push to the branch
5. Create a Pull Request
