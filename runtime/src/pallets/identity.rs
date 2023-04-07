use crate::*;

parameter_types! {
	pub const NameLimit: u32 = 40;
	pub const PhoneNumberLimit: u32 = 12;
	pub const MaxPhoneVerifiers: u32 = 5;
}

impl pallet_identity::Config for Runtime {
	/// The overarching event type.
	type RuntimeEvent = RuntimeEvent;

	type NameLimit = NameLimit;

	type PhoneNumberLimit = PhoneNumberLimit;

	type MaxPhoneVerifiers = MaxPhoneVerifiers;

	type OnNewUser = Appreciation;
}
