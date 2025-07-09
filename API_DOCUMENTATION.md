# API Documentation

## Overview

This document provides detailed API documentation for all pallets in the GridTokenX blockchain platform.

## Energy Token Pallet

### Extrinsics

#### `mint_tokens`

**Description**: Mints new energy tokens to the caller's account.

**Parameters**:
- `amount: T::TokenBalance` - The amount of tokens to mint

**Returns**: `DispatchResult`

**Events Emitted**:
- `TokensMinted { account: T::AccountId, amount: T::TokenBalance }`

**Errors**:
- `OverflowError` - When the mint operation would cause an overflow

**Example**:
```rust
// Mint 100 energy tokens
EnergyToken::mint_tokens(RuntimeOrigin::signed(account), 100)?;
```

#### `transfer`

**Description**: Transfers tokens from the caller to another account.

**Parameters**:
- `to: T::AccountId` - The recipient account
- `amount: T::TokenBalance` - The amount of tokens to transfer

**Returns**: `DispatchResult`

**Events Emitted**:
- `TokensTransferred { from: T::AccountId, to: T::AccountId, amount: T::TokenBalance }`

**Errors**:
- `InsufficientBalance` - When the sender doesn't have enough tokens
- `OverflowError` - When the transfer would cause an overflow

**Example**:
```rust
// Transfer 50 tokens to another account
EnergyToken::transfer(RuntimeOrigin::signed(from), to, 50)?;
```

### Storage

#### `TokenBalance`

**Type**: `StorageMap<_, Blake2_128Concat, T::AccountId, T::TokenBalance, ValueQuery>`

**Description**: Maps account IDs to their token balances.

**Query**:
```rust
let balance = EnergyToken::token_balance(&account);
```

## Energy Trade Pallet

### Types

#### `OrderType`
```rust
pub enum OrderType {
    Ask,    // Seller's offer
    Bid,    // Buyer's offer
}
```

#### `OrderStatus`
```rust
pub enum OrderStatus {
    Open,       // Order is open for matching
    Matched,    // Order has been matched
    InTransfer, // Energy transfer in progress
    Completed,  // Order completed successfully
    Cancelled,  // Order was cancelled
    Failed,     // Order failed
}
```

#### `TradeOrder`
```rust
pub struct TradeOrder<T: Config> {
    pub order_type: OrderType,
    pub creator: T::AccountId,
    pub counterparty: Option<T::AccountId>,
    pub energy_amount: T::TokenBalance,
    pub price_per_unit: T::TokenBalance,
    pub total_price: T::TokenBalance,
    pub status: OrderStatus,
    pub grid_location: Vec<u8>,
    pub created_at: T::BlockNumber,
    pub matched_at: Option<T::BlockNumber>,
    pub completed_at: Option<T::BlockNumber>,
    pub transfer_verification: Option<T::Hash>,
}
```

### Extrinsics

#### `create_ask_order`

**Description**: Creates a sell order for energy.

**Parameters**:
- `energy_amount: T::TokenBalance` - Amount of energy to sell
- `price_per_unit: T::TokenBalance` - Price per unit of energy
- `grid_location: Vec<u8>` - Grid location identifier

**Returns**: `DispatchResult`

**Events Emitted**:
- `AskOrderCreated { order_id, seller, amount, price, location }`

**Errors**:
- `InvalidAmount` - When energy amount is zero
- `InvalidPrice` - When price is zero or causes overflow

#### `create_bid_order`

**Description**: Creates a buy order for energy.

**Parameters**:
- `energy_amount: T::TokenBalance` - Amount of energy to buy
- `price_per_unit: T::TokenBalance` - Price per unit of energy
- `grid_location: Vec<u8>` - Grid location identifier

**Returns**: `DispatchResult`

**Events Emitted**:
- `BidOrderCreated { order_id, buyer, amount, price, location }`

