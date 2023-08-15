use crate::identity::UserInfo;
use codec::{Decode, Encode};
use scale_info::{prelude::vec::Vec, TypeInfo};
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

#[derive(Encode, Decode, TypeInfo, Default)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
pub enum TransactionStatus {
	#[default]
	Unknown = 0_isize,
	NotSubmitted,
	Submitted,
	Rejected,
	OnChain,
}

#[derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
pub struct SignedTransaction<AccountId, Signature> {
	pub signer: Option<AccountId>,
	pub transaction_body: Vec<u8>,
	pub signature: Option<Signature>,
}

#[derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
pub struct SignedTransactionWithStatus<AccountId, Signature, TransactionEvent> {
	pub signed_transaction: SignedTransaction<AccountId, Signature>,
	pub status: TransactionStatus,
	pub from: Option<UserInfo<AccountId>>,
	pub to: Option<UserInfo<AccountId>>,
	pub timestamp: u64,
	pub events: Vec<TransactionEvent>,
	pub block_number: u32,
	pub transaction_index: u32,
}
