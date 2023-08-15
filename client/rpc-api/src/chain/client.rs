use crate::chain::{error::map_err, ChainDataProviderApiServer};
use codec::Codec;
use jsonrpsee::core::RpcResult;
use runtime_api::chain::ChainDataProvider as RuntimeChainDataProvider;
use sc_client_api::BlockBackend;
use sp_api::{BlockT, ProvideRuntimeApi};
use sp_blockchain::HeaderBackend;
use sp_rpc::{BlockchainStats, CharTrait, GenesisData};
use sp_runtime::generic::SignedBlock;
use std::sync::Arc;

pub struct ChainDataProvider<C, P> {
	/// Shared reference to the client.
	client: Arc<C>,
	network_id: String,
	_marker: std::marker::PhantomData<P>,
}

impl<C, P> ChainDataProvider<C, P> {
	pub fn new(client: Arc<C>, network_id: String) -> Self {
		Self { client, network_id, _marker: Default::default() }
	}
}

impl<C, Block, AccountId> ChainDataProviderApiServer<Block, AccountId>
	for ChainDataProvider<C, Block>
where
	Block: BlockT,
	AccountId: Codec,
	C: ProvideRuntimeApi<Block>
		+ BlockBackend<Block>
		+ HeaderBackend<Block>
		+ Send
		+ Sync
		+ 'static,
	C::Api: RuntimeChainDataProvider<Block, SignedBlock<Block>, AccountId, Block::Hash>,
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

	fn get_network_id(&self) -> RpcResult<String> {
		Ok(self.network_id.clone())
	}

	fn get_char_traits(
		&self,
		from_index: Option<u32>,
		limit: Option<u32>,
		at: Option<<Block as BlockT>::Hash>,
	) -> RpcResult<Vec<CharTrait>> {
		let api = self.client.runtime_api();
		let at = at.unwrap_or_else(|| self.client.info().best_hash);

		let char_traits = api
			.get_char_traits(at, from_index, limit)
			.map_err(|e| map_err(e, "Failed to get char traits"))?;

		Ok(char_traits)
	}
}