**Errors**:
- `InvalidAmount` - When energy amount is zero
- `InvalidPrice` - When price is zero or causes overflow
- `InsufficientBalance` - When buyer doesn't have enough funds

#### `match_orders`

**Description**: Matches compatible ask and bid orders.

**Parameters**:
- `ask_id: T::Hash` - ID of the sell order
- `bid_id: T::Hash` - ID of the buy order

**Returns**: `DispatchResult`

**Events Emitted**:
- `OrdersMatched { ask_id, bid_id, seller, buyer, amount, price }`

**Errors**:
- `OrderNotFound` - When order doesn't exist
- `InvalidOrderStatus` - When order is not open
- `OrderMismatch` - When orders are incompatible

#### `verify_transfer`

**Description**: Verifies energy transfer using IoT data.

**Parameters**:
- `order_id: T::Hash` - ID of the order
- `verification_data: Vec<u8>` - IoT verification data

**Returns**: `DispatchResult`

**Events Emitted**:
- `TransferVerified { order_id, verification_hash }`

**Errors**:
- `OrderNotFound` - When order doesn't exist
- `InvalidOrderStatus` - When order is not matched

#### `complete_trade`

**Description**: Completes a verified trade.

**Parameters**:
- `order_id: T::Hash` - ID of the order to complete

**Returns**: `DispatchResult`

**Events Emitted**:
- `OrderCompleted { order_id, seller, buyer, amount, price }`

**Errors**:
- `OrderNotFound` - When order doesn't exist
- `InvalidOrderStatus` - When order is not in transfer
- `TransferVerificationFailed` - When transfer is not verified

### Storage

#### `TradeOrders`

**Type**: `StorageMap<_, Blake2_128Concat, T::Hash, TradeOrder<T>, OptionQuery>`

**Description**: Maps order IDs to trade orders.

#### `UserOrders`

**Type**: `StorageMap<_, Blake2_128Concat, T::AccountId, Vec<T::Hash>, ValueQuery>`

**Description**: Maps users to their order IDs.

## User Registry Pallet

### Types

#### `UserRole`
```rust
pub enum UserRole {
    Consumer,      // Energy buyers only
    Prosumer,      // Energy producers and consumers
    GridOperator,  // Grid management entity
    Admin,         // System administrator
}
```

#### `DeviceType`
```rust
pub enum DeviceType {
    SolarPanel,    // Solar energy generation
    Battery,       // Energy storage
    SmartMeter,    // Energy measurement
    Other,         // Other devices
}
```

#### `UserProfile`
```rust
pub struct UserProfile<T: Config> {
    pub role: UserRole,
    pub devices: Vec<T::Hash>,
    pub active: bool,
    pub reputation_score: u32,
    pub registration_date: T::BlockNumber,
}
```

#### `Device`
```rust
pub struct Device<T: Config> {
    pub owner: T::AccountId,
    pub device_type: DeviceType,
    pub max_capacity: u32,
    pub active: bool,
    pub registration_date: T::BlockNumber,
}
```

### Extrinsics

#### `register_user`

**Description**: Registers a new user with specified role.

**Parameters**:
- `role: UserRole` - The role for the new user

**Returns**: `DispatchResult`

**Events Emitted**:
- `UserRegistered { account, role }`

**Errors**:
- `UserAlreadyRegistered` - When user is already registered
- `InvalidRole` - When role is invalid

#### `register_device`

**Description**: Registers a new device (prosumers and grid operators only).

**Parameters**:
- `device_type: DeviceType` - Type of device
- `max_capacity: u32` - Maximum capacity of the device

**Returns**: `DispatchResult`

**Events Emitted**:
- `DeviceRegistered { device_id, owner, device_type }`

**Errors**:
- `UserNotFound` - When user is not registered
- `Unauthorized` - When user doesn't have permission
- `DeviceAlreadyRegistered` - When device is already registered

#### `update_user_role`

**Description**: Updates a user's role (admin only).

**Parameters**:
- `account: T::AccountId` - Account to update
- `new_role: UserRole` - New role for the user

