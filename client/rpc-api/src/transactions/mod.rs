pub mod client;
mod error;

use jsonrpsee::{core::RpcResult, proc_macros::rpc};
use sp_rpc::{
	SignedTransactionWithStatus,
};
use sp_runtime::traits::{Block as BlockT, NumberFor};

#[rpc(client, server)]
pub trait TransactionsIndexerApi<Block: BlockT, AccountId, Signature, Event, PhoneNumberHash> {
	/// RPC method provides transaction details by block number and transaction index
	#[method(name = "transactions_getTx", blocking)]
	fn get_tx(
		&self,
		block_number: NumberFor<Block>,
		tx_index: u32,
	) -> RpcResult<SignedTransactionWithStatus<AccountId, Signature, Event>>;

	/// RPC method provides transaction details by transaction hash
	#[method(name = "transactions_getTransaction", blocking)]
	fn get_transaction(
		&self,
		tx_hash: Block::Hash,
	) -> RpcResult<SignedTransactionWithStatus<AccountId, Signature, Event>>;

	/// RPC method provides transactions details by transactions hashes
	#[method(name = "transactions_getTransactionFromHashes", blocking)]
	fn get_transactions_from_hashes(
		&self,
		tx_hashes: Vec<Block::Hash>,
	) -> RpcResult<Vec<SignedTransactionWithStatus<AccountId, Signature, Event>>> {
		// Default implementation use `get_transaction` method from `Self`
		// and apply this method to each hash in `tx_hashes`
		let txs = tx_hashes
			.into_iter()
			.map(|hash| self.get_transaction(hash))
			.collect::<Result<Vec<_>, _>>()?;

		Ok(txs)
	}

	/// RPC method provides transactions, that belong to specific account
	#[method(name = "transactions_getTransactions", blocking)]
	fn get_transactions_by_account_id(
		&self,
		account_id: AccountId,
	) -> RpcResult<Vec<SignedTransactionWithStatus<AccountId, Signature, Event>>>;

	/// RPC method provides transactions, that belong to specific phone number hash
	#[method(name = "transactions_getTransactionsByPhoneNumberHash", blocking)]
	fn get_transactions_by_phone_number_hash(
		&self,
		phone_number_hash: PhoneNumberHash,
	) -> RpcResult<Vec<SignedTransactionWithStatus<AccountId, Signature, Event>>>;
}
