use crate::{VerificationResult};
use codec::{Decode, Encode};
use scale_info::prelude::vec::Vec;
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

#[derive(Encode, Decode)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
pub struct GetBlockchainEventsResponse<TransactionEvent> {
	pub tx_events: Vec<TransactionEvent>,
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
