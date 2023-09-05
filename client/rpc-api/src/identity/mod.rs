pub mod client;

use jsonrpsee::{core::RpcResult, proc_macros::rpc};
use sp_common::types::CommunityId;
use sp_rpc::{Contact, UserInfo};

#[rpc(client, server)]
pub trait IdentityApi<BlockHash, AccountId, Username, PhoneNumber, PhoneNumberHash> {
	/// RPC method provides information about user account by `AccountId`
	#[method(name = "identity_getUserInfoByAccountId")]
	fn get_user_info_by_account_id(
		&self,
		account_id: AccountId,
		at: Option<BlockHash>,
	) -> RpcResult<Option<UserInfo<AccountId>>>;

	/// RPC method provides information about user account by `Username`
	#[method(name = "identity_getUserInfoByUsername")]
	fn get_user_info_by_username(
		&self,
		username: Username,
		at: Option<BlockHash>,
	) -> RpcResult<Option<UserInfo<AccountId>>>;

	/// RPC method provides information about user account by `PhoneNumber`
	#[method(name = "identity_getUserInfoByPhoneNumber")]
	fn get_user_info_by_phone_number(
		&self,
		phone_number: PhoneNumber,
		at: Option<BlockHash>,
	) -> RpcResult<Option<UserInfo<AccountId>>>;

	/// RPC method provides information about user account by `PhoneNumberHash`
	#[method(name = "identity_getUserInfoByPhoneNumberHash")]
	fn get_user_info_by_phone_number_hash(
		&self,
		phone_number_hash: PhoneNumberHash,
		at: Option<BlockHash>,
	) -> RpcResult<Option<UserInfo<AccountId>>>;

	/// RPC method provides account metadata
	#[method(name = "identity_getMetadata")]
	fn get_metadata(
		&self,
		account_id: AccountId,
		at: Option<BlockHash>,
	) -> RpcResult<Option<Vec<u8>>>;

	/// RPC method provides list of community members with information
	/// about each member account
	#[method(name = "community_getAllUsers")]
	fn get_all_users(
		&self,
		community_id: CommunityId,
		from_index: Option<u64>,
		limit: Option<u64>,
		at: Option<BlockHash>,
	) -> RpcResult<Vec<UserInfo<AccountId>>>;

	/// RPC method provides list of users who's name starts with `prefix`
	/// also can be filtered by `community_id`, `None` mean no filtering
	#[method(name = "community_getContacts")]
	fn get_contacts(
		&self,
		prefix: Username,
		community_id: Option<CommunityId>,
		from_index: Option<u64>,
		limit: Option<u64>,
		at: Option<BlockHash>,
	) -> RpcResult<Vec<Contact<AccountId>>>;

	/// RPC method provides info about karma rewards period leaderboard
	#[method(name = "community_getLeaderBoard")]
	fn get_leader_board(&self, at: Option<BlockHash>) -> RpcResult<Vec<UserInfo<AccountId>>>;
}
