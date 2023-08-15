pub mod client;
pub mod error;

use jsonrpsee::{core::RpcResult, proc_macros::rpc};
use sp_rpc::{Nominations, ValidatorPrefs};

#[rpc(client, server)]
pub trait StakingApi<BlockHash, AccountId> {
	#[method(name = "staking_getValidators")]
	fn get_validators(&self, at: Option<BlockHash>) -> RpcResult<Vec<ValidatorPrefs<AccountId>>>;

	#[method(name = "staking_getNominations")]
	fn get_nominations(
		&self,
		account_id: AccountId,
		at: Option<BlockHash>,
	) -> RpcResult<Option<Nominations<AccountId>>>;
}
