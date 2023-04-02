use crate::*;

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
