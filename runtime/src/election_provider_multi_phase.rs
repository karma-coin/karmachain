use crate::*;

impl pallet_election_provider_multi_phase::MinerConfig for Runtime {
	/// The account id type.
	type AccountId = AccountId;
	/// Maximum length of the solution that the miner is allowed to generate.
	///
	/// Solutions are trimmed to respect this.
	type MaxLength = OffchainSolutionLengthLimit;
	/// Maximum weight of the solution that the miner is allowed to generate.
	///
	/// Solutions are trimmed to respect this.
	///
	/// The weight is computed using `solution_weight`.
	type MaxWeight = OffchainSolutionWeightLimit;
	/// The solution that the miner is mining.
	type Solution = NposCompactSolution16;
	/// Maximum number of votes per voter in the snapshots.
	type MaxVotesPerVoter = <
		<Self as pallet_election_provider_multi_phase::Config>::DataProvider
		as
		frame_election_provider_support::ElectionDataProvider
	>::MaxVotesPerVoter;
	/// Something that can compute the weight of a solution.
	///
	/// This weight estimate is then used to trim the solution, based on [`MinerConfig::MaxWeight`].
	///
	/// The unsigned submissions have to respect the weight of the submit_unsigned call, thus their
	/// weight estimate function is wired to this call's weight.
	fn solution_weight(v: u32, t: u32, a: u32, d: u32) -> Weight {
		<
			<Self as pallet_election_provider_multi_phase::Config>::WeightInfo
			as
			pallet_election_provider_multi_phase::WeightInfo
		>::submit_unsigned(v, t, a, d)
	}
}

/// The numbers configured here should always be more than the the maximum limits of staking pallet
/// to ensure election snapshot will not run out of memory.
pub struct BenchmarkConfig;
impl pallet_election_provider_multi_phase::BenchmarkingConfig for BenchmarkConfig {
	const VOTERS: [u32; 2] = [5_000, 10_000];
	const TARGETS: [u32; 2] = [1_000, 2_000];
	const ACTIVE_VOTERS: [u32; 2] = [1000, 4_000];
	const DESIRED_TARGETS: [u32; 2] = [400, 800];
	const SNAPSHOT_MAXIMUM_VOTERS: u32 = 25_000;
	const MINER_MAXIMUM_VOTERS: u32 = 15_000;
	const MAXIMUM_TARGETS: u32 = 2000;
}

impl pallet_election_provider_multi_phase::Config for Runtime {
	/// The overarching event type.
	type RuntimeEvent = RuntimeEvent;
	/// Currency type.
	type Currency = Balances;
	/// Something that can predict the fee of a call. Used to sensibly distribute rewards.
	type EstimateCallFee = TransactionPayment;
	/// Duration of the signed phase.
	type SignedPhase = SignedPhase;
	/// Duration of the unsigned phase.
	type UnsignedPhase = UnsignedPhase;
	/// Maximum number of signed submissions that can be queued.
	///
	/// It is best to avoid adjusting this during an election, as it impacts downstream data
	/// structures. In particular, `SignedSubmissionIndices<T>` is bounded on this value. If you
	/// update this value during an election, you _must_ ensure that
	/// `SignedSubmissionIndices.len()` is less than or equal to the new value. Otherwise,
	/// attempts to submit new solutions may cause a runtime panic.
	type SignedMaxSubmissions = SignedMaxSubmissions;
	/// The maximum amount of unchecked solutions to refund the call fee for.
	type SignedMaxRefunds = SignedMaxRefunds;
	/// Base reward for a signed solution
	type SignedRewardBase = SignedRewardBase;
	/// Base deposit for a signed solution.
	type SignedDepositBase = SignedDepositBase;
	/// Per-byte deposit for a signed solution.
	type SignedDepositByte = SignedDepositByte;
	/// Per-weight deposit for a signed solution.
	type SignedDepositWeight = ();
	/// Maximum weight of a signed solution.
	///
	/// If [`Config::MinerConfig`] is being implemented to submit signed solutions (outside of
	/// this pallet), then [`MinerConfig::solution_weight`] is used to compare against
	/// this value.
	type SignedMaxWeight =
		<Self::MinerConfig as pallet_election_provider_multi_phase::MinerConfig>::MaxWeight;
	/// Configurations of the embedded miner.
	///
	/// Any external software implementing this can use the [`unsigned::Miner`] type provided,
	/// which can mine new solutions and trim them accordingly.
	type MinerConfig = Self;
	/// Handler for the slashed deposits.
	type SlashHandler = (); // burn slashes
	/// Handler for the rewards.
	type RewardHandler = (); // nothing to do upon rewards
	/// The minimum amount of improvement to the solution score that defines a solution as
	/// "better" in the Unsigned phase.
	type BetterUnsignedThreshold = BetterUnsignedThreshold;
	/// The minimum amount of improvement to the solution score that defines a solution as
	/// "better" in the Signed phase.
	type BetterSignedThreshold = ();
	/// The repeat threshold of the offchain worker.
	///
	/// For example, if it is 5, that means that at least 5 blocks will elapse between attempts		/// to submit the worker's solution.
	type OffchainRepeat = OffchainRepeat;
	/// The priority of the unsigned transaction submitted in the unsigned-phase
	type MinerTxPriority = NposSolutionPriority;
		/// Something that will provide the election data.
	type DataProvider = Staking;
	/// Configuration for the fallback.
	type Fallback = frame_election_provider_support::NoElection<(
		AccountId,
		BlockNumber,
		Staking,
		MaxActiveValidators,
	)>;
	/// Configuration of the governance-only fallback.
	///
	/// As a side-note, it is recommend for test-nets to use `type ElectionProvider =
	/// BoundedExecution<_>` if the test-net is not expected to have thousands of nominators.
	type GovernanceFallback = onchain::OnChainExecution<OnChainSeqPhragmen>;
	/// OCW election solution miner algorithm implementation.
	type Solver = SequentialPhragmen<
		AccountId,
		pallet_election_provider_multi_phase::SolutionAccuracyOf<Self>,
		(),
	>;
	/// The configuration of benchmarking.
	type BenchmarkingConfig = BenchmarkConfig;
	/// Origin that can control this pallet. Note that any action taken by this origin (such)
	/// as providing an emergency solution is not checked. Thus, it must be a trusted origin.
	type ForceOrigin = EnsureRoot<AccountId>;
	/// The weight of the pallet.
	type WeightInfo = pallet_election_provider_multi_phase::weights::SubstrateWeight<Self>;
	/// The maximum number of electing voters to put in the snapshot. At the moment, snapshots
	/// are only over a single block, but once multi-block elections are introduced they will
	/// take place over multiple blocks.
	type MaxElectingVoters = MaxElectingVoters;
	/// The maximum number of electable targets to put in the snapshot.
	type MaxElectableTargets = MaxElectableTargets;
	/// The maximum number of winners that can be elected by this `ElectionProvider`
	/// implementation.
	///
	/// Note: This must always be greater or equal to `T::DataProvider::desired_targets()`.
	type MaxWinners = MaxActiveValidators;
}