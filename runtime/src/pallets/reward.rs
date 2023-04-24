use crate::*;

impl pallet_reward::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type Currency = Balances;
	type IdentityProvider = Identity;
}
