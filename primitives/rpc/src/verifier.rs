use codec::{Decode, Encode};
use scale_info::TypeInfo;
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

#[derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
pub struct VerificationEvidence<PublicKey, AccountId, Username, PhoneNumberHash> {
	pub verifier_public_key: PublicKey,
	pub account_id: AccountId,
	pub username: Username,
	pub phone_number_hash: PhoneNumberHash,
}
