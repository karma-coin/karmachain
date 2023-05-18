pub mod client;
mod error;

use jsonrpsee::{core::RpcResult, proc_macros::rpc};
use sp_rpc::{
	GetTransactionResponse, GetTransactionsFromHashesResponse, GetTransactionsResponse,
	SignedTransactionWithStatus,
};
use sp_runtime::traits::{Block as BlockT, NumberFor};

#[rpc(client, server)]
pub trait TransactionsIndexerApi<Block: BlockT, AccountId, Signature, Event> {
	/// RPC method provides transaction details by block number and transaction index
	#[method(name = "transactions_getTx", blocking)]
	fn get_tx(
		&self,
		block_number: NumberFor<Block>,
		tx_index: u32,
	) -> RpcResult<SignedTransactionWithStatus<AccountId, Signature>>;

	/// RPC method provides transaction details and transaction events by block number and
	/// transaction index
	#[method(name = "transactions_getTxWithEvents", blocking)]
	fn get_tx_with_events(
		&self,
		block_number: NumberFor<Block>,
		tx_index: u32,
	) -> RpcResult<(SignedTransactionWithStatus<AccountId, Signature>, Vec<Event>)>;

	/// RPC method provides transaction details by transaction hash
	#[method(name = "transactions_getTransaction", blocking)]
	fn get_transaction(
		&self,
		tx_hash: Block::Hash,
	) -> RpcResult<GetTransactionResponse<AccountId, Signature, Event>>;

	/// RPC method provides transactions details by transactions hashes
	#[method(name = "transactions_getTransactionFromHashes", blocking)]
	fn get_transactions_from_hashes(
		&self,
		tx_hashes: Vec<Block::Hash>,
	) -> RpcResult<GetTransactionsFromHashesResponse<AccountId, Signature, Event>> {
		// Default implementation use `get_transaction` method from `Self`
		// and apply this method to each hash in `tx_hashes`
		let (transactions, tx_events) = tx_hashes
			.into_iter()
			.map(|hash| self.get_transaction(hash))
			.collect::<Result<Vec<_>, _>>()?
			.into_iter()
			.map(|v| (v.transaction, v.tx_events))
			.unzip::<_, _, _, Vec<_>>();

		Ok(GetTransactionsFromHashesResponse {
			transactions,
			tx_events: tx_events.into_iter().flatten().collect(),
		})
	}

	/// RPC method provides transactions, that belong to specific account
	#[method(name = "transactions_getTransactions", blocking)]
	fn get_transactions(
		&self,
		account_id: AccountId,
	) -> RpcResult<GetTransactionsResponse<AccountId, Signature, Event>>;
}
