use crate::*;

impl pallet_balances::Config for Runtime {
	/// The balance of an account.
	type Balance = Balance;
	/// Handler for the unbalanced reduction when removing a dust account.
	type DustRemoval = ();
	/// The overarching event type.
	type RuntimeEvent = RuntimeEvent;
	/// The minimum amount required to keep an account open.
	type ExistentialDeposit = ConstU128<EXISTENTIAL_DEPOSIT>;
	/// The means of storing the balances of an account.
	type AccountStore = System;
	/// Weight information for extrinsics in this pallet.
	type WeightInfo = pallet_balances::weights::SubstrateWeight<Runtime>;
	/// The maximum number of locks that should exist on an account.
	/// Not strictly enforced, but used for weight estimation.
	type MaxLocks = ConstU32<50>;
	/// The maximum number of named reserves that can exist on an account.
	type MaxReserves = ();
	/// The id type for named reserves.
	type ReserveIdentifier = [u8; 8];
}
