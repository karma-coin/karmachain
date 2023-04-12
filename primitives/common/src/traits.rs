use frame_support::pallet_prelude::*;

pub trait TransactionIndexer<AccountId> {
	/// Index currently executing transaction
	fn index_transaction(account_id: AccountId) -> DispatchResult;
}
