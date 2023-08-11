use codec::Codec;
use pallet_nomination_pools::PoolId;
use sp_rpc::{BondedPool, NominationPoolsConfiguration};
use sp_std::vec::Vec;

sp_api::decl_runtime_apis! {
	/// Runtime api for accessing information about nomination pools.
	pub trait NominationPoolsApi<AccountId, Balance, BlockNumber>
		where
			AccountId: Codec,
			Balance: Codec,
			BlockNumber: Codec,
	{
		/// Returns the pending rewards for the member that the AccountId was given for.
		fn pending_rewards(who: AccountId) -> Option<Balance>;

		/// Returns the equivalent balance of `points` for a given pool.
		fn points_to_balance(pool_id: PoolId, points: Balance) -> Balance;

		/// Returns the equivalent points of `new_funds` for a given pool.
		fn balance_to_points(pool_id: PoolId, new_funds: Balance) -> Balance;

		/// Returns the information about nomination pools
		fn get_pools(from_index: Option<u32>, limit: Option<u32>) -> Vec<BondedPool<AccountId, Balance, BlockNumber>>;

		/// Return the information about pallet nomination-pools configuration
		fn get_configuration() -> NominationPoolsConfiguration<Balance>;

		/// Returns the pool id of the pool that the account is a member of
		fn member_of(account_id: AccountId) -> Option<PoolId>;
	}
}
