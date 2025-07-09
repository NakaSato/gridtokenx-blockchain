# Testing Guide

## Overview

This guide covers comprehensive testing strategies for the GridTokenX blockchain platform, including unit tests, integration tests, benchmarking, and security testing.

## Test Structure

### Directory Organization

```
gridtokenx-blockchain/
├── pallets/
│   ├── energy-token/
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── tests.rs          # Unit tests
│   │   │   └── mock.rs           # Test environment
│   │   └── Cargo.toml
│   └── energy-trade/
│       ├── src/
│       │   ├── lib.rs
│       │   ├── tests.rs          # Unit tests
│       │   └── mock.rs           # Test environment
│       └── Cargo.toml
├── runtime/
│   ├── src/
│   │   └── lib.rs
│   └── Cargo.toml
├── tests/
│   ├── integration/              # Integration tests
│   ├── benchmarks/               # Benchmarking tests
│   └── security/                 # Security tests
└── scripts/
    └── test/                     # Test scripts
```

## Unit Testing

### Test Environment Setup

Each pallet includes a `mock.rs` file that sets up the test environment:

```rust
// pallets/energy-token/src/mock.rs
use frame_support::{
    construct_runtime, parameter_types,
    traits::{ConstU16, ConstU32, ConstU64},
    weights::Weight,
};
use sp_core::H256;
use sp_runtime::{
    traits::{BlakeTwo256, IdentityLookup},
    BuildStorage,
};

type Block = frame_system::mocking::MockBlock<Test>;

construct_runtime!(
    pub enum Test
    {
        System: frame_system,
        EnergyToken: crate,
    }
);

parameter_types! {
    pub const BlockHashCount: u64 = 250;
    pub const SS58Prefix: u8 = 42;
}

impl frame_system::Config for Test {
    type RuntimeOrigin = RuntimeOrigin;
    type RuntimeCall = RuntimeCall;
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type AccountId = u64;
    type Lookup = IdentityLookup<Self::AccountId>;
    type RuntimeEvent = RuntimeEvent;
    type BlockHashCount = BlockHashCount;
    type Version = ();
    type PalletInfo = PalletInfo;
    type AccountData = ();
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type DbWeight = ();
    type BaseCallFilter = frame_support::traits::Everything;
    type SystemWeightInfo = ();
    type BlockWeights = ();
    type BlockLength = ();
    type SS58Prefix = SS58Prefix;
    type OnSetCode = ();
    type MaxConsumers = ConstU32<16>;
}

impl crate::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type TokenBalance = u64;
}

pub fn new_test_ext() -> sp_io::TestExternalities {
    frame_system::GenesisConfig::<Test>::default()
        .build_storage()
        .unwrap()
        .into()
}
```

### Writing Unit Tests

#### Energy Token Pallet Tests

```rust
// pallets/energy-token/src/tests.rs
use crate::{mock::*, Error, Event};
use frame_support::{assert_noop, assert_ok};

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
        
        System::assert_last_event(Event::TokensMinted {
            account,
            amount,
        }.into());
    });
}

#[test]
fn transfer_works() {
    new_test_ext().execute_with(|| {
        let from = 1;
        let to = 2;
        let amount = 50;

        // First mint some tokens
        assert_ok!(EnergyToken::mint_tokens(
            RuntimeOrigin::signed(from),
            100
        ));
        
        // Then transfer
        assert_ok!(EnergyToken::transfer(
            RuntimeOrigin::signed(from),
            to,
            amount
        ));
        
        assert_eq!(EnergyToken::token_balance(from), 50);
        assert_eq!(EnergyToken::token_balance(to), amount);
        
        System::assert_last_event(Event::TokensTransferred {
            from,
            to,
            amount,
        }.into());
    });
}

#[test]
fn transfer_fails_with_insufficient_balance() {
    new_test_ext().execute_with(|| {
        let from = 1;
        let to = 2;
        let amount = 100;

        assert_noop!(
            EnergyToken::transfer(RuntimeOrigin::signed(from), to, amount),
            Error::<Test>::InsufficientBalance
        );
    });
}

#[test]
fn mint_tokens_handles_overflow() {
    new_test_ext().execute_with(|| {
        let account = 1;
        let max_value = u64::MAX;

        // First mint max value
        assert_ok!(EnergyToken::mint_tokens(
            RuntimeOrigin::signed(account),
            max_value
        ));
        
        // Try to mint more - should fail
        assert_noop!(
            EnergyToken::mint_tokens(RuntimeOrigin::signed(account), 1),
            Error::<Test>::OverflowError
        );
    });
}
```

