#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Codec, Decode, Encode};
use scale_info::prelude::vec::Vec;
use sp_runtime::{traits::Get, BoundedVec};

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
	// trait_scores
	// pre_keys
	// karma_score
}

sp_api::decl_runtime_apis! {
	pub trait IdentityApi<AccountId: Codec> {
		fn get_user_info_by_account(
			account_id: AccountId,
		) -> Option<UserInfo<AccountId>>;

		fn get_user_info_by_name(
			name: Vec<u8>,
		) -> Option<UserInfo<AccountId>>;

		fn get_user_info_by_number(
			number: Vec<u8>,
		) -> Option<UserInfo<AccountId>>;
	}
}
