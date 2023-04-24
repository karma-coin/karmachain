use crate::identity::{AccountIdentity, IdentityInfo};
use codec::{Decode, Encode, MaxEncodedLen};
use sp_std::fmt::Debug;

pub trait IdentityProvider<AccountId, Username, PhoneNumber>
where
	AccountId: Encode + Decode + MaxEncodedLen + Eq + Debug + Clone,
	Username: Encode + Decode + MaxEncodedLen + Eq + Debug + Clone,
	PhoneNumber: Encode + Decode + MaxEncodedLen + Eq + Debug + Clone,
{
	fn exist_by_identity(
		account_identity: &AccountIdentity<AccountId, Username, PhoneNumber>,
	) -> bool;

	fn get_identity_info(
		account_identity: AccountIdentity<AccountId, Username, PhoneNumber>,
	) -> Option<IdentityInfo<AccountId, Username, PhoneNumber>> {
		match account_identity {
			AccountIdentity::AccountId(account_id) => Self::identity_by_id(account_id),
			AccountIdentity::Name(name) => Self::identity_by_name(name),
			AccountIdentity::PhoneNumber(phone_number) => Self::identity_by_number(phone_number),
		}
	}

	fn identity_by_id(
		account_id: AccountId,
	) -> Option<IdentityInfo<AccountId, Username, PhoneNumber>>;

	fn identity_by_name(name: Username) -> Option<IdentityInfo<AccountId, Username, PhoneNumber>>;

	fn identity_by_number(
		number: PhoneNumber,
	) -> Option<IdentityInfo<AccountId, Username, PhoneNumber>>;
}
