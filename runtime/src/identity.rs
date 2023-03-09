use crate::*;

parameter_types! {
	pub const NameLimit: u32 = 24;
	pub const NumberLimit: u32 = 12;
}

impl pallet_identity::Config for Runtime {
	/// The overarching event type.
	type RuntimeEvent = RuntimeEvent;

	type NameLimit = NameLimit;

	type NumberLimit = NameLimit;
}
