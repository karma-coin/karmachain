pub mod client;
mod error;

use jsonrpsee::{core::RpcResult, proc_macros::rpc};
use sp_rpc::{Block as RpcBlock, BlockchainStats, GenesisData};
use sp_runtime::traits::Block as BlockT;

#[rpc(client, server)]
pub trait BlocksProviderApi<Block: BlockT, AccountId> {
	/// RPC method provides block information by block number
	#[method(name = "chain_get_block_info", blocking)]
	fn get_block_info(&self, block_height: u32) -> RpcResult<RpcBlock<AccountId, Block::Hash>>;

	/// RPC method provides blocks information by the range of blocks number
	#[method(name = "chain_get_blocks", blocking)]
	fn get_blocks(
		&self,
		from_block_height: u32,
		to_block_height: u32,
	) -> RpcResult<Vec<RpcBlock<AccountId, Block::Hash>>> {
		(from_block_height..=to_block_height)
			.map(|block_number| self.get_block_info(block_number))
			.collect()
	}

	/// RPC method provides information about current blockchain state
	#[method(name = "chain_get_blockchain_data")]
	fn get_blockchain_data(&self) -> RpcResult<BlockchainStats>;

	/// RPC method provides information about blockchain genesis config
	#[method(name = "chain_get_genesis_data")]
	fn get_genesis_data(&self) -> RpcResult<GenesisData<AccountId>>;
}
