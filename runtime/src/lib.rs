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
#[cfg(any(feature = "std", test))]
pub use sp_runtime::BuildStorage;
pub use sp_runtime::{Perbill, Permill};

pub mod pallets;
pub mod types;
pub mod utils;
pub mod validators_rewards;

pub use crate::{
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

// To learn more about runtime versioning, see:
// https://docs.substrate.io/main-docs/build/upgrade#runtime-versioning
#[sp_version::runtime_version]
pub const VERSION: RuntimeVersion = RuntimeVersion {
	spec_name: create_runtime_str!("karmachain-node"),
	impl_name: create_runtime_str!("karmachain-node"),
	authoring_version: 1,
	// The version of the runtime specification. A full node will not attempt to use its native
	//   runtime in substitute for the on-chain Wasm runtime unless all of `spec_name`,
	//   `spec_version`, and `authoring_version` are the same between Wasm and native.
	// This value is set to 100 to notify Polkadot-JS App (https://polkadot.js.org/apps) to use
	//   the compatible custom types.
	spec_version: 100,
	impl_version: 1,
	apis: RUNTIME_API_VERSIONS,
	transaction_version: 1,
	state_version: 1,
};

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

impl_runtime_apis! {
	impl sp_api::Core<Block> for Runtime {
		fn version() -> RuntimeVersion {
			VERSION
		}

		fn execute_block(block: Block) {
			Executive::execute_block(block);
		}

		fn initialize_block(header: &<Block as BlockT>::Header) {
			Executive::initialize_block(header)
		}
	}

	impl sp_api::Metadata<Block> for Runtime {
		fn metadata() -> OpaqueMetadata {
			OpaqueMetadata::new(Runtime::metadata().into())
		}
	}

	impl sp_block_builder::BlockBuilder<Block> for Runtime {
		fn apply_extrinsic(extrinsic: <Block as BlockT>::Extrinsic) -> ApplyExtrinsicResult {
			Executive::apply_extrinsic(extrinsic)
		}

		fn finalize_block() -> <Block as BlockT>::Header {
			Executive::finalize_block()
		}

		fn inherent_extrinsics(data: sp_inherents::InherentData) -> Vec<<Block as BlockT>::Extrinsic> {
			data.create_extrinsics()
		}

		fn check_inherents(
			block: Block,
			data: sp_inherents::InherentData,
		) -> sp_inherents::CheckInherentsResult {
			data.check_extrinsics(&block)
		}
	}

	impl sp_transaction_pool::runtime_api::TaggedTransactionQueue<Block> for Runtime {
		fn validate_transaction(
			source: TransactionSource,
			tx: <Block as BlockT>::Extrinsic,
			block_hash: <Block as BlockT>::Hash,
		) -> TransactionValidity {
			Executive::validate_transaction(source, tx, block_hash)
		}
	}

	impl sp_offchain::OffchainWorkerApi<Block> for Runtime {
		fn offchain_worker(header: &<Block as BlockT>::Header) {
			Executive::offchain_worker(header)
		}
	}

	impl sp_session::SessionKeys<Block> for Runtime {
		fn generate_session_keys(seed: Option<Vec<u8>>) -> Vec<u8> {
			opaque::SessionKeys::generate(seed)
		}

		fn decode_session_keys(
			encoded: Vec<u8>,
		) -> Option<Vec<(Vec<u8>, KeyTypeId)>> {
			opaque::SessionKeys::decode_into_raw_public_keys(&encoded)
		}
	}

	impl fg_primitives::GrandpaApi<Block> for Runtime {
		fn grandpa_authorities() -> GrandpaAuthorityList {
			Grandpa::grandpa_authorities()
		}

		fn current_set_id() -> fg_primitives::SetId {
			Grandpa::current_set_id()
		}

		fn submit_report_equivocation_unsigned_extrinsic(
			_equivocation_proof: fg_primitives::EquivocationProof<
				<Block as BlockT>::Hash,
				NumberFor<Block>,
			>,
			_key_owner_proof: fg_primitives::OpaqueKeyOwnershipProof,
		) -> Option<()> {
			None
		}

		fn generate_key_ownership_proof(
			_set_id: fg_primitives::SetId,
			_authority_id: GrandpaId,
		) -> Option<fg_primitives::OpaqueKeyOwnershipProof> {
			// NOTE: this is the only implementation possible since we've
			// defined our key owner proof type as a bottom type (i.e. a type
			// with no values).
			None
		}
	}

	impl sp_consensus_babe::BabeApi<Block> for Runtime {
		fn configuration() -> sp_consensus_babe::BabeConfiguration {
			let epoch_config = Babe::epoch_config().unwrap_or(BABE_GENESIS_EPOCH_CONFIG);
			sp_consensus_babe::BabeConfiguration {
				slot_duration: Babe::slot_duration(),
				epoch_length: EpochDuration::get(),
				c: epoch_config.c,
				authorities: Babe::authorities().to_vec(),
				randomness: Babe::randomness(),
				allowed_slots: epoch_config.allowed_slots,
			}
		}

		fn current_epoch_start() -> sp_consensus_babe::Slot {
			Babe::current_epoch_start()
		}

		fn current_epoch() -> sp_consensus_babe::Epoch {
			Babe::current_epoch()
		}

		fn next_epoch() -> sp_consensus_babe::Epoch {
			Babe::next_epoch()
		}

		fn generate_key_ownership_proof(
			_slot: sp_consensus_babe::Slot,
			authority_id: sp_consensus_babe::AuthorityId,
		) -> Option<sp_consensus_babe::OpaqueKeyOwnershipProof> {
			use codec::Encode;

			Historical::prove((sp_consensus_babe::KEY_TYPE, authority_id))
				.map(|p| p.encode())
				.map(sp_consensus_babe::OpaqueKeyOwnershipProof::new)
		}

		fn submit_report_equivocation_unsigned_extrinsic(
			equivocation_proof: sp_consensus_babe::EquivocationProof<<Block as BlockT>::Header>,
			key_owner_proof: sp_consensus_babe::OpaqueKeyOwnershipProof,
		) -> Option<()> {
			let key_owner_proof = key_owner_proof.decode()?;

			Babe::submit_unsigned_equivocation_report(
				equivocation_proof,
				key_owner_proof,
			)
		}
	}

	impl frame_system_rpc_runtime_api::AccountNonceApi<Block, AccountId, Index> for Runtime {
		fn account_nonce(account: AccountId) -> Index {
			System::account_nonce(account)
		}
	}

	impl pallet_transaction_payment_rpc_runtime_api::TransactionPaymentApi<Block, Balance> for Runtime {
		fn query_info(
			uxt: <Block as BlockT>::Extrinsic,
			len: u32,
		) -> pallet_transaction_payment_rpc_runtime_api::RuntimeDispatchInfo<Balance> {
			TransactionPayment::query_info(uxt, len)
		}
		fn query_fee_details(
			uxt: <Block as BlockT>::Extrinsic,
			len: u32,
		) -> pallet_transaction_payment::FeeDetails<Balance> {
			TransactionPayment::query_fee_details(uxt, len)
		}
	}

	impl pallet_transaction_payment_rpc_runtime_api::TransactionPaymentCallApi<Block, Balance, RuntimeCall>
		for Runtime
	{
		fn query_call_info(
			call: RuntimeCall,
			len: u32,
		) -> pallet_transaction_payment::RuntimeDispatchInfo<Balance> {
			TransactionPayment::query_call_info(call, len)
		}
		fn query_call_fee_details(
			call: RuntimeCall,
			len: u32,
		) -> pallet_transaction_payment::FeeDetails<Balance> {
			TransactionPayment::query_call_fee_details(call, len)
		}
	}

	impl pallet_identity_rpc_runtime_api::IdentityApi<Block, AccountId> for Runtime {
		fn get_user_info_by_account(
			account_id: AccountId,
		) -> Option<pallet_identity_rpc_runtime_api::UserInfo<AccountId>> {
			Identity::identity_by_id(account_id).map(|identity_info| {
				let nonce = System::account_nonce(&identity_info.account_id);
				let balance = Balances::free_balance(&identity_info.account_id);
				let trait_scores: Vec<_> = Appreciation::trait_scores_of(&identity_info.account_id)
					.into_iter()
					.map(|(community_id, trait_id, karma_score)| {
						pallet_identity_rpc_runtime_api::TraitScore {
							trait_id, karma_score, community_id
						}
					})
					.collect();
				let community_membership: Vec<_> = Appreciation::community_membership_of(&identity_info.account_id)
					.into_iter()
					.map(|(community_id, karma_score, is_admin)| pallet_identity_rpc_runtime_api::CommunityMembership {
						community_id, karma_score, is_admin
					})
					.collect();

				let karma_score = trait_scores.iter().map(|score| score.karma_score).sum::<u32>() + community_membership.len() as u32;

				pallet_identity_rpc_runtime_api::UserInfo {
					account_id: identity_info.account_id,
					nonce: nonce.into(),
					user_name: identity_info.name.into(),
					mobile_number: identity_info.number.into(),
					balance: balance as u64,
					trait_scores,
					karma_score,
					community_membership,
				}
			})
		}

		fn get_user_info_by_name(
			name: Vec<u8>,
		) -> Option<pallet_identity_rpc_runtime_api::UserInfo<AccountId>> {
			let name: BoundedVec<u8, NameLimit> = name.try_into().ok()?;
			Identity::identity_by_name(name).map(|identity_info| {
				let nonce = System::account_nonce(&identity_info.account_id);
				let balance = Balances::free_balance(&identity_info.account_id);
				let trait_scores: Vec<_> = Appreciation::trait_scores_of(&identity_info.account_id)
					.into_iter()
					.map(|(community_id, trait_id, karma_score)| {
						pallet_identity_rpc_runtime_api::TraitScore {
							trait_id, karma_score, community_id
						}
					})
					.collect();
				let community_membership: Vec<_> = Appreciation::community_membership_of(&identity_info.account_id)
					.into_iter()
					.map(|(community_id, karma_score, is_admin)| pallet_identity_rpc_runtime_api::CommunityMembership {
						community_id, karma_score, is_admin
					})
					.collect();

				let karma_score = trait_scores.iter().map(|score| score.karma_score).sum::<u32>() + community_membership.len() as u32;


				pallet_identity_rpc_runtime_api::UserInfo {
					account_id: identity_info.account_id,
					nonce: nonce.into(),
					user_name: identity_info.name.into(),
					mobile_number: identity_info.number.into(),
					balance: balance as u64,
					trait_scores,
					karma_score,
					community_membership,
				}
			})
		}

		fn get_user_info_by_number(
			number: Vec<u8>,
		) -> Option<pallet_identity_rpc_runtime_api::UserInfo<AccountId>> {
			let number: BoundedVec<u8, NumberLimit> = number.try_into().ok()?;
			Identity::identity_by_number(number).map(|identity_info| {
				let nonce = System::account_nonce(&identity_info.account_id);
				let balance = Balances::free_balance(&identity_info.account_id);
				let trait_scores: Vec<_> = Appreciation::trait_scores_of(&identity_info.account_id)
					.into_iter()
					.map(|(community_id, trait_id, karma_score)| {
						pallet_identity_rpc_runtime_api::TraitScore {
							trait_id, karma_score, community_id
						}
					})
					.collect();
				let community_membership: Vec<_> = Appreciation::community_membership_of(&identity_info.account_id)
					.into_iter()
					.map(|(community_id, karma_score, is_admin)| pallet_identity_rpc_runtime_api::CommunityMembership {
						community_id, karma_score, is_admin
					})
					.collect();

				let karma_score = trait_scores.iter().map(|score| score.karma_score).sum::<u32>() + community_membership.len() as u32;

				pallet_identity_rpc_runtime_api::UserInfo {
					account_id: identity_info.account_id,
					nonce: nonce.into(),
					user_name: identity_info.name.into(),
					mobile_number: identity_info.number.into(),
					balance: balance as u64,
					trait_scores,
					karma_score,
					community_membership,
				}
			})
		}
	}

	#[cfg(feature = "runtime-benchmarks")]
	impl frame_benchmarking::Benchmark<Block> for Runtime {
		fn benchmark_metadata(extra: bool) -> (
			Vec<frame_benchmarking::BenchmarkList>,
			Vec<frame_support::traits::StorageInfo>,
		) {
			use frame_benchmarking::{baseline, Benchmarking, BenchmarkList};
			use frame_support::traits::StorageInfoTrait;
			use frame_system_benchmarking::Pallet as SystemBench;
			use baseline::Pallet as BaselineBench;

			let mut list = Vec::<BenchmarkList>::new();
			list_benchmarks!(list, extra);

			let storage_info = AllPalletsWithSystem::storage_info();

			(list, storage_info)
		}

		fn dispatch_benchmark(
			config: frame_benchmarking::BenchmarkConfig
		) -> Result<Vec<frame_benchmarking::BenchmarkBatch>, sp_runtime::RuntimeString> {
			use frame_benchmarking::{baseline, Benchmarking, BenchmarkBatch, TrackedStorageKey};

			use frame_system_benchmarking::Pallet as SystemBench;
			use baseline::Pallet as BaselineBench;

			impl frame_system_benchmarking::Config for Runtime {}
			impl baseline::Config for Runtime {}

			use frame_support::traits::WhitelistedStorageKeys;
			let whitelist: Vec<TrackedStorageKey> = AllPalletsWithSystem::whitelisted_storage_keys();

			let mut batches = Vec::<BenchmarkBatch>::new();
			let params = (&config, &whitelist);
			add_benchmarks!(params, batches);

			Ok(batches)
		}
	}

	#[cfg(feature = "try-runtime")]
	impl frame_try_runtime::TryRuntime<Block> for Runtime {
		fn on_runtime_upgrade(checks: frame_try_runtime::UpgradeCheckSelect) -> (Weight, Weight) {
			// NOTE: intentional unwrap: we don't want to propagate the error backwards, and want to
			// have a backtrace here. If any of the pre/post migration checks fail, we shall stop
			// right here and right now.
			let weight = Executive::try_runtime_upgrade(checks).unwrap();
			(weight, BlockWeights::get().max_block)
		}

		fn execute_block(
			block: Block,
			state_root_check: bool,
			signature_check: bool,
			select: frame_try_runtime::TryStateSelect
		) -> Weight {
			// NOTE: intentional unwrap: we don't want to propagate the error backwards, and want to
			// have a backtrace here.
			Executive::try_execute_block(block, state_root_check, signature_check, select).expect("execute-block failed")
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use frame_support::traits::WhitelistedStorageKeys;
	use sp_core::hexdisplay::HexDisplay;
	use std::collections::HashSet;

	#[test]
	fn check_whitelist() {
		let whitelist: HashSet<String> = AllPalletsWithSystem::whitelisted_storage_keys()
			.iter()
			.map(|e| HexDisplay::from(&e.key).to_string())
			.collect();

		// Block Number
		assert!(
			whitelist.contains("26aa394eea5630e07c48ae0c9558cef702a5c1b19ab7a04f536c519aca4983ac")
		);
		// Total Issuance
		assert!(
			whitelist.contains("c2261276cc9d1f8598ea4b6a74b15c2f57c875e4cff74148e4628f264b974c80")
		);
		// Execution Phase
		assert!(
			whitelist.contains("26aa394eea5630e07c48ae0c9558cef7ff553b5a9862a516939d82b3d3d8661a")
		);
		// Event Count
		assert!(
			whitelist.contains("26aa394eea5630e07c48ae0c9558cef70a98fdbe9ce6c55837576c60c7af3850")
		);
		// System Events
		assert!(
			whitelist.contains("26aa394eea5630e07c48ae0c9558cef780d41e5e16056765bc8461851072c9d7")
		);
	}
}

#[cfg(test)]
mod payout_tests {
	use super::*;
	use pallet_staking::{ActiveEraInfo, EraPayout as EraPayoutT};
	use sp_staking::EraIndex;

	#[allow(dead_code)]
	pub struct TestActiveEraInfo {
		index: EraIndex,
		start: Option<u64>,
	}

	impl From<TestActiveEraInfo> for ActiveEraInfo {
		fn from(val: TestActiveEraInfo) -> ActiveEraInfo {
			unsafe { std::mem::transmute::<TestActiveEraInfo, ActiveEraInfo>(val) }
		}
	}

	fn set_era(index: EraIndex) {
		let era_info = TestActiveEraInfo { index, start: None };
		pallet_staking::ActiveEra::<Runtime>::put(ActiveEraInfo::from(era_info));
	}

	// Note: this functions assumes next:
	// during one month all era rewards are the same,
	// each month contains 30 eras
	fn check_month_payouts(month_index: u32, total_month_reward: Balance) {
		let start_era_index = month_index * 30;
		let end_era_index = (month_index + 1) * 30;

		for era_index in start_era_index..end_era_index {
			set_era(era_index);
			let (payout, rest) = EraPayout::<Staking>::era_payout(0, 0, 0);
			assert_eq!(payout, total_month_reward / 30);
			assert_eq!(rest, 0);
		}
	}

	#[test]
	fn test_era_payout() {
		let mut ext: sp_io::TestExternalities = frame_system::GenesisConfig::default()
			.build_storage::<Runtime>()
			.unwrap()
			.into();

		ext.execute_with(|| {
			check_month_payouts(0, 10_000_000_000_000);
			check_month_payouts(1, 9_799_640_000_000);
			check_month_payouts(2, 9_603_294_412_960);
			check_month_payouts(3, 9_410_882_806_101);
			check_month_payouts(4, 9_222_326_358_197);
			check_month_payouts(5, 9_037_547_827_284);
			check_month_payouts(6, 8_856_471_519_016);
			check_month_payouts(7, 8_679_023_255_660);
			check_month_payouts(8, 8_505_130_345_709);
			check_month_payouts(9, 8_334_721_554_102);
			check_month_payouts(10, 8_167_727_073_044);
			check_month_payouts(11, 8_004_078_493_408);
		});
	}
}
