use crate::identity::IdentityApiServer;
use codec::Codec;
use jsonrpsee::{
	core::RpcResult,
	types::{error::CallError, ErrorObject},
};
use runtime_api::identity::IdentityApi;
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_common::types::CommunityId;
use sp_rpc::{Contact, LeaderboardEntry, UserInfo};
use sp_runtime::{generic::BlockId, traits::Block as BlockT};
use std::sync::Arc;

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

impl<C, Block, AccountId, Username, PhoneNumber>
	IdentityApiServer<Block::Hash, AccountId, Username, PhoneNumber> for Identity<C, Block>
where
	Block: BlockT,
	AccountId: Codec,
	Username: Codec,
	PhoneNumber: Codec,
	C: ProvideRuntimeApi<Block> + HeaderBackend<Block> + Send + Sync + 'static,
	C::Api: IdentityApi<Block, AccountId, Username, PhoneNumber>,
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
		name: Username,
		at: Option<<Block as BlockT>::Hash>,
	) -> RpcResult<Option<UserInfo<AccountId>>> {
		let api = self.client.runtime_api();
		let at = BlockId::hash(at.unwrap_or_else(|| self.client.info().best_hash));

		let name = name.try_into().map_err(|e| {
			CallError::Custom(ErrorObject::owned(
				0,
				"Name length out of bounds.",
				Some(format!("{:?}", e)),
			))
		})?;

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
		number: PhoneNumber,
		at: Option<<Block as BlockT>::Hash>,
	) -> RpcResult<Option<UserInfo<AccountId>>> {
		let api = self.client.runtime_api();
		let at = BlockId::hash(at.unwrap_or_else(|| self.client.info().best_hash));

		let number = number.try_into().map_err(|e| {
			CallError::Custom(ErrorObject::owned(
				0,
				"Name length out of bounds.",
				Some(format!("{:?}", e)),
			))
		})?;

		Ok(api.get_user_info_by_number(&at, number).map_err(|e| {
			CallError::Custom(ErrorObject::owned(
				0,
				"Unable to query user info.",
				Some(format!("{:?}", e)),
			))
		})?)
	}

	fn get_all_users(
		&self,
		community_id: CommunityId,
		at: Option<<Block as BlockT>::Hash>,
	) -> RpcResult<Vec<UserInfo<AccountId>>> {
		let api = self.client.runtime_api();
		let at = BlockId::hash(at.unwrap_or_else(|| self.client.info().best_hash));

		Ok(api.get_all_users(&at, community_id).map_err(|e| {
			CallError::Custom(ErrorObject::owned(
				0,
				"Unable to query community members.",
				Some(format!("{:?}", e)),
			))
		})?)
	}

	fn get_contacts(
		&self,
		prefix: Username,
		community_id: Option<CommunityId>,
		at: Option<<Block as BlockT>::Hash>,
	) -> RpcResult<Vec<Contact<AccountId>>> {
		let api = self.client.runtime_api();
		let at = BlockId::hash(at.unwrap_or_else(|| self.client.info().best_hash));

		Ok(api.get_contacts(&at, prefix, community_id).map_err(|e| {
			CallError::Custom(ErrorObject::owned(
				0,
				"Unable to query community members.",
				Some(format!("{:?}", e)),
			))
		})?)
	}

	fn get_leader_board(&self) -> RpcResult<Vec<LeaderboardEntry<AccountId>>> {
		// TODO: impl this with karma rewards logic
		Ok(vec![])
	}
}
