use crate::events::{error::map_err, EventsProviderApiServer};
use codec::Codec;
use jsonrpsee::core::RpcResult;
use runtime_api::events::EventProvider;
use sp_api::{BlockT, ProvideRuntimeApi};
use sp_blockchain::HeaderBackend;
use sp_runtime::{generic::BlockId, traits::NumberFor};
use std::sync::Arc;

pub struct EventsProvider<C, P> {
	/// Shared reference to the client.
	client: Arc<C>,
	_marker: std::marker::PhantomData<P>,
}

impl<C, P> EventsProvider<C, P> {
	pub fn new(client: Arc<C>) -> Self {
		Self { client, _marker: Default::default() }
	}
}

impl<C, Block, Event> EventsProviderApiServer<Block, Event> for EventsProvider<C, Block>
where
	Block: BlockT,
	Event: Codec,
	C: ProvideRuntimeApi<Block> + HeaderBackend<Block> + Send + Sync + 'static,
	C::Api: EventProvider<Block, Event>,
{
	fn get_blockchain_events(
		&self,
		from_block_height: u32,
		to_block_height: u32,
	) -> RpcResult<Vec<Event>> {
		let api = self.client.runtime_api();

		let events = (from_block_height..=to_block_height)
			.map(|block_number| {
				api.get_block_events(&BlockId::Number(block_number.into()))
					.map_err(|e| map_err(e, "Fail to get block events"))
			})
			.collect::<Result<Vec<_>, _>>()?
			.into_iter()
			.flatten()
			.collect();

		Ok(events)
	}
}
