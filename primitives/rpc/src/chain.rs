use codec::{Decode, Encode};
use scale_info::{
	prelude::{string::String, vec::Vec},
	TypeInfo,
};
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

#[derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
pub struct CharTrait {
	pub id: u32,
	pub name: String,
	pub emoji: String,
}

#[derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
pub struct PhoneVerifier<AccountId> {
	pub account_id: AccountId,
	pub name: String,
}

#[derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
pub struct BlockchainStats {
	/// Last block production time
	pub last_block_time: u64,
	/// Current block height
	pub tip_height: u64,
	/// Total number of executed transactions
	pub transaction_count: u64,
	/// Total number of payment transactions
	pub payment_transaction_count: u64,
	/// Total number of payment transactions with an appreciation
	pub appreciations_transactions_count: u64,
	/// Total number of payment transactions
	pub update_user_transactions_count: u64,
	/// Total number of verified user accounts
	pub users_count: u64,
	/// Total tx fee subsidies issued by the protocol
	pub fee_subs_amount: u128,
	pub signup_rewards_amount: u128,
	pub referral_rewards_amount: u128,
	pub validator_rewards_amount: u128,
	/// Amount of rewards paid to causes
	pub causes_rewards_amount: u128,
}

#[derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
pub struct GenesisData<AccountId> {
	pub net_id: u32,
	pub net_name: Vec<u8>,
	pub genesis_time: u64,
	pub signup_reward_phase1_alloc: u128,
	pub signup_reward_phase2_alloc: u128,

	pub signup_reward_phase1_amount: u128,
	pub signup_reward_phase2_amount: u128,
	pub signup_reward_phase3_start: u128,

	pub referral_reward_phase1_alloc: u128,
	pub referral_reward_phase2_alloc: u128,

	pub referral_reward_phase1_amount: u128,
	pub referral_reward_phase2_amount: u128,

	pub tx_fee_subsidy_max_per_user: u64,
	pub tx_fee_subsidies_alloc: u128,
	pub tx_fee_subsidy_max_amount: u128,

	pub block_reward_amount: u64,
	pub block_reward_last_block: u64,

	pub karma_reward_amount: u128,
	pub karma_reward_alloc: u128,
	pub karma_reward_top_n_users: u64,

	// pub treasury_premint_amount: u64,
	// pub treasury_account_id: AccountId,
	// pub treasury_account_name: Vec<u8>,
	pub char_traits: Vec<CharTrait>,
	pub verifiers: Vec<PhoneVerifier<AccountId>>,
}