**Returns**: `DispatchResult`

**Events Emitted**:
- `UserUpdated { account }`

**Errors**:
- `UserNotFound` - When user doesn't exist
- `Unauthorized` - When caller is not admin

### Storage

#### `UserProfiles`

**Type**: `StorageMap<_, Blake2_128Concat, T::AccountId, UserProfile<T>, OptionQuery>`

**Description**: Maps account IDs to user profiles.

#### `Devices`

**Type**: `StorageMap<_, Blake2_128Concat, T::Hash, Device<T>, OptionQuery>`

**Description**: Maps device IDs to device information.

## Energy Transfer Pallet

### Types

#### `TransferStatus`
```rust
pub enum TransferStatus {
    Pending,    // Transfer is pending
    InProgress, // Transfer is in progress
    Completed,  // Transfer completed successfully
    Failed,     // Transfer failed
}
```

#### `TransferData`
```rust
pub struct TransferData<T: Config> {
    pub order_id: T::Hash,
    pub start_time: T::Moment,
    pub end_time: Option<T::Moment>,
    pub energy_delivered: T::TokenBalance,
    pub grid_metrics: Vec<u8>,
    pub status: TransferStatus,
}
```

#### `IoTMeasurement`
```rust
pub struct IoTMeasurement {
    pub device_id: Vec<u8>,
    pub timestamp: u64,
    pub energy_amount: u64,
    pub grid_frequency: u32,
    pub voltage: u32,
}
```

### Extrinsics

#### `start_transfer`

**Description**: Initiates an energy transfer.

**Parameters**:
- `order_id: T::Hash` - ID of the matched order
- `start_time: T::Moment` - Transfer start time

**Returns**: `DispatchResult`

**Events Emitted**:
- `TransferStarted { order_id, start_time }`

**Errors**:
- `TransferAlreadyStarted` - When transfer is already in progress

#### `record_measurement`

**Description**: Records IoT measurement data during transfer.

**Parameters**:
- `order_id: T::Hash` - ID of the order
- `measurement: IoTMeasurement` - IoT measurement data

**Returns**: `DispatchResult`

**Events Emitted**:
- `MeasurementRecorded { order_id, device_id, energy_amount }`

**Errors**:
- `TransferNotFound` - When transfer doesn't exist
- `InvalidMeasurement` - When measurement is invalid

#### `complete_transfer`

**Description**: Completes an energy transfer.

**Parameters**:
- `order_id: T::Hash` - ID of the order
- `end_time: T::Moment` - Transfer end time
- `final_measurement: IoTMeasurement` - Final measurement data

**Returns**: `DispatchResult`

**Events Emitted**:
- `TransferCompleted { order_id, total_energy }`

**Errors**:
- `TransferNotFound` - When transfer doesn't exist
- `InvalidTransferStatus` - When transfer is not in progress

#### `report_transfer_failure`

**Description**: Reports a transfer failure.

**Parameters**:
- `order_id: T::Hash` - ID of the failed order
- `reason: Vec<u8>` - Failure reason

**Returns**: `DispatchResult`

**Events Emitted**:
- `TransferFailed { order_id, reason }`

**Errors**:
- `TransferNotFound` - When transfer doesn't exist

### Storage

#### `Transfers`

**Type**: `StorageMap<_, Blake2_128Concat, T::Hash, TransferData<T>, OptionQuery>`

**Description**: Maps order IDs to transfer data.

#### `IoTMeasurements`

**Type**: `StorageMap<_, Blake2_128Concat, T::Hash, Vec<IoTMeasurement>, ValueQuery>`

**Description**: Maps order IDs to IoT measurements.

## Payment Settlement Pallet

### Types

#### `PaymentMethod`
```rust
pub enum PaymentMethod {
    Native,           // Platform's native token
    Fiat,            // Traditional currency
    Stablecoin,      // USD-pegged cryptocurrency
    ExternalToken,   // Other cryptocurrency
}
```

