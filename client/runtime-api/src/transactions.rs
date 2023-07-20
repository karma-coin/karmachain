use codec::Codec;
use scale_info::prelude::vec::Vec;
use sp_rpc::SignedTransactionWithStatus;
use sp_runtime::traits::Header;

sp_api::decl_runtime_apis! {
	/// Substrate is build in the way that the runtime and the node are separate from each other.
	/// This trait needs to fetch information about transaction in node RPC client
	pub trait TransactionInfoProvider<OpaqueExtrinsic, AccountId, Signature, TransactionEvent>
	where
		OpaqueExtrinsic: Codec,
		AccountId: Codec,
		Signature: Codec,
		TransactionEvent: Codec,
	{
		/// Provide additional information about extrinsic
		fn get_transaction_info(opaque_extrinsic: OpaqueExtrinsic, tx_index: u32) -> Option<SignedTransactionWithStatus<AccountId, Signature, TransactionEvent>>;
	}

	pub trait TransactionIndexer<AccountId, PhoneNumberHash>
	where
		AccountId: Codec,
		PhoneNumberHash: Codec,
	{
		/// This method takes an AccountId as input and returns a vector of tuples, where each tuple
		/// contains two elements: the block number and the transaction index. These tuples represent
		/// all the transactions associated with the specified account_id.
		fn get_transactions_by_account(account_id: AccountId) -> Vec<(<Block::Header as Header>::Number, u32)>;
		/// This method takes an PhoneNumberHash as input and returns a vector of tuples, where each tuple
		/// contains two elements: the block number and the transaction index. These tuples represent
		/// all the transactions associated with the specified phone number hash.
		fn get_transactions_by_phone_number_hash(phone_number_hash: PhoneNumberHash) -> Vec<(<Block::Header as Header>::Number, u32)>;
		/// This method takes a tx_hash as input and returns an Option type that contains a tuple of the
		/// block number and transaction index associated with the specified hash. If no transaction is
		/// found, None is returned.
		fn get_transaction(tx_hash: Block::Hash) -> Option<(<Block::Header as Header>::Number, u32)>;
	}
}
