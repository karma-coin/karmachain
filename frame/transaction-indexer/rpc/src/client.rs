use crate::{
	error::{map_err, Error},
	traits::TransactionsApiServer,
};
use codec::{Codec, Encode};
use jsonrpsee::{
	core::RpcResult,
	types::{error::CallError, ErrorObject},
};
use sc_client_api::BlockBackend;
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_runtime::{
	generic::BlockId,
	traits::{Block as BlockT, Header},
};
use std::sync::Arc;

use pallet_transaction_indexer_rpc_runtime_api::TransactionsApi as TransactionsRuntimeApi;
use sp_rpc::{
	GetTransactionResponse, GetTransactionsResponse, SignedTransaction,
	SignedTransactionWithStatus, TransactionStatus,
};

/// Something that implement `TransactionsApiServer`
pub struct TransactionIndexer<C, P> {
	/// Shared reference to the client.
	client: Arc<C>,
	_marker: std::marker::PhantomData<P>,
}

impl<C, P> TransactionIndexer<C, P> {
	pub fn new(client: Arc<C>) -> Self {
		Self { client, _marker: Default::default() }
	}
}

impl<C, Block, AccountId, Signature, TransactionEvent>
	TransactionIndexer<C, (AccountId, Block, Signature, TransactionEvent)>
where
	Signature: Codec + Send + Sync + 'static,
	TransactionEvent: Codec + Send + Sync + 'static,
	AccountId: Codec + Clone + Send + Sync + 'static,
	Block: BlockT,
	C: BlockBackend<Block>
		+ ProvideRuntimeApi<Block>
		+ HeaderBackend<Block>
		+ Send
		+ Sync
		+ 'static,
	C::Api: TransactionsRuntimeApi<Block, AccountId, TransactionEvent>,
{
	fn get_tx_by_indexes(
		&self,
		block_number: <Block::Header as Header>::Number,
		tx_index: u32,
	) -> RpcResult<(SignedTransactionWithStatus<AccountId, Signature>, Vec<TransactionEvent>)> {
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

		let tx = txs.into_iter().nth(tx_index as usize).ok_or_else(|| {
			CallError::Custom(ErrorObject::owned(
				Error::TxNotFound.into(),
				"Transaction index out of bound",
				Option::<()>::None,
			))
		})?;

		// TODO:
		let tx = SignedTransactionWithStatus {
			signed_transaction: SignedTransaction {
				signer: None,
				transaction_body: tx.encode(),
				signature: None,
			},
			status: TransactionStatus::OnChain,
			from: None,
			to: None,
		};

		// Get block events, important in order to get block events we need to request storage
		// at block number in which this block was produced
		let events = self
			.client
			.runtime_api()
			.get_transaction_events(&BlockId::number(block_number), tx_index)
			.map_err(|e| map_err(e, "Failed to get transaction events"))?;

		Ok((tx, events))
	}
}
/// Implementing transaction RPC. Current implementation requires pallet transaction indexer in the
/// Runtime in order to index transaction by account. Also current implementation uses that
/// transaction can be identified by block number and transaction index in that block
///
/// The `C` type parameter represents client. C is expected to be a type that implements the
/// BlockBackend in order to get Block details to extract transaction details
/// The `C::Api` should provide `TransactionsRuntimeApi` in order to get account transactions for
/// `get_transactions` method and transaction block number to get transaction details from block
impl<C, Block, AccountId, Signature, TransactionEvent>
	TransactionsApiServer<AccountId, Block::Hash, Signature, TransactionEvent>
	for TransactionIndexer<C, (AccountId, Block, Signature, TransactionEvent)>
where
	Signature: Codec + Send + Sync + 'static,
	TransactionEvent: Codec + Send + Sync + 'static,
	AccountId: Codec + Clone + Send + Sync + 'static,
	Block: BlockT,
	C: BlockBackend<Block>
		+ ProvideRuntimeApi<Block>
		+ HeaderBackend<Block>
		+ Send
		+ Sync
		+ 'static,
	C::Api: TransactionsRuntimeApi<Block, AccountId, TransactionEvent>,
{
	fn get_transactions(
		&self,
		account_id: AccountId,
		at: Option<Block::Hash>,
	) -> RpcResult<GetTransactionsResponse<AccountId, Signature, TransactionEvent>> {
		let api = self.client.runtime_api();
		let at = BlockId::hash(at.unwrap_or_else(|| self.client.info().best_hash));

		let (transactions, tx_events) = api
			.get_transactions(&at, account_id)
			.map_err(|e| map_err(e, "Failed to get transactions indexes"))?
			.into_iter()
			.map(|(block_number, tx_index)| self.get_tx_by_indexes(block_number, tx_index))
			.collect::<Result<Vec<_>, _>>()?
			.into_iter()
			.unzip::<_, _, _, Vec<_>>();

		Ok(GetTransactionsResponse {
			transactions,
			tx_events: tx_events.into_iter().flatten().collect(),
		})
	}

	fn get_transaction(
		&self,
		tx_hash: Block::Hash,
		at: Option<Block::Hash>,
	) -> RpcResult<GetTransactionResponse<AccountId, Signature, TransactionEvent>> {
		let api = self.client.runtime_api();
		let at = BlockId::hash(at.unwrap_or_else(|| self.client.info().best_hash));

		// Get block number and transaction index by transaction hash
		let (block_number, tx_index) = api
			.get_transaction(&at, tx_hash)
			.map_err(|e| map_err(e, "Failed to get transaction indexes"))?
			.ok_or(CallError::Custom(ErrorObject::owned(
				Error::TxNotFound.into(),
				"Not found transaction with such hash",
				Option::<()>::None,
			)))?;

		let (transaction, tx_events) = self.get_tx_by_indexes(block_number, tx_index)?;

		Ok(GetTransactionResponse { transaction, tx_events })
	}
}