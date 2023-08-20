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
use sp_common::traits::MaybeNormalized;
use sp_core::{crypto::AccountId32, hashing::blake2_512, ByteArray};
use sp_keystore::Keystore;
use sp_rpc::{ByPassToken, VerificationEvidence, VerificationResponse, VerificationResult};
use sp_runtime::{traits::Block as BlockT, KeyTypeId};
use std::sync::Arc;

const KEY_TYPE: KeyTypeId = KeyTypeId(*b"Veri");

pub struct Verifier<C, P> {
	/// Shared reference to the client.
	client: Arc<C>,
	crypto_store: Arc<dyn Keystore>,
	bypass_token: ByPassToken,
	auth_dst: String,
	_marker: std::marker::PhantomData<P>,
}

impl<C, P> Verifier<C, P> {
	pub fn new(
		client: Arc<C>,
		crypto_store: Arc<dyn Keystore>,
		bypass_token: ByPassToken,
		auth_dst: String,
	) -> Self {
		Self { client, crypto_store, bypass_token, auth_dst, _marker: Default::default() }
	}
}

#[async_trait]
impl<C, Block, AccountId, Username, PhoneNumber, PhoneNumberHash>
	VerifierApiServer<AccountId, Username, PhoneNumber, PhoneNumberHash> for Verifier<C, Block>
where
	Block: BlockT + Send + Sync + 'static,
	AccountId:
		Codec + Clone + Send + Sync + 'static + From<sp_core::ed25519::Public> + Into<AccountId32>,
	Username: MaybeNormalized + Codec + Clone + Send + Sync + 'static,
	PhoneNumber: Codec + Clone + Send + Sync + 'static + TryInto<String>,
	<PhoneNumber as TryInto<String>>::Error: std::fmt::Display,
	Vec<u8>: From<PhoneNumber>,
	PhoneNumberHash: Codec + Clone + Send + Sync + 'static + From<[u8; 64]>,
	C: ProvideRuntimeApi<Block> + HeaderBackend<Block> + Send + Sync + 'static,
	C::Api: VerifierApi<Block, AccountId, Username, PhoneNumberHash>,
{
	async fn verify(
		&self,
		account_id: AccountId,
		username: Username,
		phone_number: PhoneNumber,
		bypass_token: Option<ByPassToken>,
	) -> RpcResult<VerificationResponse<AccountId, Username, PhoneNumberHash>> {
		let api = self.client.runtime_api();
		let at = self.client.info().best_hash;

		let username = username.normalize();
		let phone_number_hash =
			PhoneNumberHash::from(blake2_512(Vec::from(phone_number.clone()).as_slice()));

		// Verify input parameters for `new_user` tx
		match api
			.verify(at, &account_id, &username, &phone_number_hash)
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
				.authenticate(gen_auth_request(account_id.clone(), phone_number.clone())?)
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
			let public_key = Keystore::ed25519_public_keys(self.crypto_store.as_ref(), KEY_TYPE)
				.pop()
				.ok_or(map_err(Error::KeyNotFound, "No verifier keys"))?;

			let verifier_public_key: AccountId = public_key.into();
			let data = VerificationEvidence {
				verifier_public_key: verifier_public_key.clone(),
				account_id: account_id.clone(),
				username: username.clone(),
				phone_number_hash: phone_number_hash.clone(),
			}
			.encode();

			let bytes =
				Keystore::ed25519_sign(self.crypto_store.as_ref(), KEY_TYPE, &public_key, &data)
					.map_err(|e| map_err(Error::SignatureFailed, e))?
					.ok_or(map_err(Error::SignatureFailed, "Internal error"))?;
			let signature = sp_core::ed25519::Signature::try_from(bytes.0.as_slice())
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

fn gen_auth_request<AccountId, PhoneNumber>(
	account_id: AccountId,
	phone_number: PhoneNumber,
) -> Result<AuthRequest, CallError>
where
	AccountId: Into<AccountId32>,
	PhoneNumber: TryInto<String>,
	<PhoneNumber as TryInto<String>>::Error: std::fmt::Display,
{
	let account_id: AccountId32 = account_id.into();
	let phone_number = phone_number.try_into().map_err(|e| map_err(Error::InvalidString, e))?;

	Ok(AuthRequest {
		account_id: Some(base::karma_coin::karma_coin_core_types::AccountId {
			data: account_id.to_raw_vec(),
		}),
		phone_number,
	})
}