#### Energy Trade Pallet Tests

```rust
// pallets/energy-trade/src/tests.rs
use crate::{mock::*, Error, Event, OrderStatus, OrderType};
use frame_support::{assert_noop, assert_ok};

#[test]
fn create_ask_order_works() {
    new_test_ext().execute_with(|| {
        let seller = 1;
        let energy_amount = 100;
        let price_per_unit = 10;
        let location = b"grid_location_1".to_vec();

        assert_ok!(EnergyTrade::create_ask_order(
            RuntimeOrigin::signed(seller),
            energy_amount,
            price_per_unit,
            location.clone()
        ));

        // Verify order was created
        let events = System::events();
        let order_created_event = events.iter()
            .find(|e| matches!(e.event, Event::AskOrderCreated { .. }))
            .expect("AskOrderCreated event not found");

        if let Event::AskOrderCreated { order_id, .. } = order_created_event.event {
            let order = EnergyTrade::trade_orders(order_id).unwrap();
            assert_eq!(order.creator, seller);
            assert_eq!(order.energy_amount, energy_amount);
            assert_eq!(order.price_per_unit, price_per_unit);
            assert_eq!(order.status, OrderStatus::Open);
            assert_eq!(order.order_type, OrderType::Ask);
            assert_eq!(order.grid_location, location);
        }
    });
}

#[test]
fn create_bid_order_works() {
    new_test_ext().execute_with(|| {
        let buyer = 1;
        let energy_amount = 100;
        let price_per_unit = 10;
        let location = b"grid_location_1".to_vec();

        // Mock sufficient balance for buyer
        MockCurrency::set_balance(&buyer, 1000);

        assert_ok!(EnergyTrade::create_bid_order(
            RuntimeOrigin::signed(buyer),
            energy_amount,
            price_per_unit,
            location.clone()
        ));

        // Verify order was created
        let events = System::events();
        let order_created_event = events.iter()
            .find(|e| matches!(e.event, Event::BidOrderCreated { .. }))
            .expect("BidOrderCreated event not found");

        if let Event::BidOrderCreated { order_id, .. } = order_created_event.event {
            let order = EnergyTrade::trade_orders(order_id).unwrap();
            assert_eq!(order.creator, buyer);
            assert_eq!(order.energy_amount, energy_amount);
            assert_eq!(order.price_per_unit, price_per_unit);
            assert_eq!(order.status, OrderStatus::Open);
            assert_eq!(order.order_type, OrderType::Bid);
            assert_eq!(order.grid_location, location);
        }
    });
}

#[test]
fn match_orders_works() {
    new_test_ext().execute_with(|| {
        let seller = 1;
        let buyer = 2;
        let energy_amount = 100;
        let price_per_unit = 10;
        let location = b"grid_location_1".to_vec();

        // Create ask order
        assert_ok!(EnergyTrade::create_ask_order(
            RuntimeOrigin::signed(seller),
            energy_amount,
            price_per_unit,
            location.clone()
        ));

        // Mock sufficient balance for buyer
        MockCurrency::set_balance(&buyer, 1000);

        // Create bid order
        assert_ok!(EnergyTrade::create_bid_order(
            RuntimeOrigin::signed(buyer),
            energy_amount,
            price_per_unit,
            location
        ));

        // Get order IDs from events
        let events = System::events();
        let ask_order_id = events.iter()
            .find_map(|e| {
                if let Event::AskOrderCreated { order_id, .. } = e.event {
                    Some(order_id)
                } else {
                    None
                }
            })
            .unwrap();

        let bid_order_id = events.iter()
            .find_map(|e| {
                if let Event::BidOrderCreated { order_id, .. } = e.event {
                    Some(order_id)
                } else {
                    None
                }
            })
            .unwrap();

        // Match orders
        assert_ok!(EnergyTrade::match_orders(
            RuntimeOrigin::signed(seller),
            ask_order_id,
            bid_order_id
        ));

        // Verify orders are matched
        let ask_order = EnergyTrade::trade_orders(ask_order_id).unwrap();
        let bid_order = EnergyTrade::trade_orders(bid_order_id).unwrap();

        assert_eq!(ask_order.status, OrderStatus::Matched);
        assert_eq!(bid_order.status, OrderStatus::Matched);
        assert_eq!(ask_order.counterparty, Some(buyer));
        assert_eq!(bid_order.counterparty, Some(seller));
    });
}

#[test]
fn match_orders_fails_with_incompatible_orders() {
    new_test_ext().execute_with(|| {
        let seller = 1;
        let buyer = 2;
        let location = b"grid_location_1".to_vec();

        // Create ask order
        assert_ok!(EnergyTrade::create_ask_order(
            RuntimeOrigin::signed(seller),
            100,  // energy_amount
            10,   // price_per_unit
            location.clone()
        ));

        // Mock sufficient balance for buyer
        MockCurrency::set_balance(&buyer, 1000);

        // Create bid order with different amount
        assert_ok!(EnergyTrade::create_bid_order(
            RuntimeOrigin::signed(buyer),
            50,   // different energy_amount
            10,   // price_per_unit
            location
        ));

        // Get order IDs from events
        let events = System::events();
        let ask_order_id = events.iter()
            .find_map(|e| {
                if let Event::AskOrderCreated { order_id, .. } = e.event {
                    Some(order_id)
                } else {
                    None
                }
            })
            .unwrap();

        let bid_order_id = events.iter()
            .find_map(|e| {
                if let Event::BidOrderCreated { order_id, .. } = e.event {
                    Some(order_id)
                } else {
                    None
                }
            })
            .unwrap();

        // Attempt to match incompatible orders
        assert_noop!(
            EnergyTrade::match_orders(
                RuntimeOrigin::signed(seller),
                ask_order_id,
                bid_order_id
            ),
            Error::<Test>::OrderMismatch
        );
    });
}
```

