use crate::{
	pallets::{
		babe::ReportLongevity,
		staking::{BondingDuration, SessionsPerEra},
	},
	*,
};
use frame_support::traits::ConstU32;
use pallet_grandpa::AuthorityId as GrandpaId;
use sp_core::crypto::KeyTypeId;

parameter_types! {
	pub const MaxSetIdSessionEntries: u32 = BondingDuration::get() * SessionsPerEra::get();
}

impl pallet_grandpa::Config for Runtime {
	/// The event type of this module.
	type RuntimeEvent = RuntimeEvent;
	/// Weights for this pallet.
	type WeightInfo = ();
	/// Max Authorities in use
	type MaxAuthorities = ConstU32<32>;
	/// The maximum number of entries to keep in the set id to session index mapping.
	///
	/// Since the `SetIdSession` map is only used for validating equivocations this
	/// value should relate to the bonding duration of whatever staking system is
	/// being used (if any). If equivocation handling is not enabled then this value
	/// can be zero.
	type MaxSetIdSessionEntries = MaxSetIdSessionEntries;
	/// The proof of key ownership, used for validating equivocation reports
	/// The proof include the session index and validator count of the
	/// session at which the equivocation occurred.
	type KeyOwnerProof = <Historical as KeyOwnerProofSystem<(KeyTypeId, GrandpaId)>>::Proof;
	/// The equivocation handling subsystem, defines methods to check/report an
	/// offence and for submitting a transaction to report an equivocation
	/// (from an offchain context).
	type EquivocationReportSystem =
		pallet_grandpa::EquivocationReportSystem<Self, Offences, Historical, ReportLongevity>;
}
