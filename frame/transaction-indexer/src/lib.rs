#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::dispatch::DispatchResult;
pub use pallet::*;
use sp_common::traits::TransactionIndexer;
use sp_runtime::traits::{BlockNumberProvider, Hash};

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use sp_std::vec::Vec;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
	}

	#[pallet::storage]
	#[pallet::getter(fn tx_block_and_index)]
	pub type TxHashes<T: Config> = StorageMap<_, Blake2_128Concat, T::Hash, (T::BlockNumber, u32)>;

	#[pallet::storage]
	#[pallet::getter(fn accounts_tx)]
	pub type AccountTransactions<T: Config> =
		StorageMap<_, Blake2_128Concat, T::AccountId, Vec<(T::BlockNumber, u32)>>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/main-docs/build/events-errors/
	#[pallet::event]
	pub enum Event<T: Config> {}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Attempted to call `store` outside of block execution.
		BadContext,
	}
}

impl<T: Config> TransactionIndexer<T::AccountId> for Pallet<T> {
	fn index_transaction(account_id: T::AccountId) -> DispatchResult {
		let block_number = <frame_system::Pallet<T>>::current_block_number();
		let extrinsic_index =
			<frame_system::Pallet<T>>::extrinsic_index().ok_or(Error::<T>::BadContext)?;
		let extrinsic_data = <frame_system::Pallet<T>>::extrinsic_data(extrinsic_index);
		let hash = T::Hashing::hash(&extrinsic_data);

		TxHashes::<T>::insert(hash, (block_number, extrinsic_index));
		AccountTransactions::<T>::append(account_id, (block_number, extrinsic_index));

		Ok(())
	}
}
