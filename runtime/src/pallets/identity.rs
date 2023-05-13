use crate::*;
use sp_common::bounded_string::BoundedString;

parameter_types! {
	pub const NameLimit: u32 = 40;
	pub const PhoneNumberLimit: u32 = 12;
	pub const MaxPhoneVerifiers: u32 = 5;
}

pub type Username = BoundedString<NameLimit>;
pub type PhoneNumber = BoundedString<PhoneNumberLimit>;

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
	///
	type Hooks = (Appreciation, (Reward, TransactionIndexer));
	/// The currency mechanism.
	type Currency = Balances;
	/// Signature that used by `PhoneVerifier`
	type Signature = sp_core::sr25519::Signature;
	/// This is required by the `Signature` type.
	type PublicKey = sp_core::sr25519::Public;
}
