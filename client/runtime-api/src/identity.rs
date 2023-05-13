use codec::Codec;
use scale_info::prelude::vec::Vec;
use sp_common::{bounded_string::BoundedString, types::CommunityId};
use sp_rpc::{Contact, UserInfo};
use sp_runtime::traits::Get;

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
			name: BoundedString<NameLimit>,
		) -> Option<UserInfo<AccountId>>;

		/// Provide additional information about user by `PhoneNumber`
		fn get_user_info_by_number(
			number: BoundedString<PhoneNumberLimit>,
		) -> Option<UserInfo<AccountId>>;

		/// Provide list of community members with information about each member
		fn get_all_users(
			community_id: CommunityId,
		) -> Vec<UserInfo<AccountId>>;

		/// Get list of user by username prefix and community
		fn get_contacts(
			prefix: BoundedString<NameLimit>,
			community_id: Option<CommunityId>,
		) -> Vec<Contact<AccountId>>;
	}
}
