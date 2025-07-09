# Architecture Documentation

## System Overview

GridTokenX is a blockchain-based peer-to-peer energy trading platform built on Substrate. The system enables decentralized energy trading for solar-powered smart grids, allowing prosumers to trade energy tokens directly with consumers.

## High-Level Architecture

```
┌─────────────────────────────────────────────────────────────────────────┐
│                         External Interfaces                            │
├─────────────────────────────────────────────────────────────────────────┤
│  Web App  │  Mobile App  │  IoT Devices  │  Payment Gateways  │  APIs  │
├─────────────────────────────────────────────────────────────────────────┤
│                         Application Layer                              │
├─────────────────────────────────────────────────────────────────────────┤
│                        GridTokenX Runtime                              │
├─────────────────────────────────────────────────────────────────────────┤
│                        Pallet Layer                                    │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐   │
│  │   Energy    │  │   Energy    │  │    User     │  │   Energy    │   │
│  │   Token     │  │   Trade     │  │  Registry   │  │  Transfer   │   │
│  │   Pallet    │  │   Pallet    │  │   Pallet    │  │   Pallet    │   │
│  └─────────────┘  └─────────────┘  └─────────────┘  └─────────────┘   │
│                                                                         │
│  ┌─────────────┐  ┌─────────────┐                                     │
│  │   Payment   │  │    Price    │                                     │
│  │ Settlement  │  │ Discovery   │                                     │
│  │   Pallet    │  │   Pallet    │                                     │
│  └─────────────┘  └─────────────┘                                     │
├─────────────────────────────────────────────────────────────────────────┤
│                    Substrate Framework                                 │
├─────────────────────────────────────────────────────────────────────────┤
│                   Networking & Consensus                               │
├─────────────────────────────────────────────────────────────────────────┤
│                        Database Layer                                  │
└─────────────────────────────────────────────────────────────────────────┘
```

## Runtime Architecture

### Core Components

#### 1. GridTokenX Runtime
- **Location**: `runtime/src/lib.rs`
- **Purpose**: Main runtime configuration and pallet integration
- **Key Features**:
  - Pallet configuration and wiring
  - Runtime API implementations
  - Consensus mechanism setup
  - System parameter configuration

#### 2. Node Implementation
- **Location**: `node/src/`
- **Purpose**: Blockchain node implementation
- **Key Features**:
  - Service configuration
  - Network setup
  - RPC interface
  - Consensus participation

## Pallet Architecture

### Core Pallets

#### 1. Energy Token Pallet
```
┌─────────────────────────────────────────────────┐
│             Energy Token Pallet                │
├─────────────────────────────────────────────────┤
│  Storage:                                       │
│  • TokenBalance: AccountId → Balance            │
│                                                 │
│  Extrinsics:                                    │
│  • mint_tokens(amount)                          │
│  • transfer(to, amount)                         │
│                                                 │
│  Events:                                        │
│  • TokensMinted                                 │
│  • TokensTransferred                            │
└─────────────────────────────────────────────────┘
```

#### 2. Energy Trade Pallet
```
┌─────────────────────────────────────────────────┐
│             Energy Trade Pallet                │
├─────────────────────────────────────────────────┤
│  Storage:                                       │
│  • TradeOrders: Hash → TradeOrder              │
│  • UserOrders: AccountId → Vec<Hash>           │
│                                                 │
│  Extrinsics:                                    │
│  • create_ask_order(amount, price, location)   │
│  • create_bid_order(amount, price, location)   │
│  • match_orders(ask_id, bid_id)                │
│  • verify_transfer(order_id, data)             │
│  • complete_trade(order_id)                    │
│                                                 │
│  Events:                                        │
│  • AskOrderCreated                             │
│  • BidOrderCreated                             │
│  • OrdersMatched                               │
│  • OrderCompleted                              │
└─────────────────────────────────────────────────┘
```