#### `PaymentStatus`
```rust
pub enum PaymentStatus {
    Pending,     // Payment is pending
    Processing,  // Payment is being processed
    Completed,   // Payment completed successfully
    Failed,      // Payment failed
    Refunded,    // Payment was refunded
}
```

#### `Payment`
```rust
pub struct Payment<T: Config> {
    pub order_id: T::Hash,
    pub payer: T::AccountId,
    pub payee: T::AccountId,
    pub amount: T::TokenBalance,
    pub payment_method: PaymentMethod,
    pub status: PaymentStatus,
    pub external_reference: Option<Vec<u8>>,
    pub timestamp: T::BlockNumber,
}
```

### Extrinsics

#### `create_payment`

**Description**: Creates a payment for an order.

**Parameters**:
- `order_id: T::Hash` - ID of the order
- `payment_method: PaymentMethod` - Payment method
- `external_reference: Option<Vec<u8>>` - External reference

**Returns**: `DispatchResult`

**Events Emitted**:
- `PaymentCreated { payment_id, order_id, amount, method }`

**Errors**:
- `PaymentNotFound` - When associated order doesn't exist

#### `process_native_payment`

**Description**: Processes a native token payment.

**Parameters**:
- `payment_id: T::Hash` - ID of the payment

**Returns**: `DispatchResult`

**Events Emitted**:
- `PaymentCompleted { payment_id, order_id }`

**Errors**:
- `PaymentNotFound` - When payment doesn't exist
- `InvalidPaymentStatus` - When payment is not pending
- `PaymentMethodNotSupported` - When payment method is not native

#### `process_external_payment`

**Description**: Processes an external payment with proof.

**Parameters**:
- `payment_id: T::Hash` - ID of the payment
- `proof: Vec<u8>` - Payment proof

**Returns**: `DispatchResult`

**Events Emitted**:
- `PaymentCompleted { payment_id, order_id }`
- `PaymentFailed { payment_id, reason }`

**Errors**:
- `PaymentNotFound` - When payment doesn't exist
- `InvalidPaymentStatus` - When payment is not pending
- `ExternalPaymentFailed` - When proof verification fails

### Storage

#### `Payments`

**Type**: `StorageMap<_, Blake2_128Concat, T::Hash, Payment<T>, OptionQuery>`

**Description**: Maps payment IDs to payment information.

#### `ExchangeRates`

**Type**: `StorageMap<_, Blake2_128Concat, (Vec<u8>, Vec<u8>), ExchangeRate, OptionQuery>`

**Description**: Maps currency pairs to exchange rates.

## Price Discovery Pallet

### Types

#### `PricePoint`
```rust
pub struct PricePoint<T: Config> {
    pub price: T::TokenBalance,
    pub timestamp: T::BlockNumber,
    pub volume: T::TokenBalance,
    pub location: Vec<u8>,
}
```

#### `MarketData`
```rust
pub struct MarketData<T: Config> {
    pub current_price: T::TokenBalance,
    pub daily_high: T::TokenBalance,
    pub daily_low: T::TokenBalance,
    pub daily_volume: T::TokenBalance,
    pub price_history: Vec<PricePoint<T>>,
}
```

#### `GridMetrics`
```rust
pub struct GridMetrics {
    pub congestion_level: u8,    // 0-100
    pub loss_factor: u8,         // 0-100
    pub stability_index: u8,     // 0-100
}
```

### Extrinsics

#### `update_market_data`

**Description**: Updates market data for a location.

**Parameters**:
- `location: Vec<u8>` - Grid location
- `price: T::TokenBalance` - Current price
- `volume: T::TokenBalance` - Trading volume

**Returns**: `DispatchResult`

**Events Emitted**:
- `PriceUpdated { location, new_price }`

**Errors**:
- `InvalidPrice` - When price is zero

#### `update_grid_metrics`

**Description**: Updates grid metrics for a location.

