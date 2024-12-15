#![cfg_attr(not(feature = "std"), no_std)]
#![recursion_limit = "256"]

use frame_support::{
    construct_runtime, parameter_types,
    traits::{ConstU128, ConstU32, ConstU64, ConstU8},
    weights::Weight,
};
use frame_system::limits::{BlockLength, BlockWeights};
use sp_runtime::{
    create_runtime_str,
    generic,
    traits::{AccountIdLookup, BlakeTwo256, Block as BlockT, IdentifyAccount, Verify},
    transaction_validity::{TransactionSource, TransactionValidity},
    MultiSignature,
};
use sp_std::prelude::*;
use sp_version::RuntimeVersion;

pub type BlockNumber = u32;
pub type Signature = MultiSignature;
pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;
pub type Balance = u128;
pub type Hash = sp_core::H256;
pub type Nonce = u32;

parameter_types! {
    pub const Version: RuntimeVersion = RuntimeVersion {
        spec_name: create_runtime_str!("solar-grid"),
        impl_name: create_runtime_str!("solar-grid"),
        authoring_version: 1,
        spec_version: 1,
        impl_version: 1,
        apis: sp_version::ApiVersions::empty(),
        transaction_version: 1,
        state_version: 1,
    };
    pub const BlockHashCount: BlockNumber = 2400;
}

construct_runtime!(
    pub enum Runtime where
        Block = Block,
        NodeBlock = generic::Block<Header, sp_runtime::OpaqueExtrinsic>,
        UncheckedExtrinsic = UncheckedExtrinsic
    {
        System: frame_system,
        Timestamp: pallet_timestamp,
        Aura: pallet_aura,
        Grandpa: pallet_grandpa,
        Balances: pallet_balances,
        Sudo: pallet_sudo,
        
        // Custom pallets
        EnergyToken: pallet_energy_token,
        EnergyTrade: pallet_energy_trade,
        UserRegistry: pallet_user_registry,
    }
);

impl frame_system::Config for Runtime {
    type BaseCallFilter = frame_support::traits::Everything;
    type BlockWeights = BlockWeights;
    type BlockLength = BlockLength;
    type AccountId = AccountId;
    type RuntimeCall = RuntimeCall;
    type Lookup = AccountIdLookup<AccountId, ()>;
    type Hash = Hash;
    type Hashing = BlakeTwo256;
    type Block = Block;
    type RuntimeEvent = RuntimeEvent;
    type RuntimeOrigin = RuntimeOrigin;
    type BlockHashCount = BlockHashCount;
    type Version = Version;
    type PalletInfo = PalletInfo;
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type DbWeight = ();
    type SystemWeightInfo = ();
    type SS58Prefix = ConstU16<42>;
    type OnSetCode = ();
    type MaxConsumers = ConstU32<16>;
    type RuntimeTask = ();
    type Nonce = ();
    type AccountData = ();
    type SingleBlockMigrations = ();
    type MultiBlockMigrator = ();
    type PreInherents = ();
    type PostInherents = ();
    type PostTransactions = ();
}

impl pallet_energy_token::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type TokenBalance = Balance;
}

impl pallet_energy_trade::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type TokenBalance = Balance;
}

impl pallet_user_registry::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
}

// Other pallet configurations would go here...
