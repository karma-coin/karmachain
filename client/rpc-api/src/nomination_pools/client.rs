use super::{error::map_err, NominationPoolsApiServer};
use codec::Codec;
use jsonrpsee::core::RpcResult;
use runtime_api::nomination_pools::NominationPoolsApi;
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_rpc::{BondedPool, NominationPoolsConfiguration, PoolId, PoolMember};
use sp_runtime::traits::Block as BlockT;
use std::sync::Arc;

pub struct NominationPools<C, P> {
	/// Shared reference to the client.
	client: Arc<C>,
	_marker: std::marker::PhantomData<P>,
}

impl<C, P> NominationPools<C, P> {
	pub fn new(client: Arc<C>) -> Self {
		Self { client, _marker: Default::default() }
	}
}

impl<C, Block, AccountId, Balance, BlockNumber>
	NominationPoolsApiServer<Block::Hash, AccountId, Balance, BlockNumber> for NominationPools<C, Block>
where
	Block: BlockT,
	AccountId: Codec,
	Balance: Codec,
	BlockNumber: Codec,
	C: ProvideRuntimeApi<Block> + HeaderBackend<Block> + Send + Sync + 'static,
	C::Api: NominationPoolsApi<Block, AccountId, Balance, BlockNumber>,
{
	fn pending_rewards(
		&self,
		who: AccountId,
		at: Option<<Block as BlockT>::Hash>,
	) -> RpcResult<Option<Balance>> {
		let api = self.client.runtime_api();
		let at = at.unwrap_or_else(|| self.client.info().best_hash);

		Ok(api
			.pending_rewards(at, who)
			.map_err(|e| map_err(e, "Failed to query Runtime API"))?)
	}

	fn points_to_balance(
		&self,
		pool_id: PoolId,
		points: Balance,
		at: Option<<Block as BlockT>::Hash>,
	) -> RpcResult<Balance> {
		let api = self.client.runtime_api();
		let at = at.unwrap_or_else(|| self.client.info().best_hash);

		Ok(api
			.points_to_balance(at, pool_id, points)
			.map_err(|e| map_err(e, "Failed to query Runtime API"))?)
	}

	fn balance_to_points(
		&self,
		pool_id: PoolId,
		new_funds: Balance,
		at: Option<<Block as BlockT>::Hash>,
	) -> RpcResult<Balance> {
		let api = self.client.runtime_api();
		let at = at.unwrap_or_else(|| self.client.info().best_hash);

		Ok(api
			.balance_to_points(at, pool_id, new_funds)
			.map_err(|e| map_err(e, "Failed to query Runtime API"))?)
	}

	fn get_pools(
		&self,
		from_index: Option<u32>,
		limit: Option<u32>,
		at: Option<<Block as BlockT>::Hash>,
	) -> RpcResult<Vec<BondedPool<AccountId, Balance, BlockNumber>>> {
		let api = self.client.runtime_api();
		let at = at.unwrap_or_else(|| self.client.info().best_hash);

		Ok(api
			.get_pools(at, from_index, limit)
			.map_err(|e| map_err(e, "Failed to query Runtime API"))?)
	}

	fn get_configuration(
		&self,
		at: Option<<Block as BlockT>::Hash>,
	) -> RpcResult<NominationPoolsConfiguration<Balance>> {
		let api = self.client.runtime_api();
		let at = at.unwrap_or_else(|| self.client.info().best_hash);

		Ok(api
			.get_configuration(at)
			.map_err(|e| map_err(e, "Failed to query Runtime API"))?)
	}

	fn member_of(
		&self,
		account_id: AccountId,
		at: Option<<Block as BlockT>::Hash>,
	) -> RpcResult<Option<PoolMember>> {
		let api = self.client.runtime_api();
		let at = at.unwrap_or_else(|| self.client.info().best_hash);

		Ok(api
			.member_of(at, account_id)
			.map_err(|e| map_err(e, "Failed to query Runtime API"))?)
	}
}
