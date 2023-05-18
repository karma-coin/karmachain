use codec::Codec;
use scale_info::prelude::vec::Vec;
use sp_common::types::CommunityId;
use sp_rpc::{Contact, UserInfo};

sp_api::decl_runtime_apis! {
	pub trait IdentityApi<AccountId, Username, PhoneNumber>
	where
		AccountId: Codec,
		Username: Codec,
		PhoneNumber: Codec,
	{
		/// Provide additional information about user by `AccountId`
		fn get_user_info_by_account(
			account_id: AccountId,
		) -> Option<UserInfo<AccountId>>;

		/// Provide additional information about user by `Name`
		fn get_user_info_by_name(
			name: Username,
		) -> Option<UserInfo<AccountId>>;

		/// Provide additional information about user by `PhoneNumber`
		fn get_user_info_by_number(
			number: PhoneNumber,
		) -> Option<UserInfo<AccountId>>;

		/// Provide list of community members with information about each member
		fn get_all_users(
			community_id: CommunityId,
		) -> Vec<UserInfo<AccountId>>;

		/// Get list of user by username prefix and community
		fn get_contacts(
			prefix: Username,
			community_id: Option<CommunityId>,
		) -> Vec<Contact<AccountId>>;
	}
}
