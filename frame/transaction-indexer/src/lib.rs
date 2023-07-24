#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::dispatch::DispatchResult;
pub use pallet::*;
use sp_common::{
	traits::IdentityProvider,
	types::{CharTraitId, CommunityId},
};
use sp_runtime::traits::{BlockNumberProvider, Hash};

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::BlockNumberFor;
	use sp_std::vec::Vec;

	#[pallet::pallet]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config:
		frame_system::Config
		+ pallet_balances::Config
		+ pallet_identity::Config
		+ pallet_appreciation::Config
	{
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

	#[pallet::storage]
	#[pallet::getter(fn phone_number_hash_tx)]
	pub type PhoneNumberHashTransactions<T: Config> =
		StorageMap<_, Blake2_128Concat, T::PhoneNumberHash, Vec<(T::BlockNumber, u32)>>;

	#[pallet::storage]
	pub type TransactionsCount<T: Config> = StorageValue<_, u64, ValueQuery>;

	#[pallet::storage]
	pub type PaymentTransactionsCount<T: Config> = StorageValue<_, u64, ValueQuery>;

	#[pallet::storage]
	pub type AppreciationTransactionsCount<T: Config> = StorageValue<_, u64, ValueQuery>;

	#[pallet::storage]
	pub type UpdateUserTransactionsCount<T: Config> = StorageValue<_, u64, ValueQuery>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/main-docs/build/events-errors/
	#[pallet::event]
	pub enum Event<T: Config> {}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Attempted to call `store` outside of block execution.
		BadContext,
		/// User date not found
		NotFound,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn on_finalize(_n: BlockNumberFor<T>) {
			let extrinsic_count: u64 = frame_system::Pallet::<T>::extrinsic_count().into();
			TransactionsCount::<T>::mutate(|value| *value += extrinsic_count);
		}
	}
}

impl<T: Config> Pallet<T> {
	fn index_transaction_by_account_id(account_id: T::AccountId) -> DispatchResult {
		let block_number = <frame_system::Pallet<T>>::current_block_number();
		let extrinsic_index =
			<frame_system::Pallet<T>>::extrinsic_index().ok_or(Error::<T>::BadContext)?;
		let extrinsic_data = <frame_system::Pallet<T>>::extrinsic_data(extrinsic_index);
		let hash = T::Hashing::hash(&extrinsic_data);

		TxHashes::<T>::insert(hash, (block_number, extrinsic_index));
		AccountTransactions::<T>::append(account_id, (block_number, extrinsic_index));

		Ok(())
	}

	fn index_transaction_by_phone_number_hash(
		phone_number_hash: T::PhoneNumberHash,
	) -> DispatchResult {
		let block_number = <frame_system::Pallet<T>>::current_block_number();
		let extrinsic_index =
			<frame_system::Pallet<T>>::extrinsic_index().ok_or(Error::<T>::BadContext)?;
		let extrinsic_data = <frame_system::Pallet<T>>::extrinsic_data(extrinsic_index);
		let hash = T::Hashing::hash(&extrinsic_data);

		TxHashes::<T>::insert(hash, (block_number, extrinsic_index));
		PhoneNumberHashTransactions::<T>::append(
			phone_number_hash,
			(block_number, extrinsic_index),
		);

		Ok(())
	}
}

impl<T: Config> sp_common::hooks::Hooks<T::AccountId, T::Balance, T::Username, T::PhoneNumberHash>
	for Pallet<T>
{
	fn on_new_user(
		_verifier: T::AccountId,
		who: T::AccountId,
		_name: T::Username,
		phone_number_hash: T::PhoneNumberHash,
	) -> DispatchResult {
		Self::index_transaction_by_account_id(who)?;
		Self::index_transaction_by_phone_number_hash(phone_number_hash)
	}

	fn on_update_user(
		old_account_id: T::AccountId,
		new_account_id: Option<T::AccountId>,
		_username: T::Username,
		_new_username: Option<T::Username>,
		phone_number_hash: T::PhoneNumberHash,
		new_phone_number_hash: Option<T::PhoneNumberHash>,
	) -> DispatchResult {
		UpdateUserTransactionsCount::<T>::mutate(|value| *value += 1);

		Self::index_transaction_by_account_id(old_account_id)?;
		if let Some(new_account_id) = new_account_id {
			Self::index_transaction_by_account_id(new_account_id)?;
		}

		Self::index_transaction_by_phone_number_hash(phone_number_hash)?;
		if let Some(new_phone_number_hash) = new_phone_number_hash {
			Self::index_transaction_by_phone_number_hash(new_phone_number_hash)?;
		}

		Ok(())
	}

	fn on_appreciation(
		payer: T::AccountId,
		payee: T::AccountId,
		_amount: T::Balance,
		_community_id: CommunityId,
		char_trait_id: CharTraitId,
	) -> DispatchResult {
		let payer_phone_number_hash = T::IdentityProvider::identity_by_id(&payer)
			.ok_or(Error::<T>::NotFound)?
			.phone_number_hash;
		let payee_phone_number_hash = T::IdentityProvider::identity_by_id(&payer)
			.ok_or(Error::<T>::NotFound)?
			.phone_number_hash;

		Self::index_transaction_by_account_id(payer)?;
		Self::index_transaction_by_account_id(payee)?;

		Self::index_transaction_by_phone_number_hash(payer_phone_number_hash)?;
		Self::index_transaction_by_phone_number_hash(payee_phone_number_hash)?;

		let no_char_trait_id = pallet_appreciation::Pallet::<T>::no_char_trait_id()?;

		if char_trait_id == no_char_trait_id {
			PaymentTransactionsCount::<T>::mutate(|value| *value += 1);
		} else {
			AppreciationTransactionsCount::<T>::mutate(|value| *value += 1);
		}

		Ok(())
	}

	fn on_set_admin(who: T::AccountId, new_admin: T::AccountId) -> DispatchResult {
		let who_phone_number_hash = T::IdentityProvider::identity_by_id(&who)
			.ok_or(Error::<T>::NotFound)?
			.phone_number_hash;
		let new_admin_phone_number_hash = T::IdentityProvider::identity_by_id(&new_admin)
			.ok_or(Error::<T>::NotFound)?
			.phone_number_hash;

		Self::index_transaction_by_account_id(who)?;
		Self::index_transaction_by_account_id(new_admin)?;

		Self::index_transaction_by_phone_number_hash(who_phone_number_hash)?;
		Self::index_transaction_by_phone_number_hash(new_admin_phone_number_hash)
	}

	fn on_delete_user(
		account_id: T::AccountId,
		_username: T::Username,
		phone_number_hash: T::PhoneNumberHash,
	) -> DispatchResult {
		AccountTransactions::<T>::remove(&account_id);
		PhoneNumberHashTransactions::<T>::remove(&phone_number_hash);

		Ok(())
	}
}
