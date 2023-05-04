use codec::{Decode, Encode};
use scale_info::prelude::vec::Vec;
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

#[derive(Encode, Decode)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
pub struct UserInfo<AccountId> {
	pub account_id: AccountId,
	pub nonce: u64,
	pub user_name: Vec<u8>,
	pub mobile_number: Vec<u8>,
	pub balance: u64,
	pub trait_scores: Vec<TraitScore>,
	// pre_keys
	pub karma_score: u32,
	pub community_membership: Vec<CommunityMembership>,
}

#[derive(Encode, Decode)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
pub struct TraitScore {
	pub trait_id: u32,
	pub karma_score: u32,
	pub community_id: u32,
}

#[derive(Encode, Decode)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
pub struct CommunityMembership {
	pub community_id: u32,
	pub karma_score: u32,
	pub is_admin: bool,
}

#[derive(Encode, Decode)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
pub struct SignedTransaction<AccountId, Signature> {
	pub signer: Option<AccountId>,
	pub transaction_body: Vec<u8>,
	pub signature: Option<Signature>,
}

#[derive(Encode, Decode)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
pub struct SignedTransactionWithStatus<AccountId, Signature> {
	pub signed_transaction: SignedTransaction<AccountId, Signature>,
	pub status: TransactionStatus,
	pub from: Option<UserInfo<AccountId>>,
	pub to: Option<UserInfo<AccountId>>,
}

#[derive(Encode, Decode, Default)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
pub enum TransactionStatus {
	#[default]
	Unknown = 0_isize,
	NotSubmitted,
	Submitted,
	Rejected,
	OnChain,
}

#[derive(Encode, Decode, Default)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
pub struct Block<AccountId, Hash> {
	pub time: u64,
	pub author: Option<AccountId>,
	pub height: u32,
	pub transaction_hashes: Vec<Hash>,
	pub fees: u128,
	pub signature: Vec<u8>,
	// pub reward: u128,
	// pub minted: u128,
	pub digest: Vec<u8>,
}

#[derive(Encode, Decode)]
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
	/// Total tx fees collected by block producers
	pub fees_amount: u128,
	/// Total number of kCents minted by the protocol since genesis
	pub minted_amount: u128,
	/// Total number of kCents in circulation by minting. Not including pre-mint
	pub circulation: u128,
	/// Total tx fee subsidies issued by the protocol
	pub fee_subs_count: u64,
	pub fee_subs_amount: u128,
	pub signup_rewards_count: u64,
	pub signup_rewards_amount: u128,
	pub referral_rewards_count: u64,
	pub referral_rewards_amount: u128,
	pub validator_rewards_count: u64,
	pub validator_rewards_amount: u128,
	/// Amount of rewards paid to causes
	pub causes_rewards_amount: u128,
	// TODO: how can know KC cost?
	// Estimated KC to USD exchange rate
	// double exchange_rate = 19;
}

#[derive(Encode, Decode)]
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

#[derive(Encode, Decode)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
pub struct CharTrait {
	pub id: u32,
	pub name: Vec<u8>,
	pub emoji: Vec<u8>,
}

#[derive(Encode, Decode)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
pub struct PhoneVerifier<AccountId> {
	pub account_id: AccountId,
	pub name: Vec<u8>,
}

#[derive(Encode, Decode)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
pub struct Contact<AccountId> {
	pub user_name: Vec<u8>,
	pub account_id: AccountId,
	pub mobile_number: Vec<u8>,
	pub community_membership: Vec<CommunityMembership>,
	pub trait_scores: Vec<TraitScore>,
}

#[derive(Encode, Decode)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
pub struct LeaderboardEntry<AccountId> {
	pub user_name: Vec<u8>,
	pub account_id: AccountId,
	pub score: u32,
	char_traits_ids: u32,
}