### Running Unit Tests

```bash
# Run all tests
cargo test

# Run tests for specific pallet
cargo test -p pallet-energy-token

# Run specific test
cargo test test_mint_tokens_works

# Run tests with output
cargo test -- --nocapture

# Run tests with specific log level
RUST_LOG=debug cargo test
```

## Integration Testing

### Cross-Pallet Integration Tests

```rust
// tests/integration/energy_trading.rs
use frame_support::{assert_ok, construct_runtime, parameter_types};
use pallet_energy_token as energy_token;
use pallet_energy_trade as energy_trade;
use pallet_user_registry as user_registry;

#[test]
fn complete_energy_trade_flow() {
    new_test_ext().execute_with(|| {
        let seller = 1;
        let buyer = 2;
        let energy_amount = 100;
        let price_per_unit = 10;
        let location = b"grid_location_1".to_vec();

        // Step 1: Register users
        assert_ok!(UserRegistry::register_user(
            RuntimeOrigin::signed(seller),
            UserRole::Prosumer
        ));
        
        assert_ok!(UserRegistry::register_user(
            RuntimeOrigin::signed(buyer),
            UserRole::Consumer
        ));

        // Step 2: Mint tokens for seller
        assert_ok!(EnergyToken::mint_tokens(
            RuntimeOrigin::signed(seller),
            1000
        ));

        // Step 3: Create ask order
        assert_ok!(EnergyTrade::create_ask_order(
            RuntimeOrigin::signed(seller),
            energy_amount,
            price_per_unit,
            location.clone()
        ));

        // Step 4: Create bid order (mock balance for buyer)
        MockCurrency::set_balance(&buyer, 1000);
        
        assert_ok!(EnergyTrade::create_bid_order(
            RuntimeOrigin::signed(buyer),
            energy_amount,
            price_per_unit,
            location
        ));

        // Step 5: Match orders
        let events = System::events();
        let ask_order_id = extract_order_id(&events, true);
        let bid_order_id = extract_order_id(&events, false);

        assert_ok!(EnergyTrade::match_orders(
            RuntimeOrigin::signed(seller),
            ask_order_id,
            bid_order_id
        ));

        // Step 6: Verify energy transfer
        assert_ok!(EnergyTransfer::start_transfer(
            RuntimeOrigin::signed(seller),
            ask_order_id,
            System::block_number()
        ));

        // Step 7: Record IoT measurements
        let measurement = IoTMeasurement {
            device_id: b"device_1".to_vec(),
            timestamp: System::block_number() as u64,
            energy_amount: 100,
            grid_frequency: 50,
            voltage: 230,
        };

        assert_ok!(EnergyTransfer::record_measurement(
            RuntimeOrigin::signed(seller),
            ask_order_id,
            measurement.clone()
        ));

        // Step 8: Complete transfer
        assert_ok!(EnergyTransfer::complete_transfer(
            RuntimeOrigin::signed(seller),
            ask_order_id,
            System::block_number() + 10,
            measurement
        ));

        // Step 9: Complete trade
        assert_ok!(EnergyTrade::complete_trade(
            RuntimeOrigin::signed(seller),
            ask_order_id
        ));

        // Verify final state
        let order = EnergyTrade::trade_orders(ask_order_id).unwrap();
        assert_eq!(order.status, OrderStatus::Completed);
        
        // Verify payment was transferred
        assert_eq!(MockCurrency::balance(&buyer), 1000 - (energy_amount * price_per_unit));
        assert_eq!(MockCurrency::balance(&seller), energy_amount * price_per_unit);
    });
}

fn extract_order_id(events: &[EventRecord], is_ask: bool) -> H256 {
    events.iter()
        .find_map(|e| {
            match &e.event {
                Event::AskOrderCreated { order_id, .. } if is_ask => Some(*order_id),
                Event::BidOrderCreated { order_id, .. } if !is_ask => Some(*order_id),
                _ => None,
            }
        })
        .unwrap()
}
```

