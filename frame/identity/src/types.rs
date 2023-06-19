use codec::{Codec, Decode, Encode, MaxEncodedLen};
use scale_info::TypeInfo;

#[derive(Clone, Encode, Decode, Eq, MaxEncodedLen, PartialEq, Debug, TypeInfo)]
pub struct IdentityStore<Username, PhoneNumberHash, Moment>
where
	Username: Codec,
	PhoneNumberHash: Codec,
	Moment: Codec,
{
	pub username: Username,
	pub phone_number_hash: PhoneNumberHash,
	pub registration_time: Option<Moment>,
}

pub enum VerificationResult {
	/// Parameters are valid
	Valid,
	/// Parameters are valid but lead to account data migration
	Migration,
	/// This `AccountId` belong to another user
	AccountIdExists,
	/// This `Username` belong to another user
	UsernameExists,
}
