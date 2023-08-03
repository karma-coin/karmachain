use super::treasury::TreasuryPalletId;
use crate::*;
use sp_common::BoundedString;

parameter_types! {
	pub const NameLimit: u32 = 40;
	pub const PhoneNumberLimit: u32 = 12;
	pub const MaxPhoneVerifiers: u32 = 5;
}

pub type Username = BoundedString<NameLimit>;
pub type PhoneNumber = BoundedString<PhoneNumberLimit>;
pub type PhoneNumberHash = sp_core::H512;

impl pallet_identity::Config for Runtime {
	/// The overarching event type.
	type RuntimeEvent = RuntimeEvent;
	/// Username type
	type Username = Username;
	/// Phone number type
	type PhoneNumberHash = PhoneNumberHash;
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
	/// Treasury account id to get funds from deleted accounts
	type Treasury = TreasuryPalletId;
}
