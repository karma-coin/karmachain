use frame_support::pallet_prelude::*;

pub trait TransactionIndexer<AccountId> {
	/// Index currently executing transaction
	fn index_transaction(account_id: AccountId) -> DispatchResult;
}

pub trait ExtrinsicData<Signature, AccountId> {
	///
	fn signature(self) -> Option<Signature>;
	///
	fn signer(self) -> Option<AccountId>;
	///
	fn receiver(self) -> Option<AccountId>;
}
