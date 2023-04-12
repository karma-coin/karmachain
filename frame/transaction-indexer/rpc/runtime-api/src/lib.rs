#![cfg_attr(not(feature = "std"), no_std)]

use codec::Codec;
use scale_info::prelude::vec::Vec;
use sp_runtime::traits::Header;

pub use sp_rpc::*;

sp_api::decl_runtime_apis! {
	pub trait TransactionsApi<AccountId: Codec> {
		fn get_transactions(account_id: AccountId) -> Vec<(<Block::Header as Header>::Number, u32)>;

		fn get_transaction(tx_hash: Block::Hash) -> Option<(<Block::Header as Header>::Number, u32)>;
	}
}
