use crate::{
	identity::{AccountIdentity, IdentityInfo},
	types::Score,
};
use codec::{Decode, Encode, MaxEncodedLen};
use sp_std::fmt::Debug;

pub trait IdentityProvider<AccountId, Username, PhoneNumberHash>
where
	AccountId: Encode + Decode + MaxEncodedLen + Eq + Debug + Clone,
	Username: Encode + Decode + MaxEncodedLen + Eq + Debug + Clone,
	PhoneNumberHash: Encode + Decode + MaxEncodedLen + Eq + Debug + Clone,
{
	fn exist_by_identity(
		account_identity: &AccountIdentity<AccountId, Username, PhoneNumberHash>,
	) -> bool;

	fn get_identity_info(
		account_identity: &AccountIdentity<AccountId, Username, PhoneNumberHash>,
	) -> Option<IdentityInfo<AccountId, Username, PhoneNumberHash>> {
		match account_identity {
			AccountIdentity::AccountId(account_id) => Self::identity_by_id(account_id),
			AccountIdentity::Username(username) => Self::identity_by_name(username),
			AccountIdentity::PhoneNumberHash(phone_number_hash) =>
				Self::identity_by_number(phone_number_hash),
		}
	}

	fn identity_by_id(
		account_id: &AccountId,
	) -> Option<IdentityInfo<AccountId, Username, PhoneNumberHash>>;

	fn identity_by_name(
		name: &Username,
	) -> Option<IdentityInfo<AccountId, Username, PhoneNumberHash>>;

	fn identity_by_number(
		number: &PhoneNumberHash,
	) -> Option<IdentityInfo<AccountId, Username, PhoneNumberHash>>;
}

pub trait ScoreProvider<AccountId> {
	fn score_of(account_id: &AccountId) -> Score;
}

pub trait MaybeLowercase {
	fn to_lowercase(self) -> Self;
}
