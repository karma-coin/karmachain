use crate::identity::IdentityApiServer;
use codec::{Codec, MaxEncodedLen};
use jsonrpsee::{
	core::RpcResult,
	types::{error::CallError, ErrorObject},
};
use runtime_api::identity::IdentityApi;
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_common::{identity::AccountIdentity, types::CommunityId};
use sp_core::hashing::blake2_512;
use sp_rpc::{Contact, LeaderBoardEntry, UserInfo};
use sp_runtime::traits::Block as BlockT;
use sp_std::fmt::Debug;
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

impl<C, Block, AccountId, Username, PhoneNumber, PhoneNumberHash>
	IdentityApiServer<Block::Hash, AccountId, Username, PhoneNumber, PhoneNumberHash>
	for Identity<C, Block>
where
	Block: BlockT,
	AccountId: Codec + MaxEncodedLen + Eq + Debug + Clone,
	Username: Codec + MaxEncodedLen + Eq + Debug + Clone,
	PhoneNumber: Codec + MaxEncodedLen + Eq + Debug + Clone,
	Vec<u8>: From<PhoneNumber>,
	PhoneNumberHash: Codec + MaxEncodedLen + Eq + Debug + Clone + From<[u8; 64]>,
	C: ProvideRuntimeApi<Block> + HeaderBackend<Block> + Send + Sync + 'static,
	C::Api: IdentityApi<Block, AccountId, Username, PhoneNumberHash>,
{
	fn get_user_info_by_account_id(
		&self,
		account_id: AccountId,
		at: Option<<Block as BlockT>::Hash>,
	) -> RpcResult<Option<UserInfo<AccountId>>> {
		let api = self.client.runtime_api();
		let at = at.unwrap_or_else(|| self.client.info().best_hash);

		Ok(api.get_user_info(at, AccountIdentity::AccountId(account_id)).map_err(|e| {
			CallError::Custom(ErrorObject::owned(
				0,
				"Unable to query user info.",
				Some(format!("{e:?}")),
			))
		})?)
	}

	fn get_user_info_by_username(
		&self,
		username: Username,
		at: Option<<Block as BlockT>::Hash>,
	) -> RpcResult<Option<UserInfo<AccountId>>> {
		let api = self.client.runtime_api();
		let at = at.unwrap_or_else(|| self.client.info().best_hash);

		Ok(api.get_user_info(at, AccountIdentity::Username(username)).map_err(|e| {
			CallError::Custom(ErrorObject::owned(
				0,
				"Unable to query user info.",
				Some(format!("{e:?}")),
			))
		})?)
	}

	fn get_user_info_by_phone_number(
		&self,
		phone_number: PhoneNumber,
		at: Option<<Block as BlockT>::Hash>,
	) -> RpcResult<Option<UserInfo<AccountId>>> {
		let api = self.client.runtime_api();
		let at = at.unwrap_or_else(|| self.client.info().best_hash);

		let phone_number_hash =
			PhoneNumberHash::from(blake2_512(Vec::from(phone_number.clone()).as_slice()));

		Ok(api
			.get_user_info(at, AccountIdentity::PhoneNumberHash(phone_number_hash))
			.map_err(|e| {
				CallError::Custom(ErrorObject::owned(
					0,
					"Unable to query user info.",
					Some(format!("{e:?}")),
				))
			})?)
	}

	fn get_user_info_by_phone_number_hash(
		&self,
		phone_number_hash: PhoneNumberHash,
		at: Option<<Block as BlockT>::Hash>,
	) -> RpcResult<Option<UserInfo<AccountId>>> {
		let api = self.client.runtime_api();
		let at = at.unwrap_or_else(|| self.client.info().best_hash);

		Ok(api
			.get_user_info(at, AccountIdentity::PhoneNumberHash(phone_number_hash))
			.map_err(|e| {
				CallError::Custom(ErrorObject::owned(
					0,
					"Unable to query user info.",
					Some(format!("{e:?}")),
				))
			})?)
	}

	fn get_all_users(
		&self,
		community_id: CommunityId,
		from_index: Option<u64>,
		limit: Option<u64>,
		at: Option<<Block as BlockT>::Hash>,
	) -> RpcResult<Vec<UserInfo<AccountId>>> {
		let api = self.client.runtime_api();
		let at = at.unwrap_or_else(|| self.client.info().best_hash);

		Ok(api.get_all_users(at, community_id, from_index, limit).map_err(|e| {
			CallError::Custom(ErrorObject::owned(
				0,
				"Unable to query community members.",
				Some(format!("{e:?}")),
			))
		})?)
	}

	fn get_contacts(
		&self,
		prefix: Username,
		community_id: Option<CommunityId>,
		from_index: Option<u64>,
		limit: Option<u64>,
		at: Option<<Block as BlockT>::Hash>,
	) -> RpcResult<Vec<Contact<AccountId>>> {
		let api = self.client.runtime_api();
		let at = at.unwrap_or_else(|| self.client.info().best_hash);

		Ok(api.get_contacts(at, prefix, community_id, from_index, limit).map_err(|e| {
			CallError::Custom(ErrorObject::owned(
				0,
				"Unable to query contacts.",
				Some(format!("{e:?}")),
			))
		})?)
	}

	fn get_leader_board(
		&self,
		at: Option<<Block as BlockT>::Hash>,
	) -> RpcResult<Vec<LeaderBoardEntry<AccountId>>> {
		let api = self.client.runtime_api();
		let at = at.unwrap_or_else(|| self.client.info().best_hash);

		Ok(api.get_leader_board(at).map_err(|e| {
			CallError::Custom(ErrorObject::owned(
				0,
				"Unable to query karma participators.",
				Some(format!("{e:?}")),
			))
		})?)
	}
}
