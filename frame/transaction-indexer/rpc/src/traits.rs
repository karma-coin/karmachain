use jsonrpsee::{core::RpcResult, proc_macros::rpc};
use sp_rpc::{GetTransactionResponse, GetTransactionsFromHashesResponse, GetTransactionsResponse};

#[rpc(client, server)]
pub trait TransactionsApi<AccountId, Hash: Clone, Signature, TransactionEvent> {
	/// RPC method provides transactions, that belong to specific account
	#[method(name = "transaction_get_transactions", blocking)]
	fn get_transactions(
		&self,
		account_id: AccountId,
		at: Option<Hash>,
	) -> RpcResult<GetTransactionsResponse<AccountId, Signature, TransactionEvent>>;

	/// RPC method provides transaction details by transaction hash
	#[method(name = "transaction_get_transaction", blocking)]
	fn get_transaction(
		&self,
		tx_hash: Hash,
		at: Option<Hash>,
	) -> RpcResult<GetTransactionResponse<AccountId, Signature, TransactionEvent>>;

	/// RPC method provides transactions details by transactions hashes
	#[method(name = "transaction_get_transaction_from_hashes", blocking)]
	fn get_transactions_from_hashes(
		&self,
		tx_hashes: Vec<Hash>,
		at: Option<Hash>,
	) -> RpcResult<GetTransactionsFromHashesResponse<AccountId, Signature, TransactionEvent>> {
		// Default implementation use `get_transaction` method from `Self`
		// and apply this method to each hash in `tx_hashes`
		let (transactions, tx_events) = tx_hashes
			.into_iter()
			.map(|hash| self.get_transaction(hash, at.clone()))
			.collect::<Result<Vec<_>, _>>()?
			.into_iter()
			.map(|v| (v.transaction, v.tx_events))
			.unzip::<_, _, _, Vec<_>>();

		Ok(GetTransactionsFromHashesResponse {
			transactions,
			tx_events: tx_events.into_iter().flatten().collect(),
		})
	}
}