### Performance Integration Tests

```rust
// tests/integration/performance.rs
use std::time::Instant;

#[test]
fn high_volume_trading_performance() {
    new_test_ext().execute_with(|| {
        let num_orders = 1000;
        let start_time = Instant::now();

        // Create multiple orders
        for i in 0..num_orders {
            let seller = i + 1;
            let buyer = i + 1001;
            
            // Register users
            assert_ok!(UserRegistry::register_user(
                RuntimeOrigin::signed(seller),
                UserRole::Prosumer
            ));
            
            assert_ok!(UserRegistry::register_user(
                RuntimeOrigin::signed(buyer),
                UserRole::Consumer
            ));

            // Create orders
            assert_ok!(EnergyTrade::create_ask_order(
                RuntimeOrigin::signed(seller),
                100,
                10,
                format!("location_{}", i).into_bytes()
            ));

            MockCurrency::set_balance(&buyer, 1000);
            
            assert_ok!(EnergyTrade::create_bid_order(
                RuntimeOrigin::signed(buyer),
                100,
                10,
                format!("location_{}", i).into_bytes()
            ));
        }

        let duration = start_time.elapsed();
        println!("Created {} orders in {:?}", num_orders * 2, duration);

        // Verify orders were created
        let total_orders = EnergyTrade::trade_orders_iter().count();
        assert_eq!(total_orders, num_orders * 2);
    });
}
```

## Benchmarking

### Runtime Benchmarks