#### 3. User Registry Pallet
```
┌─────────────────────────────────────────────────┐
│            User Registry Pallet                │
├─────────────────────────────────────────────────┤
│  Storage:                                       │
│  • UserProfiles: AccountId → UserProfile       │
│  • Devices: Hash → Device                      │
│                                                 │
│  Extrinsics:                                    │
│  • register_user(role)                         │
│  • register_device(type, capacity)             │
│  • update_user_role(account, role)             │
│                                                 │
│  Events:                                        │
│  • UserRegistered                              │
│  • DeviceRegistered                            │
│  • UserUpdated                                 │
└─────────────────────────────────────────────────┘
```

### Supporting Pallets

#### 4. Energy Transfer Pallet
```
┌─────────────────────────────────────────────────┐
│           Energy Transfer Pallet               │
├─────────────────────────────────────────────────┤
│  Storage:                                       │
│  • Transfers: Hash → TransferData              │
│  • IoTMeasurements: Hash → Vec<Measurement>    │
│                                                 │
│  Extrinsics:                                    │
│  • start_transfer(order_id, start_time)        │
│  • record_measurement(order_id, measurement)   │
│  • complete_transfer(order_id, end_time)       │
│  • report_transfer_failure(order_id, reason)   │
│                                                 │
│  Events:                                        │
│  • TransferStarted                             │
│  • MeasurementRecorded                         │
│  • TransferCompleted                           │
│  • TransferFailed                              │
└─────────────────────────────────────────────────┘
```

#### 5. Payment Settlement Pallet
```
┌─────────────────────────────────────────────────┐
│          Payment Settlement Pallet             │
├─────────────────────────────────────────────────┤
│  Storage:                                       │
│  • Payments: Hash → Payment                    │
│  • ExchangeRates: (Token, Token) → Rate       │
│                                                 │
│  Extrinsics:                                    │
│  • create_payment(order_id, method, ref)       │
│  • process_native_payment(payment_id)          │
│  • process_external_payment(payment_id, proof) │
│  • update_exchange_rate(from, to, rate)        │
│                                                 │
│  Events:                                        │
│  • PaymentCreated                              │
│  • PaymentCompleted                            │
│  • PaymentFailed                               │
│  • ExchangeRateUpdated                         │
└─────────────────────────────────────────────────┘
```

#### 6. Price Discovery Pallet
```
┌─────────────────────────────────────────────────┐
│            Price Discovery Pallet              │
├─────────────────────────────────────────────────┤
│  Storage:                                       │
│  • MarketDataStore: Location → MarketData      │
│  • GridMetricsStore: Location → GridMetrics    │
│  • LocationPriorities: Location → Priorities   │
│                                                 │
│  Extrinsics:                                    │
│  • update_market_data(location, price, volume) │
│  • update_grid_metrics(location, metrics)      │
│  • update_location_priorities(source, prios)   │
│                                                 │
│  Events:                                        │
│  • PriceUpdated                                │
│  • GridMetricsUpdated                          │
│  • OptimalMatchFound                           │
└─────────────────────────────────────────────────┘
```

## Data Flow Architecture

### Energy Trading Flow

```
┌─────────────┐    ┌─────────────┐    ┌─────────────┐    ┌─────────────┐
│    User     │    │   Energy    │    │   Energy    │    │   Energy    │
│ Registration│───▶│    Token    │───▶│    Trade    │───▶│  Transfer   │
│   Pallet    │    │   Pallet    │    │   Pallet    │    │   Pallet    │
└─────────────┘    └─────────────┘    └─────────────┘    └─────────────┘
       │                   │                   │                   │
       │                   │                   │                   │
       ▼                   ▼                   ▼                   ▼
┌─────────────┐    ┌─────────────┐    ┌─────────────┐    ┌─────────────┐
│    Device   │    │   Balance   │    │   Order     │    │   IoT       │
│ Registration│    │   Tracking  │    │   Matching  │    │ Integration │
└─────────────┘    └─────────────┘    └─────────────┘    └─────────────┘
       │                   │                   │                   │
       │                   │                   │                   │
       ▼                   ▼                   ▼                   ▼
┌─────────────┐    ┌─────────────┐    ┌─────────────┐    ┌─────────────┐
│    Price    │    │   Payment   │    │   Final     │    │   Trade     │
│ Discovery   │───▶│ Settlement  │───▶│ Settlement  │───▶│ Completion  │
│   Pallet    │    │   Pallet    │    │             │    │             │
└─────────────┘    └─────────────┘    └─────────────┘    └─────────────┘
```

