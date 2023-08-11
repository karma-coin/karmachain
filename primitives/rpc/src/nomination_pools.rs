use codec::{Decode, Encode};
use pallet_nomination_pools::BalanceOf;
use scale_info::TypeInfo;
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use sp_runtime::Perbill;

pub type PoolId = u32;

/// A member in a pool.
#[derive(Encode, Decode, TypeInfo, PartialEq, Clone, Copy)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
pub struct PoolMember<Balance> {
	/// The identifier of the pool to which `who` belongs.
	pub pool_id: PoolId,
	/// The quantity of points this member has in the bonded pool or in a sub pool if
	/// `Self::unbonding_era` is some.
	pub points: Balance,
}

impl<T: pallet_nomination_pools::Config> From<pallet_nomination_pools::PoolMember<T>>
	for PoolMember<BalanceOf<T>>
{
	fn from(pool_member: pallet_nomination_pools::PoolMember<T>) -> Self {
		Self { pool_id: pool_member.pool_id, points: pool_member.points }
	}
}

/// A pool's possible states.
#[derive(Encode, Decode, TypeInfo, PartialEq, Clone, Copy)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
pub enum PoolState {
	/// The pool is open to be joined, and is working normally.
	Open,
	/// The pool is blocked. No one else can join.
	Blocked,
	/// The pool is in the process of being destroyed.
	///
	/// All members can now be permissionlessly unbonded, and the pool can never go back to any
	/// other state other than being dissolved.
	Destroying,
}

impl From<pallet_nomination_pools::PoolState> for PoolState {
	fn from(pool_state: pallet_nomination_pools::PoolState) -> Self {
		match pool_state {
			pallet_nomination_pools::PoolState::Open => Self::Open,
			pallet_nomination_pools::PoolState::Blocked => Self::Blocked,
			pallet_nomination_pools::PoolState::Destroying => Self::Destroying,
		}
	}
}

/// Pool administration roles.
///
/// Any pool has a depositor, which can never change. But, all the other roles are optional, and
/// cannot exist. Note that if `root` is set to `None`, it basically means that the roles of this
/// pool can never change again (except via governance).
#[derive(Encode, Decode, TypeInfo, PartialEq, Clone)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
pub struct PoolRoles<AccountId> {
	/// Creates the pool and is the initial member. They can only leave the pool once all other
	/// members have left. Once they fully leave, the pool is destroyed.
	pub depositor: AccountId,
	/// Can change the nominator, bouncer, or itself and can perform any of the actions the
	/// nominator or bouncer can.
	pub root: Option<AccountId>,
	/// Can select which validators the pool nominates.
	pub nominator: Option<AccountId>,
	/// Can change the pools state and kick members if the pool is blocked.
	pub bouncer: Option<AccountId>,
}

impl<AccountId> From<pallet_nomination_pools::PoolRoles<AccountId>> for PoolRoles<AccountId> {
	fn from(pool_roles: pallet_nomination_pools::PoolRoles<AccountId>) -> Self {
		Self {
			depositor: pool_roles.depositor,
			root: pool_roles.root,
			nominator: pool_roles.nominator,
			bouncer: pool_roles.bouncer,
		}
	}
}

/// Pool commission change rate preferences.
///
/// The pool root is able to set a commission change rate for their pool. A commission change rate
/// consists of 2 values; (1) the maximum allowed commission change, and (2) the minimum amount of
/// blocks that must elapse before commission updates are allowed again.
///
/// Commission change rates are not applied to decreases in commission.
#[derive(Encode, Decode, TypeInfo, PartialEq, Copy, Clone)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
pub struct CommissionChangeRate<BlockNumber> {
	/// The maximum amount the commission can be updated by per `min_delay` period.
	pub max_increase: Perbill,
	/// How often an update can take place.
	pub min_delay: BlockNumber,
}

