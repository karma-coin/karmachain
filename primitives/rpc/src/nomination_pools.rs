use codec::{Decode, Encode};
use scale_info::TypeInfo;
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use sp_runtime::Perbill;

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
	/// Optional commission rate of the pool along with the account commission is paid to.
	pub current: Option<(Perbill, AccountId)>,
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

/// Pool permissions and state
#[derive(Encode, Decode, TypeInfo, PartialEq, Clone)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
pub struct BondedPoolInner<AccountId, Balance, BlockNumber> {
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
