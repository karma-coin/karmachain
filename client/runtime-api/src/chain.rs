use codec::Codec;
use sp_rpc::{BlockchainStats, CharTrait, GenesisData};
use sp_std::vec::Vec;

sp_api::decl_runtime_apis! {
	pub trait ChainDataProvider<SignedBlock, AccountId, Hash>
	where
		SignedBlock: Codec,
		AccountId: Codec,
		Hash: Codec,
	{
		/// Provide information about current blockchain state
		fn get_blockchain_data() -> BlockchainStats;

		/// Provide information about blockchain genesis config
		fn get_genesis_data() -> GenesisData<AccountId>;

		/// Provide list of char traits
		fn get_char_traits(from_index: Option<u32>, limit: Option<u32>) -> Vec<CharTrait>;
	}
}
