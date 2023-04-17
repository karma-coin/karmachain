#![cfg_attr(not(feature = "std"), no_std)]

use codec::Codec;
use scale_info::prelude::vec::Vec;
use sp_runtime::traits::Header;

pub use sp_rpc::*;

sp_api::decl_runtime_apis! {
	pub trait TransactionsApi<AccountId: Codec, TransactionEvent: Codec> {
		/// This method takes an AccountId as input and returns a vector of tuples, where each tuple
		/// contains two elements: the block number and the transaction index. These tuples represent
		/// all the transactions associated with the specified account_id.
		fn get_transactions(account_id: AccountId) -> Vec<(<Block::Header as Header>::Number, u32)>;
		/// This method takes a tx_hash as input and returns an Option type that contains a tuple of the
		/// block number and transaction index associated with the specified hash. If no transaction is
		/// found, None is returned.
		fn get_transaction(tx_hash: Block::Hash) -> Option<(<Block::Header as Header>::Number, u32)>;
		/// Returns a vector of TransactionEvents. These events are associated with the block
		/// at the specified index.
		///
		/// To use this method properly you need to specify block at which this transaction happen
		/// this parameter will be automatically generated by `decl_runtime_apis!` macro
		fn get_block_events() -> Vec<TransactionEvent>;
		/// This method takes a tx_index as input and returns a vector of TransactionEvents. These
		/// events are associated with the transaction at the specified index.
		///
		/// To use this method properly you need to specify block at which this transaction happen
		/// this parameter will be automatically generated by `decl_runtime_apis!` macro
		fn get_transaction_events(tx_index: u32) -> Vec<TransactionEvent>;
	}
}
