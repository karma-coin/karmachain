use codec::{Decode, Encode};
use scale_info::{prelude::string::String, TypeInfo};
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

pub type ByPassToken = String;

#[derive(Copy, Clone, Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
pub enum VerificationResult {
	Unspecified = 0,
	/// There's already a user with the requested user name
	UserNameTaken = 1,
	/// User is verified using provided token
	Verified = 2,
	/// User is not verifier using provided token
	Unverified = 3,
	/// Request is missing required data
	MissingData = 4,
	/// Bad client signature
	InvalidSignature = 5,
	/// Different account associated with phone number
	AccountMismatch = 6,
}

impl From<i32> for VerificationResult {
	fn from(value: i32) -> Self {
		match value {
			0 => VerificationResult::Verified,
			1 => VerificationResult::Unverified,
			2 => VerificationResult::AccountMismatch,
			_ => VerificationResult::Unspecified,
		}
	}
}

#[derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
pub struct VerificationEvidence<PublicKey, AccountId, Username, PhoneNumberHash> {
	pub verifier_public_key: PublicKey,
	pub account_id: AccountId,
	pub username: Username,
	pub phone_number_hash: PhoneNumberHash,
}

#[derive(Encode, Decode)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
pub struct VerificationResponse<AccountId, Username, PhoneNumberHash> {
	pub verifier_account_id: Option<AccountId>,
	pub verification_result: VerificationResult,
	pub account_id: Option<AccountId>,
	pub phone_number_hash: Option<PhoneNumberHash>,
	pub username: Option<Username>,
	pub signature: Option<sp_core::sr25519::Signature>,
}
