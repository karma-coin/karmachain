use crate::verifier::{
	error::{map_err, Error},
	VerifierApiServer,
};
use base::karma_coin::karma_coin_auth::{auth_service_client::AuthServiceClient, AuthRequest};
use codec::{Codec, Encode};
use jsonrpsee::{
	core::{async_trait, RpcResult},
	types::error::CallError,
};
use runtime_api::verifier::VerifierApi;
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_core::{
	crypto::{AccountId32, CryptoTypePublicPair},
	ByteArray,
};
use sp_keystore::SyncCryptoStore;
use sp_rpc::{ByPassToken, VerificationEvidence, VerificationResponse, VerificationResult};
use sp_runtime::{generic::BlockId, traits::Block as BlockT, KeyTypeId};
use std::sync::Arc;

const KEY_TYPE: KeyTypeId = KeyTypeId(*b"Veri");

pub struct Verifier<C, P> {
	/// Shared reference to the client.
	client: Arc<C>,
	crypto_store: Arc<dyn SyncCryptoStore>,
	bypass_token: ByPassToken,
	auth_dst: String,
	_marker: std::marker::PhantomData<P>,
}

impl<C, P> Verifier<C, P> {
	pub fn new(
		client: Arc<C>,
		crypto_store: Arc<dyn SyncCryptoStore>,
		bypass_token: ByPassToken,
		auth_dst: String,
	) -> Self {
		Self { client, crypto_store, bypass_token, auth_dst, _marker: Default::default() }
	}
}

#[async_trait]
impl<C, Block, AccountId, Username, PhoneNumberHash>
	VerifierApiServer<AccountId, Username, PhoneNumberHash> for Verifier<C, Block>
where
	Block: BlockT + Send + Sync + 'static,
	AccountId:
		Codec + Clone + Send + Sync + 'static + From<sp_core::sr25519::Public> + Into<AccountId32>,
	Username: Codec + Clone + Send + Sync + 'static,
	PhoneNumberHash: Codec + Clone + Send + Sync + 'static,
	C: ProvideRuntimeApi<Block> + HeaderBackend<Block> + Send + Sync + 'static,
	C::Api: VerifierApi<Block, AccountId, Username, PhoneNumberHash>,
{
	async fn verify(
		&self,
		account_id: AccountId,
		username: Username,
		phone_number_hash: PhoneNumberHash,
		bypass_token: Option<ByPassToken>,
	) -> RpcResult<VerificationResponse<AccountId, Username, PhoneNumberHash>> {
		let api = self.client.runtime_api();
		let at = BlockId::hash(self.client.info().best_hash);

		// Verify input parameters for `new_user` tx
		match api
			.verify(&at, &account_id, &username, &phone_number_hash)
			.map_err(|e| map_err(Error::RuntimeError, e))?
		{
			VerificationResult::Verified => {},
			verification_result =>
				return Ok(VerificationResponse {
					verifier_account_id: None,
					verification_result,
					account_id: None,
					phone_number_hash: None,
					username: None,
					signature: None,
				}),
		}

		let verification_result = match bypass_token {
			// Bypass token passed and matched, skip verification
			Some(bypass_token) if bypass_token == self.bypass_token => VerificationResult::Verified,
			// Bypass token passed and do not match, error
			Some(bypass_token) =>
				return Err(map_err(Error::BypassTokenMismatch, bypass_token).into()),
			// No bypass token, verifying number
			None => AuthServiceClient::connect(self.auth_dst.clone())
				.await
				.map_err(|e| map_err(Error::AuthServiceOffline, e))?
				.authenticate(gen_auth_request(account_id.clone(), phone_number_hash.clone())?)
				.await
				.map(|v| v.into_inner().result.into())
				.unwrap_or(VerificationResult::Unverified),
		};

		let mut result = VerificationResponse {
			verifier_account_id: None,
			verification_result,
			account_id: None,
			phone_number_hash: None,
			username: None,
			signature: None,
		};

		if let VerificationResult::Verified = verification_result {
			let public_key =
				SyncCryptoStore::sr25519_public_keys(self.crypto_store.as_ref(), KEY_TYPE)
					.pop()
					.ok_or(map_err(Error::KeyNotFound, "No verifier keys"))?;
			let key = CryptoTypePublicPair(sp_core::sr25519::CRYPTO_ID, public_key.to_raw_vec());

			let verifier_public_key: AccountId = public_key.into();
			let data = VerificationEvidence {
				verifier_public_key: verifier_public_key.clone(),
				account_id: account_id.clone(),
				username: username.clone(),
				phone_number_hash: phone_number_hash.clone(),
			}
			.encode();

			let bytes =
				SyncCryptoStore::sign_with(self.crypto_store.as_ref(), KEY_TYPE, &key, &data)
					.map_err(|e| map_err(Error::SignatureFailed, e))?
					.ok_or(map_err(Error::SignatureFailed, "Internal error"))?;
			let signature = sp_core::sr25519::Signature::try_from(bytes.as_slice())
				.map_err(|_| map_err(Error::SignatureFailed, "Fail to wrap signature"))?;

			result.verifier_account_id = Some(verifier_public_key);
			result.account_id = Some(account_id);
			result.phone_number_hash = Some(phone_number_hash);
			result.username = Some(username);
			result.signature = Some(signature);
		}

		Ok(result)
	}
}

fn gen_auth_request<AccountId, PhoneNumberHash>(
	_account_id: AccountId,
	_phone_number_hash: PhoneNumberHash,
) -> Result<AuthRequest, CallError> {
	// Auth service do not support PhoneNumberHash
	// for now to use verifier API - use bypass token
	todo!()
}
