use codec::Codec;
use sp_rpc::VerificationResult;

sp_api::decl_runtime_apis! {
	pub trait VerifierApi<AccountId, Username, PhoneNumber>
	where
		AccountId: Codec,
		Username: Codec,
		PhoneNumber: Codec,
	{
		/// Perform validation for input parameters of `new_user` tx
		fn verify(
			account_id: &AccountId,
			username: &Username,
			phone_number: &PhoneNumber,
		) -> VerificationResult;
	}
}
