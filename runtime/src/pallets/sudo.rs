use crate::*;

impl pallet_sudo::Config for Runtime {
	/// The overarching event type.
	type RuntimeEvent = RuntimeEvent;
	/// A sudo-able call.
	type RuntimeCall = RuntimeCall;
}
