#[derive(Default)]
pub struct AccountRewardsData {
	/// true - means account got his signup reward
	pub signup_reward: bool,
	/// true - means account got his reward for refferal new user
	pub referral_reward: bool,
	/// true - means accoutn got his karma reward
	pub karma_reward: bool,
	/// Number of transaction that was subsidized for this user
	pub transaction_subsidized: u8,
}