**Parameters**:
- `location: Vec<u8>` - Grid location
- `congestion: u8` - Congestion level (0-100)
- `loss_factor: u8` - Loss factor (0-100)
- `stability: u8` - Stability index (0-100)

**Returns**: `DispatchResult`

**Events Emitted**:
- `GridMetricsUpdated { location, congestion, loss_factor }`

**Errors**:
- `InvalidMetrics` - When metrics are out of range

### Storage

#### `MarketDataStore`

**Type**: `StorageMap<_, Blake2_128Concat, Vec<u8>, MarketData<T>, OptionQuery>`

**Description**: Maps locations to market data.

#### `GridMetricsStore`

**Type**: `StorageMap<_, Blake2_128Concat, Vec<u8>, GridMetrics, OptionQuery>`

**Description**: Maps locations to grid metrics.

## Error Handling

### Common Error Types

All pallets use the standard Substrate error handling pattern:

```rust
#[pallet::error]
pub enum Error<T> {
    // Specific error variants
}
```

### Error Propagation

Errors are propagated using the `?` operator and converted to `DispatchResult`:

```rust
fn some_function() -> DispatchResult {
    ensure!(condition, Error::<T>::SomeError);
    // ... function logic
    Ok(())
}
```

### Error Handling Best Practices

1. **Descriptive Error Names**: Use clear, descriptive error names
2. **Input Validation**: Validate all inputs before processing
3. **State Consistency**: Ensure state remains consistent on errors
4. **Event Emission**: Emit events for both success and failure cases

## Rate Limiting and Security

### Access Control

- **Role-based Access**: Operations restricted by user roles
- **Owner Verification**: Device operations require ownership proof
- **Admin Functions**: Critical operations require admin privileges

### Input Validation

- **Range Checks**: Numeric inputs validated for valid ranges
- **Data Integrity**: Hash verification for critical data
- **Overflow Protection**: Mathematical operations protected against overflow

### Economic Security

- **Balance Checks**: Sufficient balance verified before operations
- **Reputation System**: User reputation affects trading privileges
- **Slashing Mechanisms**: Penalties for malicious behavior

## Integration Examples

### Creating a Complete Energy Trade

```rust
// 1. Register users
UserRegistry::register_user(RuntimeOrigin::signed(seller), UserRole::Prosumer)?;
UserRegistry::register_user(RuntimeOrigin::signed(buyer), UserRole::Consumer)?;

// 2. Mint tokens for seller
EnergyToken::mint_tokens(RuntimeOrigin::signed(seller), 1000)?;

// 3. Create ask order
let order_id = EnergyTrade::create_ask_order(
    RuntimeOrigin::signed(seller),
    100,  // energy_amount
    10,   // price_per_unit
    b"grid_location_1".to_vec()
)?;

// 4. Create bid order
let bid_id = EnergyTrade::create_bid_order(
    RuntimeOrigin::signed(buyer),
    100,  // energy_amount
    10,   // price_per_unit
    b"grid_location_1".to_vec()
)?;

// 5. Match orders
EnergyTrade::match_orders(RuntimeOrigin::signed(matcher), order_id, bid_id)?;

// 6. Start energy transfer
EnergyTransfer::start_transfer(RuntimeOrigin::signed(operator), order_id, now())?;

// 7. Record IoT measurements
let measurement = IoTMeasurement {
    device_id: b"device_1".to_vec(),
    timestamp: now(),
    energy_amount: 100,
    grid_frequency: 50,
    voltage: 230,
};
EnergyTransfer::record_measurement(RuntimeOrigin::signed(operator), order_id, measurement)?;

// 8. Complete transfer
EnergyTransfer::complete_transfer(RuntimeOrigin::signed(operator), order_id, now(), final_measurement)?;

// 9. Complete trade
EnergyTrade::complete_trade(RuntimeOrigin::signed(operator), order_id)?;
```

This completes the full energy trading workflow from user registration to trade completion.
