pub mod client;

use jsonrpsee::{core::RpcResult, proc_macros::rpc};
use sp_common::types::CommunityId;
use sp_rpc::UserInfo;

#[rpc(client, server)]
pub trait IdentityApi<BlockHash, AccountId, NameLimit, PhoneNumberLimit> {
	#[method(name = "identity_getUserInfoByAccount")]
	fn get_user_info_by_account(
		&self,
		account_id: AccountId,
		at: Option<BlockHash>,
	) -> RpcResult<Option<UserInfo<AccountId>>>;

	#[method(name = "identity_getUserInfoByName")]
	fn get_user_info_by_name(
		&self,
		name: Vec<u8>,
		at: Option<BlockHash>,
	) -> RpcResult<Option<UserInfo<AccountId>>>;

	#[method(name = "identity_getUserInfoByNumber")]
	fn get_user_info_by_number(
		&self,
		number: Vec<u8>,
		at: Option<BlockHash>,
	) -> RpcResult<Option<UserInfo<AccountId>>>;

	#[method(name = "community_get_all_users")]
	fn get_all_users(
		&self,
		community_id: CommunityId,
		at: Option<BlockHash>,
	) -> RpcResult<Vec<UserInfo<AccountId>>>;
}
