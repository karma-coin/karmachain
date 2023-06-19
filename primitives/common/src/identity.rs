use codec::{Decode, Encode, MaxEncodedLen};
use scale_info::TypeInfo;
use sp_std::fmt::Debug;

#[derive(Clone, Encode, Decode, MaxEncodedLen, TypeInfo, Debug, PartialEq, Eq)]
pub enum AccountIdentity<AccountId, Username, PhoneNumberHash>
where
	AccountId: Encode + Decode + MaxEncodedLen + Eq + Debug + Clone,
	Username: Encode + Decode + MaxEncodedLen + Eq + Debug + Clone,
	PhoneNumberHash: Encode + Decode + MaxEncodedLen + Eq + Debug + Clone,
{
	AccountId(AccountId),
	Username(Username),
	PhoneNumberHash(PhoneNumberHash),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IdentityInfo<AccountId, Username, PhoneNumberHash>
where
	AccountId: Debug + Clone + PartialEq,
	Username: Debug + Clone + PartialEq,
	PhoneNumberHash: Debug + Clone + PartialEq,
{
	pub account_id: AccountId,
	pub username: Username,
	pub phone_number_hash: PhoneNumberHash,
}
