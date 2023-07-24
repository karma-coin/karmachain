pub mod client;
pub mod error;

use jsonrpsee::{core::RpcResult, proc_macros::rpc};
use sp_rpc::{ByPassToken, VerificationResponse};

#[rpc(client, server)]
pub trait VerifierApi<AccountId, Username, PhoneNumber, PhoneNumberHash> {
	/// RPC method provide verification evidence for `new_user` tx
	#[method(name = "verifier_verify")]
	async fn verify(
		&self,
		account_id: AccountId,
		username: Username,
		phone_number: PhoneNumber,
		bypass_token: Option<ByPassToken>,
	) -> RpcResult<VerificationResponse<AccountId, Username, PhoneNumberHash>>;
}
