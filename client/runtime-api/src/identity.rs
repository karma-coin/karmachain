use codec::{Codec, MaxEncodedLen};
use scale_info::prelude::vec::Vec;
use sp_common::{identity::AccountIdentity, types::CommunityId};
use sp_rpc::{Contact, LeaderBoardEntry, UserInfo};
use sp_std::fmt::Debug;

sp_api::decl_runtime_apis! {
	pub trait IdentityApi<AccountId, Username, PhoneNumberHash>
	where
		AccountId: Codec + MaxEncodedLen + Eq + Debug + Clone,
		Username: Codec + MaxEncodedLen + Eq + Debug + Clone,
		PhoneNumberHash: Codec + MaxEncodedLen + Eq + Debug + Clone,
	{
		/// Provide additional information about user
		fn get_user_info(
			account_identity: AccountIdentity<AccountId, Username, PhoneNumberHash>,
		) -> Option<UserInfo<AccountId>>;

		/// Provide list of community members with information about each member
		fn get_all_users(
			community_id: CommunityId,
			from_index: Option<u64>,
			limit: Option<u64>,
		) -> Vec<UserInfo<AccountId>>;

		/// Get list of user by username prefix and community
		fn get_contacts(
			prefix: Username,
			community_id: Option<CommunityId>,
			from_index: Option<u64>,
			limit: Option<u64>,
		) -> Vec<Contact<AccountId>>;

		// Get list of users that participate in karma reward
		fn get_leader_board() -> Vec<LeaderBoardEntry<AccountId>>;
	}
}
