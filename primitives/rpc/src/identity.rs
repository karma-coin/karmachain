use codec::{Decode, Encode};
use scale_info::{
	prelude::{string::String, vec::Vec},
	TypeInfo,
};
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

pub type PhoneNumberHash = sp_core::H512;

#[derive(Encode, Decode, TypeInfo, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
pub struct TraitScore {
	pub trait_id: u32,
	pub karma_score: u32,
	pub community_id: u32,
}

#[derive(Encode, Decode, TypeInfo, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
pub struct CommunityMembership {
	pub community_id: u32,
	pub karma_score: u32,
	pub is_admin: bool,
}

#[derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
pub struct UserInfo<AccountId> {
	pub account_id: AccountId,
	pub nonce: u64,
	pub user_name: String,
	pub phone_number_hash: PhoneNumberHash,
	pub balance: u64,
	pub trait_scores: Vec<TraitScore>,
	// pre_keys
	pub karma_score: u32,
	pub community_membership: Vec<CommunityMembership>,
	pub metadata: Option<Vec<u8>>,
}

#[derive(Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
pub struct Contact<AccountId> {
	pub user_name: String,
	pub account_id: AccountId,
	pub phone_number_hash: PhoneNumberHash,
	pub community_membership: Vec<CommunityMembership>,
	pub trait_scores: Vec<TraitScore>,
	pub metadata: Option<Vec<u8>>,
}
