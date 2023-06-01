use crate::*;
use frame_support::PalletId;
use pallet_babe::RandomnessFromOneEpochAgo;

parameter_types! {
	pub const MaxGenerateRandom: u32 = 10;
	pub const RewardPalletId: PalletId = PalletId(*b"kr/rewar");
}

impl pallet_reward::Config for Runtime {
	type PalletId = RewardPalletId;
	type RuntimeEvent = RuntimeEvent;
	type ScoreProvider = Appreciation;
	type MaxGenerateRandom = MaxGenerateRandom;
	type Randomness = RandomnessFromOneEpochAgo<Runtime>;
}
