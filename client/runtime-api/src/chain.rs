use codec::Codec;
use sp_rpc::{BlockchainStats, GenesisData};

sp_api::decl_runtime_apis! {
	pub trait BlockInfoProvider<SignedBlock, AccountId, Hash>
	where
		SignedBlock: Codec,
		AccountId: Codec,
		Hash: Codec,
	{
		/// Provide information about current blockchain state
		fn get_blockchain_data() -> BlockchainStats;

		/// Provide information about blockchain genesis config
		fn get_genesis_data() -> GenesisData<AccountId>;
	}
}
