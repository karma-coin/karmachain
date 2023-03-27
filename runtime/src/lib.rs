#![cfg_attr(not(feature = "std"), no_std)]
// `construct_runtime!` does a lot of recursion and requires us to increase the limit to 256.
#![recursion_limit = "256"]

// Make the WASM binary available.
#[cfg(feature = "std")]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));

use pallet_grandpa::{
	fg_primitives, AuthorityId as GrandpaId, AuthorityList as GrandpaAuthorityList,
};
use sp_api::impl_runtime_apis;
use sp_core::{crypto::KeyTypeId, OpaqueMetadata};
use sp_runtime::{
	create_runtime_str, generic, impl_opaque_keys,
	traits::{
		AccountIdLookup, BlakeTwo256, Block as BlockT, IdentifyAccount, NumberFor, One, OpaqueKeys,
		Verify,
	},
	transaction_validity::{TransactionPriority, TransactionSource, TransactionValidity},
	ApplyExtrinsicResult, BoundedVec, MultiSignature,
};
use sp_staking::SessionIndex;
use sp_std::{marker::PhantomData, prelude::*};
#[cfg(feature = "std")]
use sp_version::NativeVersion;
use sp_version::RuntimeVersion;

// A few exports that help ease life for downstream crates.
use frame_election_provider_support::{generate_solution_type, onchain, SequentialPhragmen};
pub use frame_support::{
	construct_runtime,
	dispatch::DispatchClass,
	parameter_types,
	traits::{
		ConstU128, ConstU32, ConstU64, ConstU8, KeyOwnerProofSystem, Randomness, StorageInfo,
	},
	weights::{
		constants::{
			BlockExecutionWeight, ExtrinsicBaseWeight, RocksDbWeight, WEIGHT_REF_TIME_PER_SECOND,
		},
		IdentityFee, Weight,
	},
	StorageValue,
};
pub use frame_system::Call as SystemCall;
use frame_system::EnsureRoot;
pub use pallet_balances::Call as BalancesCall;
use pallet_identity::IdentityProvider;
use pallet_staking::UseValidatorsMap;
pub use pallet_timestamp::Call as TimestampCall;
use pallet_transaction_payment::{ConstFeeMultiplier, CurrencyAdapter, Multiplier};
pub use sp_runtime::{Perbill, Permill};

pub mod api;
pub mod pallets;
pub mod types;
pub mod utils;
pub mod validators_rewards;

pub use crate::{
	api::*,
	pallets::{babe::*, election_provider_multi_phase::*, identity::*, system::*},
	types::*,
	utils::*,
};

/// Opaque types. These are used by the CLI to instantiate machinery that don't need to know
/// the specifics of the runtime. They can then be made to be agnostic over specific formats
/// of data like extrinsics, allowing for them to continue syncing the network through upgrades
/// to even the core data structures.
pub mod opaque {
	use super::*;

	pub use sp_runtime::OpaqueExtrinsic as UncheckedExtrinsic;

	/// Opaque block header type.
	pub type Header = generic::Header<BlockNumber, BlakeTwo256>;
	/// Opaque block type.
	pub type Block = generic::Block<Header, UncheckedExtrinsic>;
	/// Opaque block identifier type.
	pub type BlockId = generic::BlockId<Block>;

	impl_opaque_keys! {
		pub struct SessionKeys {
			pub babe: Babe,
			pub grandpa: Grandpa,
		}
	}
}

/// This determines the average expected block time that we are targeting.
/// Blocks will be produced at a minimum duration defined by `SLOT_DURATION`.
/// `SLOT_DURATION` is picked up by `pallet_timestamp` which is in turn picked
/// up by `pallet_babe` to implement `fn slot_duration()`.
///
/// Change this to adjust the block time.
pub const MILLISECS_PER_BLOCK: u64 = 60_000;

// NOTE: Currently it is not possible to change the slot duration after the chain has started.
//       Attempting to do so will brick block production.
pub const SLOT_DURATION: u64 = MILLISECS_PER_BLOCK;
pub const EPOCH_DURATION_IN_SLOTS: BlockNumber = 4 * HOURS;
pub const ERA_DURATION_IN_EPOCH: u32 = 6;

// Time is measured by number of blocks.
pub const MINUTES: BlockNumber = 60_000 / (MILLISECS_PER_BLOCK as BlockNumber);
pub const HOURS: BlockNumber = MINUTES * 60;
pub const DAYS: BlockNumber = HOURS * 24;
// Assumse month contains 30 days
pub const MONTHS: BlockNumber = DAYS * 30;

/// Existential deposit.
pub const EXISTENTIAL_DEPOSIT: u128 = 500;

pub const KCENTS: Balance = 1;
pub const KCOINS: Balance = KCENTS * 1_000_000;

/// The version information used to identify this runtime when compiled natively.
#[cfg(feature = "std")]
pub fn native_version() -> NativeVersion {
	NativeVersion { runtime_version: VERSION, can_author_with: Default::default() }
}

// Create the runtime by composing the FRAME pallets that were previously configured.
construct_runtime!(
	pub struct Runtime
	where
		Block = Block,
		NodeBlock = opaque::Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		// Basic stuff; balances is uncallable initially.
		System: frame_system,

		// Babe must be before session.
		Babe: pallet_babe,

		Timestamp: pallet_timestamp,
		Balances: pallet_balances,
		TransactionPayment: pallet_transaction_payment,
		Sudo: pallet_sudo,

		// Consensus support.
		// Authorship must be before session in order to note author in the correct session and era
		// for im-online and staking.
		Authorship: pallet_authorship,
		Staking: pallet_staking,
		// Election pallet. Only works with staking, but placed here to maintain indices.
		ElectionProviderMultiPhase: pallet_election_provider_multi_phase,
		Historical: pallet_session::historical,
		Session: pallet_session,
		Grandpa: pallet_grandpa,
		// Governance stuff.

		// Provides a semi-sorted list of nominators for staking.
		VoterList: pallet_bags_list::<Instance1>::{Pallet, Call, Storage, Event<T>},

		// Include the custom logic from the pallet-template in the runtime.
		Identity: pallet_identity,
		Appreciation: pallet_appreciation,
	}
);

#[cfg(feature = "runtime-benchmarks")]
#[macro_use]
extern crate frame_benchmarking;

#[cfg(feature = "runtime-benchmarks")]
mod benches {
	define_benchmarks!(
		[frame_benchmarking, BaselineBench::<Runtime>]
		[frame_system, SystemBench::<Runtime>]
		[pallet_balances, Balances]
		[pallet_timestamp, Timestamp]
	);
}
