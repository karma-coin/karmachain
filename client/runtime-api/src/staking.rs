use codec::Codec;
use sp_rpc::{Nominations, ValidatorPrefs};
use sp_std::vec::Vec;

sp_api::decl_runtime_apis! {
	/// Runtime API for accessing information about staking.
	pub trait StakingApi<AccountId>
		where
			AccountId: Codec,
	{
		/// Returns the list of current validators.
		fn get_validators() -> Vec<ValidatorPrefs<AccountId>>;

		/// Returns the list of nomination of the account, `None` means the account is not nominated.
		fn get_nominations(account_id: AccountId) -> Option<Nominations<AccountId>>;
	}
}
