use crate::chain::{
	error::{map_err, Error},
	BlocksProviderApiServer,
};
use codec::Codec;
use jsonrpsee::{
	core::RpcResult,
	types::{error::CallError, ErrorObject},
};
use runtime_api::chain::BlockInfoProvider;
use sc_client_api::BlockBackend;
use sp_api::{BlockT, ProvideRuntimeApi};
use sp_blockchain::HeaderBackend;
use sp_rpc::Block as RpcBlock;
use sp_runtime::generic::{BlockId, SignedBlock};
use std::sync::Arc;

pub struct BlocksProvider<C, P> {
	/// Shared reference to the client.
	client: Arc<C>,
	_marker: std::marker::PhantomData<P>,
}

impl<C, P> BlocksProvider<C, P> {
	pub fn new(client: Arc<C>) -> Self {
		Self { client, _marker: Default::default() }
	}
}

impl<C, Block, AccountId> BlocksProviderApiServer<Block, AccountId> for BlocksProvider<C, Block>
where
	Block: BlockT,
	AccountId: Codec,
	C: ProvideRuntimeApi<Block>
		+ BlockBackend<Block>
		+ HeaderBackend<Block>
		+ Send
		+ Sync
		+ 'static,
	C::Api: BlockInfoProvider<Block, SignedBlock<Block>, AccountId, Block::Hash>,
{
	fn get_block_info(&self, block_height: u32) -> RpcResult<RpcBlock<AccountId, Block::Hash>> {
		let api = self.client.runtime_api();

		let block_hash = self
			.client
			.block_hash(block_height.into())
			.map_err(|e| map_err(e, "Failed to get block hashes"))?
			.ok_or(CallError::Custom(ErrorObject::owned(
				Error::BlockNotFound.into(),
				"Block hash not found",
				Option::<()>::None,
			)))?;

		let block = self
			.client
			.block(block_hash)
			.map_err(|e| map_err(e, "Failed to get blocks"))?
			.ok_or(CallError::Custom(ErrorObject::owned(
				Error::BlockNotFound.into(),
				"Block by hash not found",
				Option::<()>::None,
			)))?;

		let block_info = api
			.get_block_info(&BlockId::Number(block_height.into()), block)
			.map_err(|e| map_err(e, "Failed to get block info"))?;

		Ok(block_info)
	}
}
