pub mod client;
pub mod error;

use jsonrpsee::{core::RpcResult, proc_macros::rpc};
use sp_rpc::{BondedPool, NominationPoolsConfiguration, PoolId, PoolMember};

#[rpc(client, server)]
pub trait NominationPoolsApi<BlockHash, AccountId, Balance, BlockNumber> {
	#[method(name = "nominationPools_pendingRewards")]
	fn pending_rewards(&self, who: AccountId, at: Option<BlockHash>) -> RpcResult<Option<Balance>>;

	#[method(name = "nominationPools_pointsToBalance")]
	fn points_to_balance(
		&self,
		pool_id: PoolId,
		points: Balance,
		at: Option<BlockHash>,
	) -> RpcResult<Balance>;

	#[method(name = "nominationPools_balanceToPoints")]
	fn balance_to_points(
		&self,
		pool_id: PoolId,
		new_funds: Balance,
		at: Option<BlockHash>,
	) -> RpcResult<Balance>;

	#[method(name = "nominationPools_getPools")]
	fn get_pools(
		&self,
		from_index: Option<u32>,
		limit: Option<u32>,
		at: Option<BlockHash>,
	) -> RpcResult<Vec<BondedPool<AccountId, Balance, BlockNumber>>>;

	#[method(name = "nominationPools_getConfiguration")]
	fn get_configuration(
		&self,
		at: Option<BlockHash>,
	) -> RpcResult<NominationPoolsConfiguration<Balance>>;

	#[method(name = "nominationPools_memberOf")]
	fn member_of(
		&self,
		account_id: AccountId,
		at: Option<BlockHash>,
	) -> RpcResult<Option<PoolMember<Balance>>>;

	#[method(name = "nominationPools_getPoolMembers")]
	fn get_pool_members(
		&self,
		pool_id: PoolId,
		from_index: Option<u32>,
		limit: Option<u32>,
		at: Option<BlockHash>,
	) -> RpcResult<Vec<AccountId>>;
}
