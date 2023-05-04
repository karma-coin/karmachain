use codec::Codec;
use scale_info::prelude::vec::Vec;

sp_api::decl_runtime_apis! {
	pub trait EventProvider<Event>
	where
		Event: Codec,
	{
		/// Returns a vector of TransactionEvents. These events are associated with the block
		/// at the specified index.
		///
		/// To use this method properly you need to specify block at which this transaction happen
		/// this parameter will be automatically generated by `decl_runtime_apis!` macro
		fn get_block_events() -> Vec<Event>;
		/// This method takes a tx_index as input and returns a vector of TransactionEvents. These
		/// events are associated with the transaction at the specified index.
		///
		/// To use this method properly you need to specify block at which this transaction happen
		/// this parameter will be automatically generated by `decl_runtime_apis!` macro
		fn get_transaction_events(tx_index: u32) -> Vec<Event>;
	}
}
