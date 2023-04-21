use codec::Codec;
use scale_info::prelude::vec::Vec;
use sp_common::types::CommunityId;
use sp_rpc::UserInfo;
use sp_runtime::{traits::Get, BoundedVec};

sp_api::decl_runtime_apis! {
	pub trait IdentityApi<AccountId, NameLimit, PhoneNumberLimit>
	where
		AccountId: Codec,
		NameLimit: Get<u32>,
		PhoneNumberLimit: Get<u32>,
	{
		/// Provide additional information about user by `AccountId`
		fn get_user_info_by_account(
			account_id: AccountId,
		) -> Option<UserInfo<AccountId>>;

		/// Provide additional information about user by `Name`
		fn get_user_info_by_name(
			name: BoundedVec<u8, NameLimit>,
		) -> Option<UserInfo<AccountId>>;

		/// Provide additional information about user by `PhoneNumber`
		fn get_user_info_by_number(
			number: BoundedVec<u8, PhoneNumberLimit>,
		) -> Option<UserInfo<AccountId>>;

		fn get_all_users(
			community_id: CommunityId,
		) -> Vec<UserInfo<AccountId>>;
	}
}
