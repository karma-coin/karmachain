pub mod client;
mod error;

use jsonrpsee::{core::RpcResult, proc_macros::rpc};
use sp_runtime::traits::{Block as BlockT, NumberFor};

#[rpc(client, server)]
pub trait EventsProviderApi<Block: BlockT, Event> {
	/// RPC method provides events for specific blocks
	#[method(name = "get_blockchain_events", blocking)]
	fn get_blockchain_events(
		&self,
		from_block_height: u32,
		to_block_height: u32,
	) -> RpcResult<Vec<Event>>;
}