### Inter-Pallet Communication

#### Direct Dependencies
- **Energy Trade** depends on **Energy Token** for balance checks
- **Energy Transfer** depends on **Energy Trade** for order verification
- **Payment Settlement** depends on **Energy Trade** for order details
- **Price Discovery** depends on **Energy Trade** for market data

#### Event-Driven Communication
- **Energy Trade** emits events consumed by **Energy Transfer**
- **Energy Transfer** emits events consumed by **Payment Settlement**
- **Payment Settlement** emits events consumed by **Energy Trade**

## Storage Architecture

### Database Schema

#### Primary Storage Items

1. **Token Balances**
   - Key: `AccountId`
   - Value: `TokenBalance`
   - Type: `StorageMap`

2. **Trade Orders**
   - Key: `Hash` (order ID)
   - Value: `TradeOrder`
   - Type: `StorageMap`

3. **User Profiles**
   - Key: `AccountId`
   - Value: `UserProfile`
   - Type: `StorageMap`

4. **Device Registry**
   - Key: `Hash` (device ID)
   - Value: `Device`
   - Type: `StorageMap`

#### Secondary Storage Items

1. **User Orders**
   - Key: `AccountId`
   - Value: `Vec<Hash>`
   - Type: `StorageMap`

2. **IoT Measurements**
   - Key: `Hash` (order ID)
   - Value: `Vec<IoTMeasurement>`
   - Type: `StorageMap`

3. **Market Data**
   - Key: `Vec<u8>` (location)
   - Value: `MarketData`
   - Type: `StorageMap`

### Storage Optimization

#### Indexing Strategy
- **Primary Keys**: Direct hash-based lookups
- **Secondary Keys**: Reverse lookups for user orders
- **Composite Keys**: Location-based pricing data

#### Data Pruning
- **Historical Data**: Automatic cleanup of old measurements
- **Completed Orders**: Archive after completion
- **Price History**: Rolling window of market data

## Security Architecture

### Access Control

#### Role-Based Security
```
┌─────────────┐    ┌─────────────┐    ┌─────────────┐    ┌─────────────┐
│    Admin    │    │    Grid     │    │  Prosumer   │    │  Consumer   │
│    Role     │    │  Operator   │    │    Role     │    │    Role     │
└─────────────┘    └─────────────┘    └─────────────┘    └─────────────┘
       │                   │                   │                   │
       │                   │                   │                   │
       ▼                   ▼                   ▼                   ▼
┌─────────────┐    ┌─────────────┐    ┌─────────────┐    ┌─────────────┐
│   System    │    │   Device    │    │   Energy    │    │   Energy    │
│ Management  │    │ Management  │    │ Production  │    │ Consumption │
└─────────────┘    └─────────────┘    └─────────────┘    └─────────────┘
```

#### Permission Matrix
| Role | User Mgmt | Device Mgmt | Trade Creation | Price Setting | Payment Processing |
|------|-----------|-------------|----------------|---------------|--------------------|
| Admin | ✓ | ✓ | ✓ | ✓ | ✓ |
| Grid Operator | ✗ | ✓ | ✓ | ✓ | ✓ |
| Prosumer | ✗ | Own Only | ✓ | ✗ | Own Only |
| Consumer | ✗ | ✗ | ✓ | ✗ | Own Only |

### Input Validation

