use codec::{Decode, Encode, MaxEncodedLen};
use scale_info::TypeInfo;

#[derive(Default, Decode, Encode, MaxEncodedLen, TypeInfo)]
pub struct UserVerificationData<PublicKey, AccountId, Username, PhoneNumber> {
	pub verifier_public_key: PublicKey,
	pub account_id: AccountId,
	pub username: Username,
	pub phone_number: PhoneNumber,
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
