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
}
