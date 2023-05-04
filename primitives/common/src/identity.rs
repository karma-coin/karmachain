use codec::{Decode, Encode, MaxEncodedLen};
use scale_info::TypeInfo;
use sp_std::fmt::Debug;

#[derive(Clone, Encode, Decode, MaxEncodedLen, TypeInfo, Debug, PartialEq, Eq)]
pub enum AccountIdentity<AccountId, Username, PhoneNumber>
where
	AccountId: Encode + Decode + MaxEncodedLen + Eq + Debug + Clone,
	Username: Encode + Decode + MaxEncodedLen + Eq + Debug + Clone,
	PhoneNumber: Encode + Decode + MaxEncodedLen + Eq + Debug + Clone,
{
	AccountId(AccountId),
	Name(Username),
	PhoneNumber(PhoneNumber),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IdentityInfo<AccountId, Username, PhoneNumber>
where
	AccountId: Debug + Clone + PartialEq,
	Username: Debug + Clone + PartialEq,
	PhoneNumber: Debug + Clone + PartialEq,
{
	pub account_id: AccountId,
	pub name: Username,
	pub number: PhoneNumber,
}
