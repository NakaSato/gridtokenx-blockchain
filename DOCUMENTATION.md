# GridTokenX Blockchain Documentation

## Overview

GridTokenX is a blockchain-based peer-to-peer energy trading platform built on the Substrate framework. The platform enables decentralized energy trading for solar-powered smart grids, allowing prosumers to trade energy tokens directly with consumers while maintaining transparency and security.

## Table of Contents

1. [Architecture](#architecture)
2. [Pallets Overview](#pallets-overview)
3. [Core Components](#core-components)
4. [API Reference](#api-reference)
5. [Development Guide](#development-guide)
6. [Testing](#testing)
7. [Deployment](#deployment)

## Architecture

### System Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                     Frontend Interface                      │
├─────────────────────────────────────────────────────────────┤
│                    GridTokenX Runtime                       │
├─────────────────────────────────────────────────────────────┤
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐         │
│  │   Energy    │  │   Energy    │  │    User     │         │
│  │   Token     │  │   Trade     │  │  Registry   │         │
│  │   Pallet    │  │   Pallet    │  │   Pallet    │         │
│  └─────────────┘  └─────────────┘  └─────────────┘         │
│                                                             │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐         │
│  │   Energy    │  │   Payment   │  │    Price    │         │
│  │  Transfer   │  │ Settlement  │  │ Discovery   │         │
│  │   Pallet    │  │   Pallet    │  │   Pallet    │         │
│  └─────────────┘  └─────────────┘  └─────────────┘         │
├─────────────────────────────────────────────────────────────┤
│                     Substrate Framework                     │
└─────────────────────────────────────────────────────────────┘
```

### Project Structure

```
gridtokenx-blockchain/
├── pallets/                    # Custom pallets (smart contracts)
│   ├── energy-token/          # Energy token management
│   ├── energy-trade/          # P2P trading logic
│   ├── energy-transfer/       # Energy transfer validation
│   ├── payment-settlement/    # Payment processing
│   ├── price-discovery/       # Price optimization
│   └── user-registry/         # Identity management
├── runtime/                   # Chain runtime configuration
├── node/                      # Blockchain node implementation
└── target/                    # Build artifacts
```

## Pallets Overview

### 1. Energy Token Pallet (`pallet-energy-token`)

**Purpose**: Manages energy tokens that represent units of energy production and consumption.

**Key Features**:
- Token minting for energy production
- Token transfer between accounts
- Balance tracking and validation

**Storage**:
- `TokenBalance`: Maps `AccountId` to token balance

**Extrinsics**:
- `mint_tokens(amount)`: Mint new energy tokens
- `transfer(to, amount)`: Transfer tokens between accounts

**Events**:
- `TokensMinted`: Emitted when tokens are minted
- `TokensTransferred`: Emitted when tokens are transferred

### 2. Energy Trade Pallet (`pallet-energy-trade`)

**Purpose**: Facilitates peer-to-peer energy trading through an order book system.

**Key Features**:
- Create ask/bid orders
- Automated order matching
- Trade settlement with verification
- Location-based trading

**Storage**:
- `TradeOrders`: Maps order IDs to trade orders
- `UserOrders`: Maps users to their order IDs

**Extrinsics**:
- `create_ask_order(energy_amount, price_per_unit, grid_location)`: Create a sell order
- `create_bid_order(energy_amount, price_per_unit, grid_location)`: Create a buy order
- `match_orders(ask_id, bid_id)`: Match compatible orders
- `verify_transfer(order_id, verification_data)`: Verify energy transfer
- `complete_trade(order_id)`: Complete a trade

**Events**:
- `AskOrderCreated`: Emitted when a sell order is created
- `BidOrderCreated`: Emitted when a buy order is created
- `OrdersMatched`: Emitted when orders are matched
- `OrderCompleted`: Emitted when a trade is completed

### 3. User Registry Pallet (`pallet-user-registry`)

**Purpose**: Manages user identities, roles, and device registrations.

**Key Features**:
- User registration with role assignment
- Device registration and management
- Role-based access control
- Reputation system

**Storage**:
- `UserProfiles`: Maps `AccountId` to user profiles
- `Devices`: Maps device IDs to device information

**Extrinsics**:
- `register_user(role)`: Register a new user
- `register_device(device_type, max_capacity)`: Register a device
- `update_user_role(account, new_role)`: Update user role (admin only)

**Events**:
- `UserRegistered`: Emitted when a user is registered
- `DeviceRegistered`: Emitted when a device is registered
- `UserUpdated`: Emitted when user information is updated

### 4. Energy Transfer Pallet (`pallet-energy-transfer`)

**Purpose**: Validates and tracks actual energy transfers using IoT device measurements.

**Key Features**:
- Real-time energy transfer monitoring
- IoT device integration
- Transfer verification and validation
- Grid metrics tracking

**Storage**:
- `Transfers`: Maps order IDs to transfer data
- `IoTMeasurements`: Maps order IDs to IoT measurements

**Extrinsics**:
- `start_transfer(order_id, start_time)`: Start an energy transfer
- `record_measurement(order_id, measurement)`: Record IoT measurement
- `complete_transfer(order_id, end_time, final_measurement)`: Complete transfer
- `report_transfer_failure(order_id, reason)`: Report transfer failure

### 5. Payment Settlement Pallet (`pallet-payment-settlement`)

**Purpose**: Handles payment processing for energy trades across different payment methods.

**Key Features**:
- Multiple payment methods (native, fiat, stablecoin)
- Exchange rate management
- Payment verification
- Settlement automation

**Storage**:
- `Payments`: Maps payment IDs to payment information
- `ExchangeRates`: Maps currency pairs to exchange rates

**Extrinsics**:
- `create_payment(order_id, payment_method, external_reference)`: Create payment
- `process_native_payment(payment_id)`: Process native token payment
- `process_external_payment(payment_id, proof)`: Process external payment
- `update_exchange_rate(from_token, to_token, rate)`: Update exchange rates

### 6. Price Discovery Pallet (`pallet-price-discovery`)

**Purpose**: Optimizes energy pricing based on market conditions and grid metrics.

**Key Features**:
- Market data aggregation
- Grid congestion analysis
- Optimal price calculation
- Location-based pricing

**Storage**:
- `MarketDataStore`: Maps locations to market data
- `GridMetricsStore`: Maps locations to grid metrics
- `LocationPriorities`: Maps locations to priority data

**Extrinsics**:
- `update_market_data(location, price, volume)`: Update market data
- `update_grid_metrics(location, congestion, loss_factor, stability)`: Update grid metrics
- `update_location_priorities(source, priorities)`: Update location priorities

## Core Components

### User Roles

```rust
pub enum UserRole {
    Consumer,      // Energy buyers
    Prosumer,      // Energy producers and consumers
    GridOperator,  // Grid management
    Admin,         // System administration
}
```

### Device Types

```rust
pub enum DeviceType {
    SolarPanel,    // Solar energy generation
    Battery,       // Energy storage
    SmartMeter,    // Energy measurement
    Other,         // Other devices
}
```

### Order Status Flow

```
Open → Matched → InTransfer → Completed
  ↓       ↓         ↓
Cancelled → Failed → Failed
```

### Payment Methods

```rust
pub enum PaymentMethod {
    Native,           // Platform's native token
    Fiat,            // Traditional currency
    Stablecoin,      // USD-pegged cryptocurrency
    ExternalToken,   // Other cryptocurrency
}
```

## API Reference

### Energy Token Pallet

#### mint_tokens
```rust
fn mint_tokens(
    origin: OriginFor<T>,
    amount: T::TokenBalance,
) -> DispatchResult
```
Mints new energy tokens to the caller's account.

#### transfer
```rust
fn transfer(
    origin: OriginFor<T>,
    to: T::AccountId,
    amount: T::TokenBalance,
) -> DispatchResult
```
Transfers tokens from caller to specified account.

### Energy Trade Pallet

#### create_ask_order
```rust
fn create_ask_order(
    origin: OriginFor<T>,
    energy_amount: T::TokenBalance,
    price_per_unit: T::TokenBalance,
    grid_location: Vec<u8>,
) -> DispatchResult
```
Creates a sell order for energy.

#### create_bid_order
```rust
fn create_bid_order(
    origin: OriginFor<T>,
    energy_amount: T::TokenBalance,
    price_per_unit: T::TokenBalance,
    grid_location: Vec<u8>,
) -> DispatchResult
```
Creates a buy order for energy.

#### match_orders
```rust
fn match_orders(
    origin: OriginFor<T>,
    ask_id: T::Hash,
    bid_id: T::Hash,
) -> DispatchResult
```
Matches compatible ask and bid orders.

### User Registry Pallet

#### register_user
```rust
fn register_user(
    origin: OriginFor<T>,
    role: UserRole,
) -> DispatchResult
```
Registers a new user with specified role.

#### register_device
```rust
fn register_device(
    origin: OriginFor<T>,
    device_type: DeviceType,
    max_capacity: u32,
) -> DispatchResult
```
Registers a new device (prosumers and grid operators only).

## Development Guide

### Prerequisites

- Rust 1.88+ and Cargo
- Substrate development environment
- Node.js 14+ (for frontend)
- Docker (optional)

### Building the Project

1. Clone the repository:
```bash
git clone <repository-url>
cd gridtokenx-blockchain
```

2. Install Rust and Substrate dependencies:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup update nightly
rustup target add wasm32-unknown-unknown --toolchain nightly
```

3. Build the project:
```bash
cargo build --release
```

### Running the Node

```bash
./target/release/solar-grid-node --dev
```

### Configuration

The runtime configuration is located in `runtime/src/lib.rs`. Key configurations include:

- Block time and finality
- Pallet configurations
- System parameters
- API versions

### Adding New Pallets

1. Create a new pallet directory in `pallets/`
2. Implement the pallet using Substrate's `#[pallet]` macro
3. Add the pallet to `runtime/Cargo.toml`
4. Configure the pallet in `runtime/src/lib.rs`
5. Add the pallet to the `construct_runtime!` macro

## Testing

### Unit Tests

Run unit tests for all pallets:
```bash
cargo test --all
```

Run tests for a specific pallet:
```bash
cargo test -p pallet-energy-token
```

### Integration Tests

Integration tests are located in the `tests/` directory of each pallet. They test the interaction between different pallets.

### Test Coverage

The project includes comprehensive test coverage for:
- Token minting and transfer
- Order creation and matching
- User and device registration
- Payment processing
- Energy transfer validation

### Sample Test Data

```rust
// Example test for energy token minting
#[test]
fn mint_tokens_works() {
    new_test_ext().execute_with(|| {
        let account = 1;
        let amount = 100;

        assert_ok!(EnergyToken::mint_tokens(
            RuntimeOrigin::signed(account),
            amount
        ));
        
        assert_eq!(EnergyToken::token_balance(account), amount);
    });
}
```

## Deployment

### Local Development

1. Build the project:
```bash
cargo build --release
```

2. Run the node:
```bash
./target/release/solar-grid-node --dev
```

### Production Deployment

1. Configure chain specification:
```bash
./target/release/solar-grid-node build-spec --chain staging > chain-spec.json
```

2. Convert to raw format:
```bash
./target/release/solar-grid-node build-spec --chain chain-spec.json --raw > chain-spec-raw.json
```

3. Start validator nodes:
```bash
./target/release/solar-grid-node --chain chain-spec-raw.json --validator
```

### Docker Deployment

```dockerfile
FROM substrate/substrate:latest
COPY ./target/release/solar-grid-node /usr/local/bin/
EXPOSE 9944 9933 30333
CMD ["solar-grid-node", "--dev"]
```

### CI/CD Pipeline

The project includes a GitHub Actions workflow for:
- Code formatting checks
- Linting with clippy
- Building and testing
- Coverage reporting
- Automated deployment

## Security Considerations

### Access Control

- Role-based permissions for sensitive operations
- Multi-signature support for critical functions
- Device ownership verification

### Data Validation

- Input validation for all extrinsics
- Overflow protection for numerical operations
- Proof verification for external payments

### Economic Security

- Slashing mechanisms for malicious behavior
- Reputation system for user credibility
- Economic incentives for honest participation

## Performance Optimization

### Storage Optimization

- Efficient data structures for large-scale operations
- Pruning of historical data
- Indexed lookups for fast queries

### Network Optimization

- Optimized message passing between pallets
- Efficient serialization/deserialization
- Minimal on-chain data storage

## Future Enhancements

### Planned Features

1. **Smart Contracts**: Integration with ink! smart contracts
2. **Cross-chain Support**: Interoperability with other blockchains
3. **Advanced Analytics**: Machine learning for price prediction
4. **Mobile App**: Mobile interface for prosumers
5. **Governance**: On-chain governance for protocol upgrades

### Roadmap

- Q1 2025: Complete core functionality
- Q2 2025: Security audit and mainnet preparation
- Q3 2025: Mainnet launch
- Q4 2025: Cross-chain integration

## Contributing

### Code Style

- Follow Rust conventions
- Use `cargo fmt` for formatting
- Pass all `cargo clippy` checks
- Write comprehensive tests

### Submission Process

1. Fork the repository
2. Create a feature branch
3. Implement changes with tests
4. Submit a pull request
5. Address review feedback

### Issue Reporting

- Use GitHub issues for bug reports
- Include reproduction steps
- Provide system information
- Tag issues appropriately

## License

This project is licensed under the MIT License. See the LICENSE file for details.

## Support

For support and questions:
- GitHub Issues: Report bugs and feature requests
- Documentation: Refer to this documentation
- Community: Join our Discord/Telegram channels

---

*Last Updated: July 2025*
*Version: 0.1.0*