impl<BlockNumber> From<pallet_nomination_pools::CommissionChangeRate<BlockNumber>>
	for CommissionChangeRate<BlockNumber>
{
	fn from(
		commission_change_rate: pallet_nomination_pools::CommissionChangeRate<BlockNumber>,
	) -> Self {
		Self {
			max_increase: commission_change_rate.max_increase,
			min_delay: commission_change_rate.min_delay,
		}
	}
}

/// Pool commission.
///
/// The pool `root` can set commission configuration after pool creation. By default, all commission
/// values are `None`. Pool `root` can also set `max` and `change_rate` configurations before
/// setting an initial `current` commission.
///
/// `current` is a tuple of the commission percentage and payee of commission. `throttle_from`
/// keeps track of which block `current` was last updated. A `max` commission value can only be
/// decreased after the initial value is set, to prevent commission from repeatedly increasing.
///
/// An optional commission `change_rate` allows the pool to set strict limits to how much commission
/// can change in each update, and how often updates can take place.
#[derive(Encode, Decode, Default, TypeInfo, PartialEq, Copy, Clone)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
pub struct Commission<AccountId, BlockNumber> {
	/// Optional the account commission is paid to.
	pub beneficiary: Option<AccountId>,
	/// Optional commission rate of the pool.
	pub current: Option<Perbill>,
	/// Optional maximum commission that can be set by the pool `root`. Once set, this value can
	/// only be updated to a decreased value.
	pub max: Option<Perbill>,
	/// Optional configuration around how often commission can be updated, and when the last
	/// commission update took place.
	pub change_rate: Option<CommissionChangeRate<BlockNumber>>,
	/// The block from where throttling should be checked from. This value will be updated on all
	/// commission updates and when setting an initial `change_rate`.
	pub throttle_from: Option<BlockNumber>,
}

impl<T: pallet_nomination_pools::Config> From<pallet_nomination_pools::Commission<T>>
	for Commission<T::AccountId, T::BlockNumber>
{
	fn from(commission: pallet_nomination_pools::Commission<T>) -> Self {
		let (current, beneficiary) = commission.current.unzip();
		Self {
			beneficiary,
			current,
			max: commission.max,
			change_rate: commission.change_rate.map(Into::into),
			throttle_from: commission.throttle_from,
		}
	}
}

/// Pool permissions and state
#[derive(Encode, Decode, TypeInfo, PartialEq, Clone)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
pub struct BondedPool<AccountId, Balance, BlockNumber> {
	/// The identifier of the pool.
	pub id: PoolId,
	/// The commission rate of the pool.
	pub commission: Commission<AccountId, BlockNumber>,
	/// Count of members that belong to the pool.
	pub member_counter: u32,
	/// Total points of all the members in the pool who are actively bonded.
	pub points: Balance,
	/// See [`PoolRoles`].
	pub roles: PoolRoles<AccountId>,
	/// The current state of the pool.
	pub state: PoolState,
}

impl<T: pallet_nomination_pools::Config> From<(u32, pallet_nomination_pools::BondedPoolInner<T>)>
	for BondedPool<T::AccountId, BalanceOf<T>, T::BlockNumber>
{
	fn from((id, pool): (u32, pallet_nomination_pools::BondedPoolInner<T>)) -> Self {
		Self {
			id,
			commission: pool.commission.into(),
			member_counter: pool.member_counter,
			points: pool.points,
			roles: pool.roles.into(),
			state: pool.state.into(),
		}
	}
}

/// Current configuration of pallet nomination-pools
#[derive(Encode, Decode, TypeInfo, PartialEq, Clone)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
pub struct NominationPoolsConfiguration<Balance> {
	pub min_join_bond: Balance,
	pub min_create_bond: Balance,
	pub max_pools: Option<u32>,
	pub max_members_per_pool: Option<u32>,
	pub max_members: Option<u32>,
	pub global_max_commission: Option<Perbill>,
}
