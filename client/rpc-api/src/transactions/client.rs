use crate::transactions::{
	error::{map_err, Error},
	TransactionsIndexerApiServer,
};
use codec::Codec;
use jsonrpsee::{
	core::RpcResult,
	types::{error::CallError, ErrorObject},
};
use runtime_api::{
	events::EventProvider,
	transactions::{TransactionIndexer, TransactionInfoProvider},
};
use sc_client_api::BlockBackend;
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_rpc::{GetTransactionResponse, GetTransactionsResponse, SignedTransactionWithStatus};
use sp_runtime::{
	generic::BlockId,
	traits::{Block as BlockT, NumberFor},
};
use std::sync::Arc;

pub struct TransactionsIndexer<C, P> {
	/// Shared reference to the client.
	client: Arc<C>,
	_marker: std::marker::PhantomData<P>,
}

impl<C, P> TransactionsIndexer<C, P> {
	pub fn new(client: Arc<C>) -> Self {
		Self { client, _marker: Default::default() }
	}
}

impl<C, Block, AccountId, Signature, Event>
	TransactionsIndexerApiServer<Block, AccountId, Signature, Event> for TransactionsIndexer<C, Block>
where
	Block: BlockT,
	AccountId: Codec,
	Signature: Codec,
	Event: Codec,
	C: BlockBackend<Block>
		+ ProvideRuntimeApi<Block>
		+ HeaderBackend<Block>
		+ Send
		+ Sync
		+ 'static,
	C::Api: TransactionInfoProvider<Block, Block::Extrinsic, AccountId, Signature>,
	C::Api: TransactionIndexer<Block, AccountId>,
	C::Api: EventProvider<Block, Event>,
{
	fn get_tx(
		&self,
		block_number: NumberFor<Block>,
		tx_index: u32,
	) -> RpcResult<SignedTransactionWithStatus<AccountId, Signature>> {
		let api = self.client.runtime_api();
		let at = BlockId::hash(self.client.info().best_hash);

		// Convert block number to block hash
		let block_hash = self
			.client
			.block_hash(block_number)
			.map_err(|e| map_err(e, "Failed to get block hash"))?
			.ok_or_else(|| {
				CallError::Custom(ErrorObject::owned(
					Error::BlockNotFound.into(),
					"Block with this number not found",
					Option::<()>::None,
				))
			})?;

		// Get block transactions by block hash
		let txs = self
			.client
			.block_body(block_hash)
			.map_err(|e| map_err(e, "Failed to get block body"))?
			.ok_or_else(|| {
				// Impossible error, just in case
				CallError::Custom(ErrorObject::owned(
					Error::BlockNotFound.into(),
					"Block by hash not found",
					Option::<()>::None,
				))
			})?;

		let opaque_extrinsic = txs.into_iter().nth(tx_index as usize).ok_or_else(|| {
			CallError::Custom(ErrorObject::owned(
				Error::TxNotFound.into(),
				"Transaction index out of bound",
				Option::<()>::None,
			))
		})?;

		let tx = api
			.get_transaction_info(&at, opaque_extrinsic)
			.map_err(|e| map_err(e, "Failed to get transaction details"))?;

		Ok(tx.unwrap())
	}

	fn get_tx_with_events(
		&self,
		block_number: NumberFor<Block>,
		tx_index: u32,
	) -> RpcResult<(SignedTransactionWithStatus<AccountId, Signature>, Vec<Event>)> {
		let api = self.client.runtime_api();

		let transaction = <TransactionsIndexer<C, Block> as TransactionsIndexerApiServer<
			Block,
			AccountId,
			Signature,
			Event,
		>>::get_tx(self, block_number, tx_index)
		.map_err(|e| map_err(e, "Failed to get transaction details"))?;

		let tx_events = api
			.get_transaction_events(&BlockId::number(block_number), tx_index)
			.map_err(|e| map_err(e, "Failed to get transaction events"))?;

		Ok((transaction, tx_events))
	}

	fn get_transaction(
		&self,
		tx_hash: Block::Hash,
	) -> RpcResult<GetTransactionResponse<AccountId, Signature, Event>> {
		let api = self.client.runtime_api();
		let at = BlockId::hash(self.client.info().best_hash);

		// Get block number and transaction index by transaction hash
		let (block_number, tx_index) = api
			.get_transaction(&at, tx_hash)
			.map_err(|e| map_err(e, "Failed to get transaction indexes"))?
			.ok_or(CallError::Custom(ErrorObject::owned(
				Error::TxNotFound.into(),
				"Not found transaction with such hash",
				Option::<()>::None,
			)))?;

		let (transaction, tx_events) =
			<TransactionsIndexer<C, Block> as TransactionsIndexerApiServer<
				Block,
				AccountId,
				Signature,
				Event,
			>>::get_tx_with_events(self, block_number, tx_index)?;

		Ok(GetTransactionResponse { transaction, tx_events })
	}

	fn get_transactions(
		&self,
		account_id: AccountId,
	) -> RpcResult<GetTransactionsResponse<AccountId, Signature, Event>> {
		let api = self.client.runtime_api();
		let at = BlockId::hash(self.client.info().best_hash);

		let (transactions, tx_events) = api
			.get_transactions_by_account(&at, account_id)
			.map_err(|e| map_err(e, "Failed to get transactions indexes"))?
			.into_iter()
			.map(|(block_number, tx_index)| self.get_tx_with_events(block_number, tx_index))
			.collect::<Result<Vec<_>, _>>()?
			.into_iter()
			.unzip::<_, _, _, Vec<_>>();

		Ok(GetTransactionsResponse {
			transactions,
			tx_events: tx_events.into_iter().flatten().collect(),
		})
	}
}
