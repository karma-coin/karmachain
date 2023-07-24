use crate::*;

parameter_types! {
	pub const SessionsPerEra: SessionIndex = ERA_DURATION_IN_EPOCH;
	pub const BondingDuration: sp_staking::EraIndex = 28;
	pub const SlashDeferDuration: sp_staking::EraIndex = 27;
	pub const MaxNominatorRewardedPerValidator: u32 = 512;
	pub const OffendingValidatorsThreshold: Perbill = Perbill::from_percent(17);
	pub const MaxNominations: u32 = <NposCompactSolution16 as frame_election_provider_support::NposSolution>::LIMIT as u32;
}

pub struct StakingBenchmarkingConfig;
impl pallet_staking::BenchmarkingConfig for StakingBenchmarkingConfig {
	type MaxValidators = ConstU32<1000>;
	type MaxNominators = ConstU32<1000>;
}

impl pallet_staking::Config for Runtime {
	/// Maximum number of nominations per nominator.
	type MaxNominations = MaxNominations;
	/// The staking balance.
	type Currency = Balances;
	/// Just the `Currency::Balance` type; we have this item to allow us to constrain it to
	/// `From<u64>`.
	type CurrencyBalance = Balance;
	/// Time used for computing era duration.
	///
	/// It is guaranteed to start being called from the first `on_finalize`. Thus value at
	/// genesis is not used.
	type UnixTime = Timestamp;
	/// Convert a balance into a number used for election calculation. This must fit into a
	/// `u64` but is allowed to be sensibly lossy. The `u64` is used to communicate with the
	/// [`frame_election_provider_support`] crate which accepts u64 numbers and does operations
	/// in 128.
	/// Consequently, the backward convert is used convert the u128s from sp-elections back to a
	/// [`BalanceOf`].
	type CurrencyToVote = CurrencyToVote;
	/// Tokens have been minted and are unused for validator-reward.
	type RewardRemainder = (); // TODO:
	/// The overarching event type.
	type RuntimeEvent = RuntimeEvent;
	/// Handler for the unbalanced reduction when slashing a staker.
	type Slash = (); // TODO:
	/// Handler for the unbalanced increment when rewarding a staker.
	/// NOTE: in most cases, the implementation of `OnUnbalanced` should modify the total
	/// issuance.
	type Reward = (); // rewards are minted from the void
	/// Number of sessions per era.
	type SessionsPerEra = SessionsPerEra;
	/// Number of eras that staked funds must remain bonded for.
	type BondingDuration = BondingDuration;
	/// Number of eras that slashes are deferred by, after computation.
	///
	/// This should be less than the bonding duration. Set to 0 if slashes
	/// should be applied immediately, without opportunity for intervention.
	type SlashDeferDuration = SlashDeferDuration;
	/// The origin which can manage less critical staking parameters that does not require root.
	///
	/// Supported actions: (1) cancel deferred slash, (2) set minimum commission.
	type AdminOrigin = EnsureRoot<Self::AccountId>; // TODO:
	/// Interface for interacting with a session pallet.
	type SessionInterface = Self;
	/// The payout for validators and the system for the current era.
	type EraPayout = EraPayout<Staking>;
	/// The maximum number of nominators rewarded for each validator.
	///
	/// For each validator only the `$MaxNominatorRewardedPerValidator` biggest stakers can
	/// claim their reward. This used to limit the i/o cost for the nominator payout.
	type MaxNominatorRewardedPerValidator = MaxNominatorRewardedPerValidator;
	/// The fraction of the validator set that is safe to be offending.
	/// After the threshold is reached a new era will be forced.
	type OffendingValidatorsThreshold = OffendingValidatorsThreshold;
	/// Something that can estimate the next session change, accurately or as a best effort
	/// guess.
	type NextNewSession = Session;
	/// Something that provides the election functionality.
	type ElectionProvider = ElectionProviderMultiPhase;
	/// Something that provides the election functionality at genesis.
	type GenesisElectionProvider = onchain::OnChainExecution<OnChainSeqPhragmen>;
	/// Something that provides a best-effort sorted list of voters aka electing nominators,
	/// used for NPoS election.
	///
	/// The changes to nominators are reported to this. Moreover, each validator's self-vote is
	/// also reported as one independent vote.
	///
	/// To keep the load off the chain as much as possible, changes made to the staked amount
	/// via rewards and slashes are not reported and thus need to be manually fixed by the
	/// staker. In case of `bags-list`, this always means using `rebag` and `putInFrontOf`.
	///
	/// Invariant: what comes out of this list will always be a nominator.
	type VoterList = VoterList;
	/// Something that provides a best-effort sorted list of targets aka electable validators,
	/// used for NPoS election.
	type TargetList = UseValidatorsMap<Self>;
	/// The maximum number of `unlocking` chunks a [`StakingLedger`] can
	/// have. Effectively determines how many unique eras a staker may be
	/// unbonding in.
	///
	/// Note: `MaxUnlockingChunks` is used as the upper bound for the
	/// `BoundedVec` item `StakingLedger.unlocking`. Setting this value
	/// lower than the existing value can lead to inconsistencies in the
	/// `StakingLedger` and will need to be handled properly in a runtime
	/// migration. The test `reducing_max_unlocking_chunks_abrupt` shows
	/// this effect.
	type MaxUnlockingChunks = frame_support::traits::ConstU32<32>;
	/// Number of eras to keep in history.
	///
	/// Following information is kept for eras in `[current_era -
	/// HistoryDepth, current_era]`: `ErasStakers`, `ErasStakersClipped`,
	/// `ErasValidatorPrefs`, `ErasValidatorReward`, `ErasRewardPoints`,
	/// `ErasTotalStake`, `ErasStartSessionIndex`,
	/// `StakingLedger.claimed_rewards`.
	///
	/// Must be more than the number of eras delayed by session.
	/// I.e. active era must always be in history. I.e. `active_era >
	/// current_era - history_depth` must be guaranteed.
	///
	/// If migrating an existing pallet from storage value to config value,
	/// this should be set to same value or greater as in storage.
	///
	/// Note: `HistoryDepth` is used as the upper bound for the `BoundedVec`
	/// item `StakingLedger.claimed_rewards`. Setting this value lower than
	/// the existing value can lead to inconsistencies in the
	/// `StakingLedger` and will need to be handled properly in a migration.
	/// The test `reducing_history_depth_abrupt` shows this effect.
	type HistoryDepth = frame_support::traits::ConstU32<84>;
	/// Some parameters of the benchmarking.
	type BenchmarkingConfig = StakingBenchmarkingConfig;
	/// A hook called when any staker is slashed. Mostly likely this can be a no-op unless
	/// other pallets exist that are affected by slashing per-staker.
	type OnStakerSlash = (); // TODO:
	/// Weight information for extrinsics in this pallet.
	type WeightInfo = pallet_staking::weights::SubstrateWeight<Runtime>;
}
