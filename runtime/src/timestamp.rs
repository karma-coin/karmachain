use crate::*;

impl pallet_timestamp::Config for Runtime {
	/// Type used for expressing timestamp.
	/// A timestamp: milliseconds since the unix epoch.
	type Moment = u64;
	/// Something which can be notified when the timestamp is set.
	/// Set this to `()` if not needed.
	type OnTimestampSet = Babe;
	/// The minimum period between blocks. Beware that this is different to the *expected*
	/// period that the block production apparatus provides. Your chosen consensus system will
	/// generally work with this to determine a sensible block time. e.g. For Aura, it will be
	/// double this period on default settings.
	type MinimumPeriod = ConstU64<{ SLOT_DURATION / 2 }>;
	/// Weight information for extrinsics in this pallet.
	type WeightInfo = ();
}
