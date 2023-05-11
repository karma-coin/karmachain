use crate::{SignedTransactionWithStatus, VerificationResult};
use codec::{Decode, Encode};
use scale_info::prelude::vec::Vec;
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

#[derive(Encode, Decode)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
pub struct GetTransactionsFromHashesResponse<AccountId, Signature, TransactionEvent> {
	pub transactions: Vec<SignedTransactionWithStatus<AccountId, Signature>>,
	pub tx_events: Vec<TransactionEvent>,
}

#[derive(Encode, Decode)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
pub struct GetTransactionsResponse<AccountId, Signature, TransactionEvent> {
	pub transactions: Vec<SignedTransactionWithStatus<AccountId, Signature>>,
	pub tx_events: Vec<TransactionEvent>,
}

#[derive(Encode, Decode)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
pub struct GetTransactionResponse<AccountId, Signature, TransactionEvent> {
	pub transaction: SignedTransactionWithStatus<AccountId, Signature>,
	pub tx_events: Vec<TransactionEvent>,
}

#[derive(Encode, Decode)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
pub struct GetBlockchainEventsResponse<TransactionEvent> {
	pub tx_events: Vec<TransactionEvent>,
}

#[derive(Encode, Decode)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
pub struct VerificationResponse<AccountId, Username, PhoneNumber> {
	pub verifier_account_id: AccountId,
	pub verification_result: VerificationResult,
	pub account_id: AccountId,
	pub phone_number: PhoneNumber,
	pub username: Username,
	pub signature: sp_core::sr25519::Signature,
}
