pub mod client;
mod error;

use jsonrpsee::{core::RpcResult, proc_macros::rpc};
use sp_rpc::{BlockchainStats, CharTrait, GenesisData};
use sp_runtime::traits::Block as BlockT;

#[rpc(client, server)]
pub trait ChainDataProviderApi<Block: BlockT, AccountId> {
	/// RPC method provides information about current blockchain state
	#[method(name = "chain_getBlockchainData")]
	fn get_blockchain_data(&self) -> RpcResult<BlockchainStats>;

	/// RPC method provides information about blockchain genesis config
	#[method(name = "chain_getGenesisData")]
	fn get_genesis_data(&self) -> RpcResult<GenesisData<AccountId>>;

	/// RPC method provides chain network id ("testnet", "mainnet", etc.)
	#[method(name = "chain_getNetworkId")]
	fn get_network_id(&self) -> RpcResult<String>;

	/// RPC method provide current list of char traits
	#[method(name = "chain_getCharTraits")]
	fn get_char_traits(
		&self,
		from_index: Option<u32>,
		limit: Option<u32>,
		at: Option<Block::Hash>,
	) -> RpcResult<Vec<CharTrait>>;
}
