use super::{error::map_err, StakingApiServer};
use codec::Codec;
use jsonrpsee::core::RpcResult;
use runtime_api::staking::StakingApi;
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_rpc::{Nominations, ValidatorPrefs};
use sp_runtime::traits::Block as BlockT;
use std::sync::Arc;

pub struct Staking<C, P> {
	/// Shared reference to the client.
	client: Arc<C>,
	_marker: std::marker::PhantomData<P>,
}

impl<C, P> Staking<C, P> {
	pub fn new(client: Arc<C>) -> Self {
		Self { client, _marker: Default::default() }
	}
}

impl<C, Block, AccountId> StakingApiServer<Block::Hash, AccountId> for Staking<C, Block>
where
	Block: BlockT,
	AccountId: Codec,
	C: ProvideRuntimeApi<Block> + HeaderBackend<Block> + Send + Sync + 'static,
	C::Api: StakingApi<Block, AccountId>,
{
	fn get_validators(
		&self,
		at: Option<<Block as BlockT>::Hash>,
	) -> RpcResult<Vec<ValidatorPrefs<AccountId>>> {
		let api = self.client.runtime_api();
		let at = at.unwrap_or_else(|| self.client.info().best_hash);

		Ok(api.get_validators(at).map_err(|e| map_err(e, "Failed to query Runtime API"))?)
	}

	fn get_nominations(
		&self,
		account_id: AccountId,
		at: Option<<Block as BlockT>::Hash>,
	) -> RpcResult<Option<Nominations<AccountId>>> {
		let api = self.client.runtime_api();
		let at = at.unwrap_or_else(|| self.client.info().best_hash);

		Ok(api
			.get_nominations(at, account_id)
			.map_err(|e| map_err(e, "Failed to query Runtime API"))?)
	}
}