#### Data Validation Layers
1. **Type Safety**: Rust type system
2. **Range Validation**: Numeric bounds checking
3. **Business Logic**: Domain-specific validation
4. **Cryptographic**: Signature verification

#### Validation Flow
```
Input → Type Check → Range Check → Business Logic → Cryptographic → Execute
  ↓        ↓           ↓             ↓               ↓
Error   Error       Error         Error           Error
```

### Economic Security

#### Incentive Mechanisms
- **Reputation System**: User credibility scoring
- **Stake Requirements**: Economic guarantees
- **Slashing Conditions**: Penalties for misbehavior

#### Attack Prevention
- **Sybil Resistance**: Identity verification
- **DoS Protection**: Rate limiting
- **Economic Attacks**: Collateral requirements

## Consensus Architecture

### Consensus Mechanism

#### Aura Consensus
- **Block Production**: Round-robin authority rotation
- **Block Time**: 6 seconds
- **Finality**: GRANDPA finality gadget

#### Network Topology
```
┌─────────────┐    ┌─────────────┐    ┌─────────────┐
│ Validator 1 │◄──►│ Validator 2 │◄──►│ Validator 3 │
│   (Alice)   │    │    (Bob)    │    │  (Charlie)  │
└─────────────┘    └─────────────┘    └─────────────┘
       ▲                   ▲                   ▲
       │                   │                   │
       ▼                   ▼                   ▼
┌─────────────┐    ┌─────────────┐    ┌─────────────┐
│ Full Node 1 │    │ Full Node 2 │    │ Full Node 3 │
└─────────────┘    └─────────────┘    └─────────────┘
       ▲                   ▲                   ▲
       │                   │                   │
       ▼                   ▼                   ▼
┌─────────────┐    ┌─────────────┐    ┌─────────────┐
│ Light Client│    │ Light Client│    │ Light Client│
│     1       │    │     2       │    │     3       │
└─────────────┘    └─────────────┘    └─────────────┘
```

## API Architecture

### Runtime API

#### Standard APIs
- **Core API**: Basic runtime information
- **Metadata API**: Runtime metadata
- **Block Builder API**: Block construction
- **Tagged Transaction Queue API**: Transaction pool

#### Custom APIs
- **Energy Token API**: Token balance queries
- **Energy Trade API**: Order book queries
- **User Registry API**: User profile queries

### RPC Interface

#### Standard RPC Methods
- `system_*`: System information
- `chain_*`: Chain queries
- `state_*`: State queries
- `author_*`: Transaction submission

#### Custom RPC Methods
- `energyToken_*`: Token-specific queries
- `energyTrade_*`: Trade-specific queries
- `userRegistry_*`: User-specific queries

## Performance Architecture

### Scalability Considerations

#### Transaction Throughput
- **Block Time**: 6 seconds
- **Block Size**: Configurable weight limit
- **Transaction Pool**: Prioritized queue

#### Storage Efficiency
- **Compact Encoding**: Efficient serialization
- **Pruning**: Automatic cleanup
- **Caching**: In-memory optimizations

### Optimization Strategies

#### Runtime Optimization
- **Weight System**: Execution cost management
- **Benchmarking**: Performance measurement
- **Profiling**: Bottleneck identification

#### Network Optimization
- **Message Compression**: Reduced bandwidth
- **Batch Processing**: Bulk operations
- **Asynchronous Processing**: Non-blocking operations

## Integration Architecture

### External System Integration

#### IoT Device Integration
```
┌─────────────┐    ┌─────────────┐    ┌─────────────┐
│ Smart Meter │    │ Solar Panel │    │   Battery   │
│   Device    │    │   Device    │    │   Device    │
└─────────────┘    └─────────────┘    └─────────────┘
       │                   │                   │
       │                   │                   │
       ▼                   ▼                   ▼
┌─────────────┐    ┌─────────────┐    ┌─────────────┐
│ IoT Gateway │    │ IoT Gateway │    │ IoT Gateway │
└─────────────┘    └─────────────┘    └─────────────┘
       │                   │                   │
       │                   │                   │
       ▼                   ▼                   ▼
┌─────────────────────────────────────────────────────┐
│           Energy Transfer Pallet                   │
└─────────────────────────────────────────────────────┘
```

