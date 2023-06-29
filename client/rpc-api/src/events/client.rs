use crate::events::{
	error::{map_err, Error},
	EventsProviderApiServer,
};
use codec::Codec;
use jsonrpsee::{
	core::RpcResult,
	types::{error::CallError, ErrorObject},
};
use runtime_api::events::EventProvider;
use sc_client_api::BlockBackend;
use sp_api::{BlockT, ProvideRuntimeApi};
use sp_blockchain::HeaderBackend;
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
	C: ProvideRuntimeApi<Block>
		+ BlockBackend<Block>
		+ HeaderBackend<Block>
		+ Send
		+ Sync
		+ 'static,
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
				self.client
					.block_hash(block_number.into())
					.map_err(|e| map_err(e, "Failed to get block hashes"))
					.map(|option| {
						option.ok_or(CallError::Custom(ErrorObject::owned(
							Error::BlockNotFound.into(),
							"Block hash not found",
							Option::<()>::None,
						)))
					})
			})
			.collect::<Result<Result<Vec<_>, _>, _>>()??
			.into_iter()
			.map(|block_hash| {
				api.get_block_events(block_hash)
					.map_err(|e| map_err(e, "Fail to get block events"))
			})
			.collect::<Result<Vec<_>, _>>()?
			.into_iter()
			.flatten()
			.collect();

		Ok(events)
	}
}
