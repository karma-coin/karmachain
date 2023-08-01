use crate::*;
use frame_support::PalletId;
use sp_runtime::FixedU128;

parameter_types! {
	pub const PoolsPalletId: PalletId = PalletId(*b"py/nopls");
	pub const MaxPointsToBalance: u8 = 10;
}

impl pallet_nomination_pools::Config for Runtime {
	/// The overarching event type.
	type RuntimeEvent = RuntimeEvent;
	/// Weight information for extrinsics in this pallet.
	type WeightInfo = pallet_nomination_pools::weights::SubstrateWeight<Runtime>;
	/// The nominating balance.
	type Currency = Balances;
	/// The type that is used for reward counter.
	///
	/// The arithmetic of the reward counter might saturate based on the size of the
	/// `Currency::Balance`. If this happens, operations fails. Nonetheless, this type should be
	/// chosen such that this failure almost never happens, as if it happens, the pool basically
	/// needs to be dismantled (or all pools migrated to a larger `RewardCounter` type, which is
	/// a PITA to do).
	///
	/// See the inline code docs of `Member::pending_rewards` and `RewardPool::update_recorded`
	/// for example analysis. A [`sp_runtime::FixedU128`] should be fine for chains with balance
	/// types similar to that of Polkadot and Kusama, in the absence of severe slashing (or
	/// prevented via a reasonable `MaxPointsToBalance`), for many many years to come.
	type RewardCounter = FixedU128;
	/// The nomination pool's pallet id.
	type PalletId = PoolsPalletId;
	/// The maximum pool points-to-balance ratio that an `open` pool can have.
	///
	/// This is important in the event slashing takes place and the pool's points-to-balance
	/// ratio becomes disproportional.
	///
	/// Moreover, this relates to the `RewardCounter` type as well, as the arithmetic operations
	/// are a function of number of points, and by setting this value to e.g. 10, you ensure
	/// that the total number of points in the system are at most 10 times the total_issuance of
	/// the chain, in the absolute worse case.
	///
	/// For a value of 10, the threshold would be a pool points-to-balance ratio of 10:1.
	/// Such a scenario would also be the equivalent of the pool being 90% slashed.
	type MaxPointsToBalance = MaxPointsToBalance;
	/// Infallible method for converting `Currency::Balance` to `U256`.
	type BalanceToU256 = BalanceToU256;
	/// Infallible method for converting `U256` to `Currency::Balance`.
	type U256ToBalance = U256ToBalance;
	/// The interface for nominating.
	type Staking = Staking;
	/// The amount of eras a `SubPools::with_era` pool can exist before it gets merged into the
	/// `SubPools::no_era` pool. In other words, this is the amount of eras a member will be
	/// able to withdraw from an unbonding pool which is guaranteed to have the correct ratio of
	/// points to balance; once the `with_era` pool is merged into the `no_era` pool, the ratio
	/// can become skewed due to some slashed ratio getting merged in at some point.
	type PostUnbondingPoolsWindow = ConstU32<4>;
	/// The maximum length, in bytes, that a pools metadata maybe.
	type MaxMetadataLen = ConstU32<256>;
	/// The maximum number of simultaneous unbonding chunks that can exist per member.
	// we use the same number of allowed unlocking chunks as with staking.
	type MaxUnbonding = <Self as pallet_staking::Config>::MaxUnlockingChunks;
}