```rust
// pallets/energy-token/src/benchmarking.rs
use frame_benchmarking::{benchmarks, impl_benchmark_test_suite};
use frame_support::traits::Get;
use frame_system::RawOrigin;

benchmarks! {
    mint_tokens {
        let caller: T::AccountId = whitelisted_caller();
        let amount = 1000u32.into();
    }: _(RawOrigin::Signed(caller.clone()), amount)
    verify {
        assert_eq!(Pallet::<T>::token_balance(&caller), amount);
    }

    transfer {
        let caller: T::AccountId = whitelisted_caller();
        let recipient: T::AccountId = account("recipient", 0, 0);
        let amount = 500u32.into();
        
        // Setup: mint tokens first
        let _ = Pallet::<T>::mint_tokens(
            RawOrigin::Signed(caller.clone()).into(),
            1000u32.into()
        );
    }: _(RawOrigin::Signed(caller.clone()), recipient.clone(), amount)
    verify {
        assert_eq!(Pallet::<T>::token_balance(&caller), 500u32.into());
        assert_eq!(Pallet::<T>::token_balance(&recipient), amount);
    }
}

impl_benchmark_test_suite!(Pallet, crate::mock::new_test_ext(), crate::mock::Test);
```

### Running Benchmarks

```bash
# Build with benchmarking features
cargo build --release --features runtime-benchmarks

# Run benchmarks for specific pallet
./target/release/solar-grid-node benchmark pallet \
    --pallet pallet_energy_token \
    --extrinsic "*" \
    --steps 50 \
    --repeat 20 \
    --output ./pallets/energy-token/src/weights.rs

# Run all benchmarks
./target/release/solar-grid-node benchmark pallet \
    --pallet "*" \
    --extrinsic "*" \
    --steps 50 \
    --repeat 20
```

## Load Testing

### Stress Tests

```rust
// tests/stress/load_test.rs
use std::sync::Arc;
use std::thread;
use std::time::Duration;

#[test]
fn concurrent_trading_load_test() {
    new_test_ext().execute_with(|| {
        let num_threads = 10;
        let orders_per_thread = 100;
        let handles: Vec<_> = (0..num_threads)
            .map(|thread_id| {
                thread::spawn(move || {
                    for i in 0..orders_per_thread {
                        let seller = thread_id * 1000 + i;
                        let buyer = thread_id * 1000 + i + 500;
                        
                        // Register users
                        assert_ok!(UserRegistry::register_user(
                            RuntimeOrigin::signed(seller),
                            UserRole::Prosumer
                        ));
                        
                        // Create orders
                        assert_ok!(EnergyTrade::create_ask_order(
                            RuntimeOrigin::signed(seller),
                            100,
                            10,
                            format!("location_{}_{}", thread_id, i).into_bytes()
                        ));
                        
                        thread::sleep(Duration::from_millis(10));
                    }
                })
            })
            .collect();

        // Wait for all threads to complete
        for handle in handles {
            handle.join().unwrap();
        }

        // Verify all orders were created
        let total_orders = EnergyTrade::trade_orders_iter().count();
        assert_eq!(total_orders, num_threads * orders_per_thread);
    });
}
```

### Memory Usage Tests

```rust
// tests/stress/memory_test.rs
use std::mem;

#[test]
fn memory_usage_test() {
    new_test_ext().execute_with(|| {
        let initial_memory = get_memory_usage();
        
        // Create large number of orders
        for i in 0..10000 {
            let seller = i;
            
            assert_ok!(UserRegistry::register_user(
                RuntimeOrigin::signed(seller),
                UserRole::Prosumer
            ));
            
            assert_ok!(EnergyTrade::create_ask_order(
                RuntimeOrigin::signed(seller),
                100,
                10,
                format!("location_{}", i).into_bytes()
            ));
        }
        
        let final_memory = get_memory_usage();
        let memory_increase = final_memory - initial_memory;
        
        println!("Memory increase: {} bytes", memory_increase);
        
        // Verify memory usage is reasonable
        assert!(memory_increase < 100_000_000); // Less than 100MB
    });
}

fn get_memory_usage() -> usize {
    // Platform-specific memory usage calculation
    // This is a simplified example
    std::process::Command::new("ps")
        .args(&["-o", "rss=", "-p", &std::process::id().to_string()])
        .output()
        .map(|output| {
            String::from_utf8_lossy(&output.stdout)
                .trim()
                .parse::<usize>()
                .unwrap_or(0) * 1024
        })
        .unwrap_or(0)
}
```

## Security Testing

### Input Validation Tests

