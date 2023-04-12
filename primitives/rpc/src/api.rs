use crate::{SignedTransactionWithStatus, TransactionEvent};
use codec::{Decode, Encode};
use scale_info::prelude::vec::Vec;
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

#[derive(Encode, Decode)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
pub struct GetTransactionsFromHashesResponse<AccountId, Signature> {
	pub transactions: Vec<SignedTransactionWithStatus<AccountId, Signature>>,
	pub tx_events: Vec<TransactionEvent>,
}

#[derive(Encode, Decode)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
pub struct GetTransactionsResponse<AccountId, Signature> {
	pub transactions: Vec<SignedTransactionWithStatus<AccountId, Signature>>,
	pub tx_events: Vec<TransactionEvent>,
}

#[derive(Encode, Decode)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
pub struct GetTransactionResponse<AccountId, Signature> {
	pub transaction: SignedTransactionWithStatus<AccountId, Signature>,
	pub tx_events: Vec<TransactionEvent>,
}
