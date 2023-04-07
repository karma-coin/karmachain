use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::{
	sp_std, traits::Get, BoundedVec, CloneNoBound, EqNoBound, PartialEqNoBound, RuntimeDebugNoBound,
};
use scale_info::TypeInfo;
use sp_std::fmt::Debug;

#[derive(
	CloneNoBound,
	Encode,
	Decode,
	MaxEncodedLen,
	TypeInfo,
	RuntimeDebugNoBound,
	PartialEqNoBound,
	EqNoBound,
)]
#[codec(mel_bound())]
#[scale_info(skip_type_params(PhoneNumberLimit, NameLimit))]
pub enum AccountIdentity<AccountId, NameLimit: Get<u32>, PhoneNumberLimit: Get<u32>>
where
	AccountId: Encode + Decode + MaxEncodedLen + Eq + Debug + Clone,
{
	AccountId(AccountId),
	PhoneNumber(BoundedVec<u8, PhoneNumberLimit>),
	Name(BoundedVec<u8, NameLimit>),
}

#[derive(RuntimeDebugNoBound, CloneNoBound, PartialEqNoBound, Eq)]
pub struct IdentityInfo<
	AccountId: Debug + Clone + PartialEq,
	NameLimit: Get<u32>,
	PhoneNumberLimit: Get<u32>,
> {
	pub account_id: AccountId,
	pub name: BoundedVec<u8, NameLimit>,
	pub number: BoundedVec<u8, PhoneNumberLimit>,
}

pub trait IdentityProvider<
	AccountId: Encode + Decode + MaxEncodedLen + Eq + Debug + Clone,
	NameLimit: Get<u32>,
	PhoneNumberLimit: Get<u32>,
>
{
	fn exist_by_identity(
		account_identity: &AccountIdentity<AccountId, NameLimit, PhoneNumberLimit>,
	) -> bool;

	fn identity_by_id(
		account_id: AccountId,
	) -> Option<IdentityInfo<AccountId, NameLimit, PhoneNumberLimit>>;

	fn identity_by_name(
		name: BoundedVec<u8, NameLimit>,
	) -> Option<IdentityInfo<AccountId, NameLimit, PhoneNumberLimit>>;

	fn identity_by_number(
		number: BoundedVec<u8, PhoneNumberLimit>,
	) -> Option<IdentityInfo<AccountId, NameLimit, PhoneNumberLimit>>;
}
