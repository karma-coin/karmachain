use crate::{
	pallets::staking::{BondingDuration, SessionsPerEra},
	*,
};

/// The BABE epoch configuration at genesis.
pub const BABE_GENESIS_EPOCH_CONFIG: sp_consensus_babe::BabeEpochConfiguration =
	sp_consensus_babe::BabeEpochConfiguration {
		// 1 in 4 blocks (on average, not counting collisions) will be primary babe blocks.
		// The choice of is done in accordance to the slot duration and expected target
		// block time, for safely resisting network delays of maximum two seconds.
		// <https://research.web3.foundation/en/latest/polkadot/BABE/Babe/#6-practical-results>
		c: (1, 4),
		allowed_slots: sp_consensus_babe::AllowedSlots::PrimaryAndSecondaryVRFSlots,
	};

parameter_types! {
	pub const EpochDuration: u64 = EPOCH_DURATION_IN_SLOTS as u64;
	pub const ExpectedBlockTime: u64 = MILLISECS_PER_BLOCK;
	pub const MaxAuthorities: u32 = 100_000;
	pub const ReportLongevity: u64 =
		BondingDuration::get() as u64 * SessionsPerEra::get() as u64 * EpochDuration::get();
}

impl pallet_babe::Config for Runtime {
	/// The amount of time, in slots, that each epoch should last.
	/// NOTE: Currently it is not possible to change the epoch duration after
	/// the chain has started. Attempting to do so will brick block production.
	type EpochDuration = EpochDuration;
	/// The expected average block time at which BABE should be creating
	/// blocks. Since BABE is probabilistic it is not trivial to figure out
	/// what the expected average block time should be based on the slot
	/// duration and the security parameter `c` (where `1 - c` represents
	/// the probability of a slot being empty).
	type ExpectedBlockTime = ExpectedBlockTime;
	/// BABE requires some logic to be triggered on every block to query for whether an epoch
	/// has ended and to perform the transition to the next epoch.
	///
	/// Typically, the `ExternalTrigger` type should be used. An internal trigger should only be
	/// used when no other module is responsible for changing authority set.
	type EpochChangeTrigger = pallet_babe::ExternalTrigger;
	/// A way to check whether a given validator is disabled and should not be authoring blocks.
	/// Blocks authored by a disabled validator will lead to a panic as part of this module's
	/// initialization.
	type DisabledValidators = Session;
	/// Weights for this pallet.
	type WeightInfo = ();
	/// Max number of authorities allowed.
	type MaxAuthorities = MaxAuthorities;
	/// The proof of key ownership, used for validating equivocation reports.
	/// The proof must include the session index and validator count of the
	/// session at which the equivocation occurred.
	type KeyOwnerProof =
		<Historical as KeyOwnerProofSystem<(KeyTypeId, pallet_babe::AuthorityId)>>::Proof;
	/// The equivocation handling subsystem, defines methods to check/report an
	/// offence and for submitting a transaction to report an equivocation
	/// (from an offchain context).
	type EquivocationReportSystem =
		pallet_babe::EquivocationReportSystem<Self, Offences, Historical, ReportLongevity>;
}
