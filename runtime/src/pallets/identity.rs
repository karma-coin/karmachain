use crate::*;

parameter_types! {
	pub const NameLimit: u32 = 40;
	pub const PhoneNumberLimit: u32 = 12;
	pub const MaxPhoneVerifiers: u32 = 5;
}

pub type Username = BoundedVec<u8, NameLimit>;
pub type PhoneNumber = BoundedVec<u8, PhoneNumberLimit>;

impl pallet_identity::Config for Runtime {
	/// The overarching event type.
	type RuntimeEvent = RuntimeEvent;
	/// Max length of username
	type UsernameLimit = NameLimit;
	/// Username type
	type Username = Username;
	/// Max length of phone number
	type PhoneNumberLimit = PhoneNumberLimit;
	/// Phone number type
	type PhoneNumber = PhoneNumber;
	/// Max number of phone verifiers accounts
	type MaxPhoneVerifiers = MaxPhoneVerifiers;
	/// The currency mechanism.
	type Currency = Balances;

	type Hooks = (Appreciation, (Reward, TransactionIndexer));
}