#### Payment System Integration
```
┌─────────────┐    ┌─────────────┐    ┌─────────────┐
│    Bank     │    │ Cryptocurrency│    │ Stablecoin  │
│   Gateway   │    │   Gateway    │    │   Gateway   │
└─────────────┘    └─────────────┘    └─────────────┘
       │                   │                   │
       │                   │                   │
       ▼                   ▼                   ▼
┌─────────────────────────────────────────────────────┐
│          Payment Settlement Pallet                 │
└─────────────────────────────────────────────────────┘
```

### Frontend Integration

#### Web Application
- **Framework**: React.js
- **Blockchain Library**: Polkadot.js
- **State Management**: Redux
- **Wallet Integration**: Polkadot.js Extension

#### Mobile Application
- **Framework**: React Native
- **Blockchain Library**: Polkadot.js
- **State Management**: Redux
- **Wallet Integration**: Custom implementation

## Monitoring Architecture

### Metrics Collection

#### System Metrics
- **Block Production**: Block time, finality
- **Transaction Metrics**: TPS, queue size
- **Network Metrics**: Peers, sync status

#### Business Metrics
- **Trading Volume**: Energy traded per period
- **User Activity**: Active users, registrations
- **Device Metrics**: Active devices, measurements

### Alerting System

#### Alert Categories
- **Critical**: System failures, security breaches
- **Warning**: Performance degradation, capacity issues
- **Info**: Normal operations, maintenance

#### Alert Channels
- **Email**: Administrative notifications
- **Slack**: Team notifications
- **SMS**: Critical alerts
- **Dashboard**: Real-time monitoring

## Deployment Architecture

### Network Topology

#### Mainnet Deployment
```
┌─────────────┐    ┌─────────────┐    ┌─────────────┐
│ Validator 1 │    │ Validator 2 │    │ Validator 3 │
│   (AWS)     │    │   (GCP)     │    │  (Azure)    │
└─────────────┘    └─────────────┘    └─────────────┘
       │                   │                   │
       │                   │                   │
       ▼                   ▼                   ▼
┌─────────────┐    ┌─────────────┐    ┌─────────────┐
│ Load Balancer│    │ Load Balancer│    │ Load Balancer│
└─────────────┘    └─────────────┘    └─────────────┘
       │                   │                   │
       │                   │                   │
       ▼                   ▼                   ▼
┌─────────────┐    ┌─────────────┐    ┌─────────────┐
│ RPC Nodes   │    │ RPC Nodes   │    │ RPC Nodes   │
│   (3x)      │    │   (3x)      │    │   (3x)      │
└─────────────┘    └─────────────┘    └─────────────┘
```

#### Testnet Deployment
```
┌─────────────┐    ┌─────────────┐
│ Validator 1 │    │ Validator 2 │
│   (Local)   │    │   (Local)   │
└─────────────┘    └─────────────┘
       │                   │
       │                   │
       ▼                   ▼
┌─────────────┐    ┌─────────────┐
│ RPC Node 1  │    │ RPC Node 2  │
└─────────────┘    └─────────────┘
```

### Infrastructure Requirements

#### Hardware Requirements
- **CPU**: 8+ cores
- **Memory**: 32GB+ RAM
- **Storage**: 500GB+ SSD
- **Network**: 1Gbps+ bandwidth

#### Software Requirements
- **OS**: Ubuntu 20.04+
- **Runtime**: Substrate runtime
- **Database**: RocksDB
- **Monitoring**: Prometheus, Grafana

This architecture provides a scalable, secure, and maintainable foundation for the GridTokenX energy trading platform.
