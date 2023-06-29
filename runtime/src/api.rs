use crate::{validators_rewards::era_payout, *};
use codec::{Decode, Encode};
use frame_system::Phase;
use pallet_identity::types::VerificationResult as IdentityVerificationResult;
use pallet_transaction_payment_rpc_runtime_api::{FeeDetails, RuntimeDispatchInfo};
use sp_common::{types::CommunityId, BoundedString};
use sp_rpc::{
	BlockchainStats, CharTrait, CommunityMembership, Contact, GenesisData, PhoneVerifier,
	SignedTransaction, SignedTransactionWithStatus, TraitScore, TransactionStatus, UserInfo,
	VerificationResult,
};
use sp_runtime::{generic::SignedBlock, traits::StaticLookup};

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

	impl pallet_transaction_payment_rpc_runtime_api::TransactionPaymentApi<
		Block,
		Balance,
	> for Runtime {
		fn query_info(uxt: <Block as BlockT>::Extrinsic, len: u32) -> RuntimeDispatchInfo<Balance> {
			TransactionPayment::query_info(uxt, len)
		}
		fn query_fee_details(uxt: <Block as BlockT>::Extrinsic, len: u32) -> FeeDetails<Balance> {
			TransactionPayment::query_fee_details(uxt, len)
		}
		fn query_weight_to_fee(weight: Weight) -> Balance {
			TransactionPayment::weight_to_fee(weight)
		}
		fn query_length_to_fee(length: u32) -> Balance {
			TransactionPayment::length_to_fee(length)
		}
	}

	impl pallet_transaction_payment_rpc_runtime_api::TransactionPaymentCallApi<Block, Balance, RuntimeCall>
		for Runtime
	{
		fn query_call_info(call: RuntimeCall, len: u32) -> RuntimeDispatchInfo<Balance> {
			TransactionPayment::query_call_info(call, len)
		}
		fn query_call_fee_details(call: RuntimeCall, len: u32) -> FeeDetails<Balance> {
			TransactionPayment::query_call_fee_details(call, len)
		}
		fn query_weight_to_fee(weight: Weight) -> Balance {
			TransactionPayment::weight_to_fee(weight)
		}
		fn query_length_to_fee(length: u32) -> Balance {
			TransactionPayment::length_to_fee(length)
		}
	}

	impl runtime_api::chain::BlockInfoProvider<Block, SignedBlock<Block>, AccountId, Hash> for Runtime {
		fn get_blockchain_data() -> BlockchainStats {
			let tip_height = System::block_number().into();
			let transaction_count = pallet_transaction_indexer::TransactionsCount::<Runtime>::get();
			let payment_transaction_count = pallet_transaction_indexer::PaymentTransactionsCount::<Runtime>::get();
			let appreciations_transactions_count = pallet_transaction_indexer::AppreciationTransactionsCount::<Runtime>::get();
			let update_user_transactions_count = pallet_transaction_indexer::UpdateUserTransactionsCount::<Runtime>::get();
			let users_count = pallet_identity::IdentityOf::<Runtime>::count().into();
			let fees_amount = 0; // TODO:
			let minted_amount = Reward::total_rewarded();
			let circulation = Reward::total_rewarded();
			let fee_subs_count = 0; // TODO:
			let fee_subs_amount = pallet_reward::TxFeeSubsidiesTotalAllocated::<Runtime>::get();
			let signup_rewards_count = 0; // TODO:
			let signup_rewards_amount = pallet_reward::SignupRewardTotalAllocated::<Runtime>::get();
			let referral_rewards_count = 0; // TODO:
			let referral_rewards_amount = pallet_reward::ReferralRewardTotalAllocated::<Runtime>::get();
			let validator_rewards_count = 0;
			let validator_rewards_amount = (0..Staking::current_era().unwrap_or_default())
				.map(era_payout)
				.sum();
			let causes_rewards_amount = 0;

			BlockchainStats {
				last_block_time: MILLISECS_PER_BLOCK,
				tip_height,
				transaction_count,
				payment_transaction_count,
				appreciations_transactions_count,
				update_user_transactions_count,
				users_count,
				fees_amount,
				minted_amount,
				circulation,
				fee_subs_count,
				fee_subs_amount,
				signup_rewards_count,
				signup_rewards_amount,
				referral_rewards_count,
				referral_rewards_amount,
				validator_rewards_count,
				validator_rewards_amount,
				causes_rewards_amount,
			}
		}

		fn get_genesis_data() -> GenesisData<AccountId> {
			let net_id = 0; // TODO:
			let net_name = vec![]; // TODO:
			let genesis_time = 0; // TODO:

			let signup_reward_phase1_alloc = pallet_reward::SignupRewardPhase1Alloc::<Runtime>::get();
			let signup_reward_phase2_alloc = pallet_reward::SignupRewardPhase2Alloc::<Runtime>::get();

			let signup_reward_phase1_amount = pallet_reward::SignupRewardPhase1Amount::<Runtime>::get();
			let signup_reward_phase2_amount = pallet_reward::SignupRewardPhase2Amount::<Runtime>::get();
			// TODO: Q: what `start` means?
			let signup_reward_phase3_start = pallet_reward::SignupRewardPhase3Amount::<Runtime>::get();

			let referral_reward_phase1_alloc = pallet_reward::ReferralRewardPhase1Alloc::<Runtime>::get();
			let referral_reward_phase2_alloc = pallet_reward::ReferralRewardPhase2Alloc::<Runtime>::get();

			let referral_reward_phase1_amount = pallet_reward::ReferralRewardPhase1Amount::<Runtime>::get();
			let referral_reward_phase2_amount = pallet_reward::ReferralRewardPhase2Amount::<Runtime>::get();

			let tx_fee_subsidy_max_per_user = pallet_reward::TxFeeSubsidyMaxPerUser::<Runtime>::get().into();
			let tx_fee_subsidies_alloc = pallet_reward::TxFeeSubsidiesAlloc::<Runtime>::get();
			let tx_fee_subsidy_max_amount = pallet_reward::TxFeeSubsidyMaxAmount::<Runtime>::get();

			let block_reward_amount = 0; // TODO:
			let block_reward_last_block = 0; // TODO:

			let karma_reward_amount = pallet_reward::KarmaRewardAmount::<Runtime>::get();
			let karma_reward_alloc = pallet_reward::MaxKarmaRewardAlloc::<Runtime>::get();
			let karma_reward_top_n_users = pallet_reward::KarmaRewardUsersParticipates::<Runtime>::get().into();

			// let treasury_premint_amount = 0; // TODO:
			// let treasury_account_id = todo!(); // TODO:
			// let treasury_account_name = vec![]; // TODO:

			let char_traits = Appreciation::char_traits()
				.into_iter()
				.map(|v| CharTrait {
					id: v.id,
					name: v.name.try_into().unwrap_or_default(),
					emoji: v.emoji.try_into().unwrap_or_default(),
				})
				.collect();
			let verifiers = Identity::verifiers()
				.into_iter()
				.map(|v| PhoneVerifier {
					account_id: v,
					name: Default::default(), // TODO:
				})
				.collect();

			GenesisData {
				net_id,
				net_name,
				genesis_time,
				signup_reward_phase1_alloc,
				signup_reward_phase2_alloc,
				signup_reward_phase1_amount,
				signup_reward_phase2_amount,
				signup_reward_phase3_start,
				referral_reward_phase1_alloc,
				referral_reward_phase2_alloc,
				referral_reward_phase1_amount,
				referral_reward_phase2_amount,
				tx_fee_subsidy_max_per_user,
				tx_fee_subsidies_alloc,
				tx_fee_subsidy_max_amount,
				block_reward_amount,
				block_reward_last_block,
				karma_reward_amount,
				karma_reward_alloc,
				karma_reward_top_n_users,
				char_traits,
				verifiers,
			}
		}
	}

	impl runtime_api::events::EventProvider<Block, RuntimeEvent> for Runtime {
		fn get_block_events() -> Vec<RuntimeEvent> {
			// Just ask pallet System for events
			System::read_events_no_consensus().map(|v| v.event).collect()
		}

		fn get_transaction_events(tx_index: u32) -> Vec<RuntimeEvent> {
			// Just ask pallet System for events and then filter by extrinsic index
			 // in order to get only that transaction events
			System::read_events_no_consensus()
				.filter(|v| matches!(v.phase, Phase::ApplyExtrinsic(index) if index == tx_index))
				.map(|v| v.event).collect()
		}
	}

	impl runtime_api::identity::IdentityApi<Block, AccountId, Username, PhoneNumber> for Runtime {
		fn get_user_info_by_account(
			account_id: AccountId,
		) -> Option<UserInfo<AccountId>> {
			Identity::identity_by_id(&account_id).map(|identity_info| {
				let nonce = System::account_nonce(&identity_info.account_id);
				let balance = Balances::free_balance(&identity_info.account_id);
				let trait_scores: Vec<_> = Appreciation::trait_scores_of(&identity_info.account_id)
					.into_iter()
					.map(|(community_id, trait_id, karma_score)| {
						TraitScore {
							trait_id, karma_score, community_id
						}
					})
					.collect();
				let community_membership: Vec<_> = Appreciation::community_membership_of(&identity_info.account_id)
					.into_iter()
					.map(|(community_id, karma_score, is_admin)| CommunityMembership {
						community_id, karma_score, is_admin
					})
					.collect();

				let karma_score = trait_scores.iter().map(|score| score.karma_score).sum::<u32>() + community_membership.len() as u32;

				UserInfo {
					account_id: identity_info.account_id,
					nonce: nonce.into(),
					user_name: identity_info.name.try_into().unwrap_or_default(),
					mobile_number: identity_info.number.try_into().unwrap_or_default(),
					balance: balance as u64,
					trait_scores,
					karma_score,
					community_membership,
				}
			})
		}

		fn get_user_info_by_name(
			name: BoundedString<NameLimit>,
		) -> Option<UserInfo<AccountId>> {
			Identity::identity_by_name(&name).map(|identity_info| {
				let nonce = System::account_nonce(&identity_info.account_id);
				let balance = Balances::free_balance(&identity_info.account_id);
				let trait_scores: Vec<_> = Appreciation::trait_scores_of(&identity_info.account_id)
					.into_iter()
					.map(|(community_id, trait_id, karma_score)| {
						TraitScore {
							trait_id, karma_score, community_id
						}
					})
					.collect();
				let community_membership: Vec<_> = Appreciation::community_membership_of(&identity_info.account_id)
					.into_iter()
					.map(|(community_id, karma_score, is_admin)| CommunityMembership {
						community_id, karma_score, is_admin
					})
					.collect();

				let karma_score = trait_scores.iter().map(|score| score.karma_score).sum::<u32>() + community_membership.len() as u32;


				UserInfo {
					account_id: identity_info.account_id,
					nonce: nonce.into(),
					user_name: identity_info.name.try_into().unwrap_or_default(),
					mobile_number: identity_info.number.try_into().unwrap_or_default(),
					balance: balance as u64,
					trait_scores,
					karma_score,
					community_membership,
				}
			})
		}

		fn get_user_info_by_number(
			number: BoundedString<PhoneNumberLimit>,
		) -> Option<UserInfo<AccountId>> {
			Identity::identity_by_number(&number).map(|identity_info| {
				let nonce = System::account_nonce(&identity_info.account_id);
				let balance = Balances::free_balance(&identity_info.account_id);
				let trait_scores: Vec<_> = Appreciation::trait_scores_of(&identity_info.account_id)
					.into_iter()
					.map(|(community_id, trait_id, karma_score)| {
						TraitScore {
							trait_id, karma_score, community_id
						}
					})
					.collect();
				let community_membership: Vec<_> = Appreciation::community_membership_of(&identity_info.account_id)
					.into_iter()
					.map(|(community_id, karma_score, is_admin)| CommunityMembership {
						community_id, karma_score, is_admin
					})
					.collect();

				let karma_score = trait_scores.iter().map(|score| score.karma_score).sum::<u32>() + community_membership.len() as u32;

				UserInfo {
					account_id: identity_info.account_id,
					nonce: nonce.into(),
					user_name: identity_info.name.try_into().unwrap_or_default(),
					mobile_number: identity_info.number.try_into().unwrap_or_default(),
					balance: balance as u64,
					trait_scores,
					karma_score,
					community_membership,
				}
			})
		}

		fn get_all_users(
			community_id: CommunityId,
		) -> Vec<UserInfo<AccountId>> {
			pallet_appreciation::CommunityMembership::<Runtime>::iter()
				.filter(|(_, id, _)| *id == community_id)
				.flat_map(|(account_id, _, _)| Self::get_user_info_by_account(account_id))
				.collect()
		}

		fn get_contacts(
			prefix: BoundedString<NameLimit>,
			community_id: Option<CommunityId>,
		) -> Vec<Contact<AccountId>> {
			Identity::get_contacts(prefix)
				.into_iter()
				.filter(|(account_id, _)| {
					// If `community_id` provided filter by it
					community_id
						.map(|community_id|
							pallet_appreciation::CommunityMembership::<Runtime>::get(account_id, community_id)
								.is_some()
						)
						.unwrap_or(true)
				})
				.map(|(account_id, identity_store)| {
					let trait_scores: Vec<_> = Appreciation::trait_scores_of(&account_id)
						.into_iter()
						.map(|(community_id, trait_id, karma_score)| {
							TraitScore {
								trait_id, karma_score, community_id
							}
						})
						.collect();
					let community_membership: Vec<_> = Appreciation::community_membership_of(&account_id)
						.into_iter()
						.map(|(community_id, karma_score, is_admin)| CommunityMembership {
							community_id, karma_score, is_admin
						})
						.collect();

					Contact {
						user_name: identity_store.name.try_into().unwrap_or_default(),
						account_id,
						mobile_number: identity_store.phone_number.try_into().unwrap_or_default(),
						community_membership,
						trait_scores,
					}
				})
				.collect()
		}
	}

	impl runtime_api::transactions::TransactionInfoProvider<Block, opaque::UncheckedExtrinsic, AccountId, Signature> for Runtime
	{
		fn get_transaction_info(opaque_extrinsic: opaque::UncheckedExtrinsic) -> Option<SignedTransactionWithStatus<AccountId, Signature>> {
			use runtime_api::identity::runtime_decl_for_identity_api::IdentityApiV1;

			// Convert `OpaqueExtrinsic` into bytes and then decode `UncheckedExtrinsic` from that bytes
			let transaction_body = opaque_extrinsic.encode();
			let mut bytes = transaction_body.as_slice();
			let extrinsic = UncheckedExtrinsic::decode(&mut bytes).ok()?;

			let (address, signature) = extrinsic.signature
				.map(|(address, signature, _extra)| (address, signature))
				.unzip();

			// Convert `Address` into `AccountId`
			let signer = address
				.map(<Runtime as frame_system::Config>::Lookup::lookup)
				.transpose()
				.ok()?;

			// Get info about transaction sender and receiver
			let from = signer.clone().and_then(Self::get_user_info_by_account);
			let to = extrinsic.function.get_recipient().and_then(|account_identity| {
				match account_identity {
					AccountIdentity::AccountId(account_id) => Self::get_user_info_by_account(account_id),
					AccountIdentity::Name(name) => Self::get_user_info_by_name(name),
					AccountIdentity::PhoneNumber(phone_number) => Self::get_user_info_by_number(phone_number),
				}
			});

			Some(SignedTransactionWithStatus {
				signed_transaction: SignedTransaction {
					signer,
					transaction_body,
					signature,
				},
				status: TransactionStatus::OnChain,
				from,
				to,
			})
		}
	}

	impl runtime_api::transactions::TransactionIndexer<Block, AccountId> for Runtime {
		fn get_transactions_by_account(account_id: AccountId) -> Vec<(BlockNumber, u32)> {
			TransactionIndexer::accounts_tx(account_id).unwrap_or_default()
		}

		fn get_transaction(tx_hash: Hash) -> Option<(BlockNumber, u32)> {
			TransactionIndexer::tx_block_and_index(tx_hash)
		}
	}

	impl runtime_api::verifier::VerifierApi<Block, AccountId, Username, PhoneNumber> for Runtime {
		fn verify(
			account_id: &AccountId,
			username: &Username,
			phone_number: &PhoneNumber,
		) -> VerificationResult {
			match Identity::verify(account_id, username, phone_number) {
				IdentityVerificationResult::Valid => VerificationResult::Verified,
				IdentityVerificationResult::Migration => VerificationResult::Verified,
				IdentityVerificationResult::AccountIdExists => VerificationResult::AccountMismatch,
				IdentityVerificationResult::UsernameExists => VerificationResult::UserNameTaken,
			}
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
