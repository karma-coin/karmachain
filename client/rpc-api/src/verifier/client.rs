use crate::verifier::{error::map_err, VerifierApiServer};
use base::karma_coin::karma_coin_auth::{auth_service_client::AuthServiceClient, AuthRequest};
use codec::{Codec, Encode};
use jsonrpsee::core::{async_trait, RpcResult};
use sp_api::ProvideRuntimeApi;
use sp_core::{crypto::AccountId32, ByteArray};
use sp_keystore::SyncCryptoStore;
use sp_rpc::{ByPassToken, VerificationEvidence, VerificationResponse, VerificationResult};
use sp_runtime::{traits::Block as BlockT, KeyTypeId};
use std::sync::Arc;

const KEY_TYPE: KeyTypeId = KeyTypeId(*b"Veri");

pub struct Verifier<C, P> {
	/// Shared reference to the client.
	_client: Arc<C>,
	crypto_store: Arc<dyn SyncCryptoStore>,
	bypass_token: ByPassToken,
	_marker: std::marker::PhantomData<P>,
}

impl<C, P> Verifier<C, P> {
	pub fn new(
		client: Arc<C>,
		crypto_store: Arc<dyn SyncCryptoStore>,
		bypass_token: ByPassToken,
	) -> Self {
		Self { _client: client, crypto_store, bypass_token, _marker: Default::default() }
	}
}

#[async_trait]
impl<C, Block, AccountId, Username, PhoneNumber> VerifierApiServer<AccountId, Username, PhoneNumber>
	for Verifier<C, Block>
where
	Block: BlockT + Send + Sync + 'static,
	AccountId:
		Codec + Clone + Send + Sync + 'static + From<sp_core::sr25519::Public> + Into<AccountId32>,
	Username: Codec + Clone + Send + Sync + 'static,
	PhoneNumber: Codec + Clone + Send + Sync + 'static,
	C: ProvideRuntimeApi<Block> + Send + Sync + 'static,
{
	async fn verify(
		&self,
		account_id: AccountId,
		username: Username,
		phone_number: PhoneNumber,
		bypass_token: Option<ByPassToken>,
	) -> RpcResult<VerificationResponse<AccountId, Username, PhoneNumber>> {
		// TODO: perform checks for parameters

		let verification_result = if bypass_token.filter(|v| v.eq(&self.bypass_token)).is_some() {
			// TODO: perform check for `PhoneNumber`
			let account_id: AccountId32 = account_id.clone().into();
			AuthServiceClient::connect("")
				.await
				.unwrap()
				.authenticate(AuthRequest {
					account_id: Some(base::karma_coin::karma_coin_core_types::AccountId {
						data: account_id.to_raw_vec(),
					}),
					phone_number: "".to_string(),
				})
				.await
				.unwrap()
				.into_inner()
				.result
				.into()
		} else {
			VerificationResult::Verified
		};

		let public_keys =
			SyncCryptoStore::sr25519_public_keys(self.crypto_store.as_ref(), KEY_TYPE);
		let keys = SyncCryptoStore::keys(self.crypto_store.as_ref(), KEY_TYPE)
			.map_err(|e| map_err(e, "Keys not found"))?;
		let (public_key, key) = std::iter::zip(public_keys.into_iter(), keys.into_iter())
			.next()
			.ok_or("No keys")
			.map_err(|e| map_err(e, "Keys not found"))?;

		let verifier_account_id: AccountId = public_key.into();
		let data = VerificationEvidence {
			verifier_account_id: verifier_account_id.clone(),
			account_id: account_id.clone(),
			username: username.clone(),
			phone_number: phone_number.clone(),
		}
		.encode();

		let bytes = SyncCryptoStore::sign_with(self.crypto_store.as_ref(), KEY_TYPE, &key, &data)
			.map_err(|e| map_err(e, "Fail to sign"))?
			.ok_or("`sign_with` failed")
			.map_err(|e| map_err(e, "Fail to sign"))?;
		let signature = sp_core::sr25519::Signature::try_from(bytes.as_slice())
			.map_err(|_| map_err("", "Fail to wrap signature"))?;

		Ok(VerificationResponse {
			verifier_account_id,
			verification_result,
			account_id,
			phone_number,
			username,
			signature,
		})
	}
}