```rust
// tests/security/input_validation.rs
use frame_support::assert_noop;

#[test]
fn test_overflow_protection() {
    new_test_ext().execute_with(|| {
        let account = 1;
        let max_value = u64::MAX;

        // Test token minting overflow
        assert_ok!(EnergyToken::mint_tokens(
            RuntimeOrigin::signed(account),
            max_value
        ));
        
        assert_noop!(
            EnergyToken::mint_tokens(RuntimeOrigin::signed(account), 1),
            Error::<Test>::OverflowError
        );

        // Test transfer overflow
        assert_noop!(
            EnergyToken::transfer(RuntimeOrigin::signed(account), 2, max_value),
            Error::<Test>::OverflowError
        );
    });
}

#[test]
fn test_zero_amount_validation() {
    new_test_ext().execute_with(|| {
        let seller = 1;
        let location = b"location".to_vec();

        // Test zero energy amount
        assert_noop!(
            EnergyTrade::create_ask_order(
                RuntimeOrigin::signed(seller),
                0,  // zero amount
                10,
                location.clone()
            ),
            Error::<Test>::InvalidAmount
        );

        // Test zero price
        assert_noop!(
            EnergyTrade::create_ask_order(
                RuntimeOrigin::signed(seller),
                100,
                0,  // zero price
                location
            ),
            Error::<Test>::InvalidPrice
        );
    });
}
```

### Access Control Tests

```rust
// tests/security/access_control.rs
#[test]
fn test_role_based_access_control() {
    new_test_ext().execute_with(|| {
        let admin = 1;
        let user = 2;
        let consumer = 3;

        // Register admin
        assert_ok!(UserRegistry::register_user(
            RuntimeOrigin::signed(admin),
            UserRole::Admin
        ));

        // Register regular user
        assert_ok!(UserRegistry::register_user(
            RuntimeOrigin::signed(user),
            UserRole::Consumer
        ));

        // Test that only admin can update roles
        assert_ok!(UserRegistry::update_user_role(
            RuntimeOrigin::signed(admin),
            user,
            UserRole::Prosumer
        ));

        // Test that non-admin cannot update roles
        assert_noop!(
            UserRegistry::update_user_role(
                RuntimeOrigin::signed(user),
                consumer,
                UserRole::Prosumer
            ),
            Error::<Test>::Unauthorized
        );
    });
}

#[test]
fn test_device_registration_permissions() {
    new_test_ext().execute_with(|| {
        let consumer = 1;
        let prosumer = 2;

        // Register consumer
        assert_ok!(UserRegistry::register_user(
            RuntimeOrigin::signed(consumer),
            UserRole::Consumer
        ));

        // Register prosumer
        assert_ok!(UserRegistry::register_user(
            RuntimeOrigin::signed(prosumer),
            UserRole::Prosumer
        ));

        // Test that consumer cannot register device
        assert_noop!(
            UserRegistry::register_device(
                RuntimeOrigin::signed(consumer),
                DeviceType::SolarPanel,
                1000
            ),
            Error::<Test>::Unauthorized
        );

        // Test that prosumer can register device
        assert_ok!(UserRegistry::register_device(
            RuntimeOrigin::signed(prosumer),
            DeviceType::SolarPanel,
            1000
        ));
    });
}
```

## Automated Testing

### Continuous Integration

```yaml
# .github/workflows/test.yml
name: Tests

on:
  push:
    branches: [ main, develop ]
  pull_request:
    branches: [ main ]

jobs:
  test:
    runs-on: ubuntu-latest
    
    steps:
    - uses: actions/checkout@v3
    
    - name: Install Rust
      uses: actions-rs/toolchain@v1
      with:
        toolchain: stable
        components: rustfmt, clippy
    
    - name: Cache dependencies
      uses: actions/cache@v3
      with:
        path: |
          ~/.cargo/registry
          ~/.cargo/git
          target
        key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}
    
    - name: Run unit tests
      run: cargo test --all
    
    - name: Run integration tests
      run: cargo test --test integration
    
    - name: Run benchmarks
      run: |
        cargo build --release --features runtime-benchmarks
        ./target/release/solar-grid-node benchmark pallet \
          --pallet "*" \
          --extrinsic "*" \
          --steps 10 \
          --repeat 2
    
    - name: Run security tests
      run: |
        cargo install cargo-audit
        cargo audit
        cargo test --test security
```

