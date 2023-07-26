use codec::{Decode, Encode, MaxEncodedLen};
use scale_info::TypeInfo;

#[derive(Default, Decode, Encode, MaxEncodedLen, TypeInfo)]
pub struct AccountRewardsData {
	/// true - means account got his signup reward
	pub signup_reward: bool,
	/// true - means account got his reward for referral new user
	pub referral_reward: bool,
	/// true - means account got his karma reward
	pub karma_reward: bool,
	/// Number of transaction that was subsidized for this user
	pub transaction_subsidized: u8,
	/// Number of appreciation transaction, need to participate in karma rewards
	pub appreciation_count: u32,
}

#[derive(Debug, Clone, Eq, PartialEq, Decode, Encode, MaxEncodedLen, TypeInfo)]
pub enum RewardType {
	Signup = 0_isize,
	Referral,
	Karma,
	Subsidy,
}
