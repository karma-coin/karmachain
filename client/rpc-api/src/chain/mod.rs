pub mod client;
mod error;

use jsonrpsee::{core::RpcResult, proc_macros::rpc};
use sp_rpc::{BlockchainStats, GenesisData};
use sp_runtime::traits::Block as BlockT;

#[rpc(client, server)]
pub trait BlocksProviderApi<Block: BlockT, AccountId> {
	/// RPC method provides information about current blockchain state
	#[method(name = "chain_getBlockchainData")]
	fn get_blockchain_data(&self) -> RpcResult<BlockchainStats>;

	/// RPC method provides information about blockchain genesis config
	#[method(name = "chain_getGenesisData")]
	fn get_genesis_data(&self) -> RpcResult<GenesisData<AccountId>>;
}
