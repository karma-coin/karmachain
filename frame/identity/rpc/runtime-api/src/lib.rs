#![cfg_attr(not(feature = "std"), no_std)]

use codec::Codec;
use scale_info::prelude::vec::Vec;

pub use sp_rpc::*;

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
