use crate::chain::{error::map_err, BlocksProviderApiServer};
use codec::Codec;
use jsonrpsee::core::RpcResult;
use runtime_api::chain::BlockInfoProvider;
use sc_client_api::BlockBackend;
use sp_api::{BlockT, ProvideRuntimeApi};
use sp_blockchain::HeaderBackend;
use sp_rpc::{BlockchainStats, GenesisData};
use sp_runtime::generic::SignedBlock;
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
	fn get_blockchain_data(&self) -> RpcResult<BlockchainStats> {
		let api = self.client.runtime_api();
		let at = self.client.info().best_hash;

		let blockchain_data = api
			.get_blockchain_data(at)
			.map_err(|e| map_err(e, "Failed to get blockchain data"))?;

		Ok(blockchain_data)
	}

	fn get_genesis_data(&self) -> RpcResult<GenesisData<AccountId>> {
		let api = self.client.runtime_api();
		let at = self.client.info().best_hash;

		let genesis_data =
			api.get_genesis_data(at).map_err(|e| map_err(e, "Failed to get genesis data"))?;

		Ok(genesis_data)
	}
}
