//! Provides the logic and rules for the blockchain network to function properly. It defines the
//! state transition function, which is responsible for modifying the state of the blockchain
//! network based on the incoming transactions. The primary purpose of the runtime is to manage the
//! state transitions of the blockchain network. This includes updating the account balances,
//! storing data. The runtime also handles the validation of transactions and blocks, ensuring that
//! they comply with the consensus rules defined by the network.
//!
//! The runtime provides a framework for building and customizing blockchain networks by allowing
//! developers to add their own logic and functionality to the network. This modularity is achieved
//! through the use of Substrate runtime pallets.

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
	ApplyExtrinsicResult, MultiSignature,
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
use pallet_staking::UseValidatorsMap;
pub use pallet_timestamp::Call as TimestampCall;
use pallet_transaction_payment::{ConstFeeMultiplier, CurrencyAdapter, Multiplier};
use sp_common::{identity::AccountIdentity, traits::IdentityProvider};
pub use sp_runtime::{Perbill, Permill};

pub mod api;
pub mod consts;
pub mod extensions;
pub mod offchain;
pub mod opaque;
pub mod pallets;
pub mod types;
pub mod utils;
pub mod validators_rewards;

pub use crate::{
	api::*,
	consts::*,
	pallets::{babe::*, election_provider_multi_phase::*, identity::*, system::*},
	types::*,
	utils::*,
};

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
		TransactionIndexer: pallet_transaction_indexer,
		Reward: pallet_reward,
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
