use crate::*;

impl pallet_session::Config for Runtime {
	/// The overarching event type.
	type RuntimeEvent = RuntimeEvent;
	/// A stable ID for a validator.
	type ValidatorId = AccountId;
	/// A conversion from account ID to validator ID.
	///
	/// Its cost must be at most one storage read.
	type ValidatorIdOf = pallet_staking::StashOf<Self>;
	/// Indicator for when to end the session.
	type ShouldEndSession = Babe;
	/// Something that can predict the next session rotation. This should typically come from
	/// the same logical unit that provides [`ShouldEndSession`], yet, it gives a best effort
	/// estimate. It is helpful to implement [`EstimateNextNewSession`].
	type NextSessionRotation = Babe;
	/// Handler for managing new session.
	type SessionManager = pallet_session::historical::NoteHistoricalRoot<Self, Staking>;
	/// Handler when a session has changed.
	type SessionHandler = <opaque::SessionKeys as OpaqueKeys>::KeyTypeIdProviders;
	/// The keys.
	type Keys = opaque::SessionKeys;
	/// Weight information for extrinsics in this pallet.
	type WeightInfo = pallet_session::weights::SubstrateWeight<Runtime>;
}
