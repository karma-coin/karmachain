#![cfg_attr(not(feature = "std"), no_std)]

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