### Test Scripts

```bash
#!/bin/bash
# scripts/test/run_all_tests.sh

set -e

echo "Running all tests for GridTokenX..."

# Unit tests
echo "Running unit tests..."
cargo test --all

# Integration tests
echo "Running integration tests..."
cargo test --test integration

# Benchmarks
echo "Running benchmarks..."
cargo build --release --features runtime-benchmarks
./target/release/solar-grid-node benchmark pallet \
    --pallet "*" \
    --extrinsic "*" \
    --steps 20 \
    --repeat 10

# Security tests
echo "Running security audit..."
cargo audit

# Load tests
echo "Running load tests..."
cargo test --test load_test --release

echo "All tests completed successfully!"
```

## Coverage Analysis

### Code Coverage

```bash
# Install coverage tools
cargo install cargo-tarpaulin

# Generate coverage report
cargo tarpaulin --out html --output-dir coverage

# Coverage with specific features
cargo tarpaulin --features runtime-benchmarks --out html

# Coverage for specific package
cargo tarpaulin -p pallet-energy-token --out html
```

### Coverage Configuration

```toml
# Cargo.toml
[package.metadata.tarpaulin]
exclude = [
    "mock.rs",
    "tests.rs",
    "benchmarking.rs",
]
```

## Test Documentation

### Test Cases Documentation

```markdown
# Test Cases

## Energy Token Pallet

### TC-ET-001: Token Minting
- **Description**: Verify that energy tokens can be minted successfully
- **Preconditions**: Valid account ID and amount
- **Steps**:
  1. Call `mint_tokens` with valid parameters
  2. Verify balance is updated
  3. Verify event is emitted
- **Expected Result**: Tokens minted successfully

### TC-ET-002: Token Transfer
- **Description**: Verify that tokens can be transferred between accounts
- **Preconditions**: Sender has sufficient balance
- **Steps**:
  1. Mint tokens to sender
  2. Call `transfer` with valid parameters
  3. Verify balances are updated
  4. Verify event is emitted
- **Expected Result**: Tokens transferred successfully

### TC-ET-003: Insufficient Balance
- **Description**: Verify that transfer fails with insufficient balance
- **Preconditions**: Sender has insufficient balance
- **Steps**:
  1. Call `transfer` with amount greater than balance
  2. Verify error is returned
- **Expected Result**: `InsufficientBalance` error
```

## Testing Best Practices

### Test Organization

1. **Arrange-Act-Assert Pattern**:
   ```rust
   #[test]
   fn test_function() {
       // Arrange
       let account = 1;
       let amount = 100;
       
       // Act
       let result = function_call(account, amount);
       
       // Assert
       assert_eq!(result, expected_value);
   }
   ```

2. **Descriptive Test Names**:
   ```rust
   #[test]
   fn mint_tokens_works() { ... }
   
   #[test]
   fn transfer_fails_with_insufficient_balance() { ... }
   
   #[test]
   fn order_matching_requires_compatible_parameters() { ... }
   ```

3. **Test Data Management**:
   ```rust
   fn create_test_user(account: u64, role: UserRole) {
       assert_ok!(UserRegistry::register_user(
           RuntimeOrigin::signed(account),
           role
       ));
   }
   
   fn create_test_order(seller: u64, amount: u64, price: u64) -> H256 {
       assert_ok!(EnergyTrade::create_ask_order(
           RuntimeOrigin::signed(seller),
           amount,
           price,
           b"test_location".to_vec()
       ));
       // Extract and return order ID
   }
   ```

### Performance Testing Guidelines

1. **Baseline Measurements**: Establish performance baselines
2. **Regression Testing**: Detect performance regressions
3. **Scalability Testing**: Test with increasing load
4. **Resource Monitoring**: Monitor memory and CPU usage

This comprehensive testing guide ensures the reliability, security, and performance of the GridTokenX blockchain platform.
