pub mod client;

use jsonrpsee::{core::RpcResult, proc_macros::rpc};
use sp_common::types::CommunityId;
use sp_rpc::{Contact, LeaderboardEntry, UserInfo};

#[rpc(client, server)]
pub trait IdentityApi<BlockHash, AccountId, Username, PhoneNumber> {
	/// RPC method provides information about user account by `AccountId`
	#[method(name = "identity_getUserInfoByAccount")]
	fn get_user_info_by_account(
		&self,
		account_id: AccountId,
		at: Option<BlockHash>,
	) -> RpcResult<Option<UserInfo<AccountId>>>;

	/// RPC method provides information about user account by `Name`
	#[method(name = "identity_getUserInfoByName")]
	fn get_user_info_by_name(
		&self,
		name: Username,
		at: Option<BlockHash>,
	) -> RpcResult<Option<UserInfo<AccountId>>>;

	/// RPC method provides information about user account by `PhoneNumber`
	#[method(name = "identity_getUserInfoByNumber")]
	fn get_user_info_by_number(
		&self,
		number: PhoneNumber,
		at: Option<BlockHash>,
	) -> RpcResult<Option<UserInfo<AccountId>>>;

	/// RPC method provides list of community members with information
	/// about each member account
	#[method(name = "community_getAllUsers")]
	fn get_all_users(
		&self,
		community_id: CommunityId,
		at: Option<BlockHash>,
	) -> RpcResult<Vec<UserInfo<AccountId>>>;

	/// RPC method provides list of users who's name starts with `prefix`
	/// also can be filtered by `community_id`, `None` mean no filtering
	#[method(name = "community_getContacts")]
	fn get_contacts(
		&self,
		prefix: Username,
		community_id: Option<CommunityId>,
		at: Option<BlockHash>,
	) -> RpcResult<Vec<Contact<AccountId>>>;

	/// RPC method provides info about karma rewards period leaderboard
	#[method(name = "community_getLeaderBoard")]
	fn get_leader_board(&self) -> RpcResult<Vec<LeaderboardEntry<AccountId>>>;
}
