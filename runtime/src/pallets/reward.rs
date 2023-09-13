use crate::*;
use frame_support::PalletId;
use pallet_babe::RandomnessFromOneEpochAgo;
use pallet_reward::crypto::AuthorityId;

parameter_types! {
	pub const RewardPalletId: PalletId = PalletId(*b"kr/rewar");
	pub const MaxGenerateRandom: u32 = 10;
	pub const MaxWinners: u32 = 1000;
	pub const MaxOffchainAccounts: u32 = 10;
}

impl pallet_reward::Config for Runtime {
	/// The Reward's pallet id
	type PalletId = RewardPalletId;
	type RuntimeEvent = RuntimeEvent;
	/// Something that provides trait score in the runtime
	type ScoreProvider = Appreciation;
	/// Number of time we should try to generate a random number that has no modulo bias.
	/// The larger this number, the more potential computation is used for picking the winner,
	/// but also the more likely that the chosen winner is done fairly.
	type MaxGenerateRandom = MaxGenerateRandom;
	/// Something that provides randomness in the runtime.
	type Randomness = RandomnessFromOneEpochAgo<Runtime>;
	/// Maximum number of winners in karma rewards per one round
	type MaxWinners = MaxWinners;
	/// Maximum number of offchain account that can sign `submit_karma_rewards` tx
	type MaxOffchainAccounts = MaxOffchainAccounts;
	type AuthorityId = AuthorityId;
}
