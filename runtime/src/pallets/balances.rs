use crate::*;

impl pallet_balances::Config for Runtime {
	/// The overarching event type.
	type RuntimeEvent = RuntimeEvent;
	/// Weight information for extrinsics in this pallet.
	type WeightInfo = pallet_balances::weights::SubstrateWeight<Runtime>;
	/// The balance of an account.
	type Balance = Balance;
	/// Handler for the unbalanced reduction when removing a dust account.
	type DustRemoval = ();
	/// The minimum amount required to keep an account open.
	type ExistentialDeposit = ConstU128<EXISTENTIAL_DEPOSIT>;
	/// The means of storing the balances of an account.
	type AccountStore = System;
	/// The ID type for reserves.
	///
	/// Use of reserves is deprecated in favour of holds. See `https://github.com/paritytech/substrate/pull/12951/`
	type ReserveIdentifier = [u8; 8];
	/// The ID type for holds.
	type HoldIdentifier = RuntimeHoldReason;
	/// The ID type for freezes.
	type FreezeIdentifier = ();
	/// The maximum number of locks that should exist on an account.
	/// Not strictly enforced, but used for weight estimation.
	type MaxLocks = ConstU32<50>;
	/// The maximum number of named reserves that can exist on an account.
	type MaxReserves = ();
	// The maximum number of holds that can exist on an account at any time.
	type MaxHolds = ConstU32<1>;
	/// The maximum number of individual freeze locks that can exist on an account at any time.
	type MaxFreezes = ();
}
