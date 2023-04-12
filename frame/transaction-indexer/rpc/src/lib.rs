use std::sync::Arc;

use codec::{Codec, Encode};
use jsonrpsee::{
	core::RpcResult,
	proc_macros::rpc,
	types::error::{CallError, ErrorObject},
};
use sc_client_api::BlockBackend;
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_runtime::{
	generic::BlockId,
	traits::{Block as BlockT, Header},
};

pub use pallet_transaction_indexer_rpc_runtime_api::TransactionsApi as TransactionsRuntimeApi;

mod error;

use error::{map_err, Error};

#[rpc(client, server)]
pub trait TransactionsApi<AccountId, Hash: Clone> {
	#[method(name = "transaction_get_transaction_from_hashes", blocking)]
	fn get_transactions_from_hashes(
		&self,
		tx_hashes: Vec<Hash>,
		at: Option<Hash>,
	) -> RpcResult<Vec<Vec<u8>>> {
		tx_hashes
			.into_iter()
			.map(|hash| self.get_transaction(hash, at.clone()))
			.collect()
	}

	#[method(name = "transaction_get_transactions", blocking)]
	fn get_transactions(&self, account_id: AccountId, at: Option<Hash>) -> RpcResult<Vec<Vec<u8>>>;

	#[method(name = "transaction_get_transaction", blocking)]
	fn get_transaction(&self, tx_hash: Hash, at: Option<Hash>) -> RpcResult<Vec<u8>>;
}

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

impl<C, Block> TransactionIndexer<C, Block>
where
	Block: BlockT,
	C: ProvideRuntimeApi<Block> + HeaderBackend<Block> + Send + Sync + 'static,
	C: BlockBackend<Block>,
{
	fn get_tx_by_indexes(
		&self,
		block_number: <Block::Header as Header>::Number,
		tx_index: u32,
	) -> RpcResult<Block::Extrinsic> {
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

		Ok(tx)
	}
}

impl<C, Block, AccountId> TransactionsApiServer<AccountId, Block::Hash>
	for TransactionIndexer<C, Block>
where
	AccountId: Codec + Clone,
	Block: BlockT,
	C: BlockBackend<Block>
		+ ProvideRuntimeApi<Block>
		+ HeaderBackend<Block>
		+ Send
		+ Sync
		+ 'static,
	C::Api: TransactionsRuntimeApi<Block, AccountId>,
{
	fn get_transactions(
		&self,
		account_id: AccountId,
		at: Option<Block::Hash>,
	) -> RpcResult<Vec<Vec<u8>>> {
		let api = self.client.runtime_api();
		let at = BlockId::hash(at.unwrap_or_else(|| self.client.info().best_hash));

		let txs = api
			.get_transactions(&at, account_id)
			.map_err(|e| map_err(e, "Failed to get transactions indexes"))?
			.into_iter()
			.map(|(block_number, tx_index)| {
				self.get_tx_by_indexes(block_number, tx_index).map(|v| v.encode())
			})
			.collect::<Result<Vec<_>, _>>()?;

		Ok(txs)
	}

	fn get_transaction(&self, tx_hash: Block::Hash, at: Option<Block::Hash>) -> RpcResult<Vec<u8>> {
		let api = self.client.runtime_api();
		let at = BlockId::hash(at.unwrap_or_else(|| self.client.info().best_hash));

		let (block_number, tx_index) = api
			.get_transaction(&at, tx_hash)
			.map_err(|e| map_err(e, "Failed to get transaction indexes"))?
			.ok_or(CallError::Custom(ErrorObject::owned(
				Error::TxNotFound.into(),
				"Not found transaction with such hash",
				Option::<()>::None,
			)))?;

		let tx = self.get_tx_by_indexes(block_number, tx_index)?.encode();

		Ok(tx)
	}
}
