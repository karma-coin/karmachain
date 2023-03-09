use std::sync::Arc;

use codec::Codec;
use jsonrpsee::{
	core::RpcResult,
	proc_macros::rpc,
	types::error::{CallError, ErrorObject},
};
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_runtime::{generic::BlockId, traits::Block as BlockT};

pub use pallet_identity_rpc_runtime_api::{IdentityApi as IdentityRuntimeApi, UserInfo};

#[rpc(client, server)]
pub trait IdentityApi<BlockHash, AccountId> {
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
}

pub struct Identity<C, P> {
	/// Shared reference to the client.
	client: Arc<C>,
	_marker: std::marker::PhantomData<P>,
}

impl<C, P> Identity<C, P> {
	pub fn new(client: Arc<C>) -> Self {
		Self { client, _marker: Default::default() }
	}
}

impl<C, Block, AccountId> IdentityApiServer<<Block as BlockT>::Hash, AccountId>
	for Identity<C, Block>
where
	AccountId: Codec,
	Block: BlockT,
	C: ProvideRuntimeApi<Block> + HeaderBackend<Block> + Send + Sync + 'static,
	C::Api: IdentityRuntimeApi<Block, AccountId>,
{
	fn get_user_info_by_account(
		&self,
		account_id: AccountId,
		at: Option<<Block as BlockT>::Hash>,
	) -> RpcResult<Option<UserInfo<AccountId>>> {
		let api = self.client.runtime_api();
		let at = BlockId::hash(at.unwrap_or_else(|| self.client.info().best_hash));

		Ok(api.get_user_info_by_account(&at, account_id).map_err(|e| {
			CallError::Custom(ErrorObject::owned(
				0,
				"Unable to query user info.",
				Some(format!("{:?}", e)),
			))
		})?)
	}

	fn get_user_info_by_name(
		&self,
		name: Vec<u8>,
		at: Option<<Block as BlockT>::Hash>,
	) -> RpcResult<Option<UserInfo<AccountId>>> {
		let api = self.client.runtime_api();
		let at = BlockId::hash(at.unwrap_or_else(|| self.client.info().best_hash));

		Ok(api.get_user_info_by_name(&at, name).map_err(|e| {
			CallError::Custom(ErrorObject::owned(
				0,
				"Unable to query user info.",
				Some(format!("{:?}", e)),
			))
		})?)
	}

	fn get_user_info_by_number(
		&self,
		number: Vec<u8>,
		at: Option<<Block as BlockT>::Hash>,
	) -> RpcResult<Option<UserInfo<AccountId>>> {
		let api = self.client.runtime_api();
		let at = BlockId::hash(at.unwrap_or_else(|| self.client.info().best_hash));

		Ok(api.get_user_info_by_number(&at, number).map_err(|e| {
			CallError::Custom(ErrorObject::owned(
				0,
				"Unable to query user info.",
				Some(format!("{:?}", e)),
			))
		})?)
	}
}
