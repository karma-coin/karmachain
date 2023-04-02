use crate::*;
use frame_support::traits::ConstU32;
use pallet_grandpa::AuthorityId as GrandpaId;
use sp_core::crypto::KeyTypeId;

impl pallet_grandpa::Config for Runtime {
	/// The event type of this module.
	type RuntimeEvent = RuntimeEvent;
	/// The proof of key ownership, used for validating equivocation reports
	/// The proof must include the session index and validator count of the
	/// session at which the equivocation occurred.
	type KeyOwnerProofSystem = ();
	/// The identification of a key owner, used when reporting equivocations.
	type KeyOwnerProof =
		<Self::KeyOwnerProofSystem as KeyOwnerProofSystem<(KeyTypeId, GrandpaId)>>::Proof;
	/// A system for proving ownership of keys, i.e. that a given key was part
	/// of a validator set, needed for validating equivocation reports.
	type KeyOwnerIdentification = <Self::KeyOwnerProofSystem as KeyOwnerProofSystem<(
		KeyTypeId,
		GrandpaId,
	)>>::IdentificationTuple;
	/// The equivocation handling subsystem, defines methods to report an
	/// offence (after the equivocation has been validated) and for submitting a
	/// transaction to report an equivocation (from an offchain context).
	/// NOTE: when enabling equivocation handling (i.e. this type isn't set to
	/// `()`) you must use this pallet's `ValidateUnsigned` in the runtime
	/// definition.
	type HandleEquivocation = ();
	/// Weights for this pallet.
	type WeightInfo = ();
	/// Max Authorities in use
	type MaxAuthorities = ConstU32<32>;
}
