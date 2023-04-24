#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::{
	pallet_prelude::DispatchResult, traits::Get, BoundedVec, CloneNoBound, PartialEqNoBound,
	RuntimeDebugNoBound,
};
use scale_info::TypeInfo;
use sp_std::{prelude::*, vec};

pub use pallet::*;
use sp_common::identity::{AccountIdentity, IdentityInfo, IdentityProvider};

#[derive(
	CloneNoBound, Encode, Decode, Eq, MaxEncodedLen, PartialEqNoBound, RuntimeDebugNoBound, TypeInfo,
)]
#[codec(mel_bound())]
#[scale_info(skip_type_params(NameLimit, PhoneNumberLimit))]
pub struct IdentityStore<NameLimit: Get<u32>, PhoneNumberLimit: Get<u32>> {
	pub name: BoundedVec<u8, NameLimit>,
	pub phone_number: BoundedVec<u8, PhoneNumberLimit>,
}

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use sp_common::hooks::Hooks;

	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_balances::Config {
		/// The overarching event type.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		/// Max length of name
		type NameLimit: Get<u32>;
		/// Max length of number
		type PhoneNumberLimit: Get<u32>;
		/// Max number of phone verifiers allowed
		type MaxPhoneVerifiers: Get<u32>;
		/// Handler for when a new user has just been registered
		type Hooks: Hooks<Self::AccountId, Self::Balance, Self::NameLimit, Self::PhoneNumberLimit>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config> {
		pub phone_verifiers: sp_std::vec::Vec<T::AccountId>,
	}

	#[cfg(feature = "std")]
	impl<T: Config> Default for GenesisConfig<T> {
		fn default() -> Self {
			Self { phone_verifiers: vec![] }
		}
	}

	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
		fn build(&self) {
			let bounded_phone_verifiers: BoundedVec<T::AccountId, T::MaxPhoneVerifiers> =
				self.phone_verifiers.clone().try_into().expect(
					"Initial number of phone_verifiers should be lower than T::MaxPhoneVerifiers",
				);
			PhoneVerifiers::<T>::put(bounded_phone_verifiers);
		}
	}

	#[cfg(feature = "std")]
	impl<T: Config> GenesisConfig<T> {
		/// Direct implementation of `GenesisBuild::build_storage`.
		///
		/// Kept in order not to break dependency.
		pub fn build_storage(&self) -> Result<sp_runtime::Storage, std::string::String> {
			<Self as GenesisBuild<T>>::build_storage(self)
		}
	}

	#[pallet::storage]
	pub type IdentityOf<T: Config> = CountedStorageMap<
		_,
		Blake2_128Concat,
		T::AccountId,
		IdentityStore<T::NameLimit, T::PhoneNumberLimit>,
	>;

	#[pallet::storage]
	pub type NameFor<T: Config> =
		StorageMap<_, Blake2_128Concat, BoundedVec<u8, T::NameLimit>, T::AccountId>;

	#[pallet::storage]
	pub type PhoneNumberFor<T: Config> =
		StorageMap<_, Blake2_128Concat, BoundedVec<u8, T::PhoneNumberLimit>, T::AccountId>;

	#[pallet::storage]
	#[pallet::getter(fn verifiers)]
	pub type PhoneVerifiers<T: Config> =
		StorageValue<_, BoundedVec<T::AccountId, T::MaxPhoneVerifiers>, ValueQuery>;

	#[pallet::error]
	pub enum Error<T> {
		/// By this ACCOUNT ID already already registered user
		AlreadyRegistered,
		/// There's already a user with the requested user name
		UserNameTaken,
		/// There's already a user with the requested phone number
		PhoneNumberTaken,
		/// Account isn't found.
		NotFound,
		///
		NotAllowed,
	}

	#[pallet::event]
	pub enum Event<T: Config> {}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(3,1).ref_time())]
		pub fn new_user(
			origin: OriginFor<T>,
			account_id: T::AccountId,
			name: BoundedVec<u8, T::NameLimit>,
			phone_number: BoundedVec<u8, T::PhoneNumberLimit>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			ensure!(PhoneVerifiers::<T>::get().contains(&who), Error::<T>::NotAllowed);

			if IdentityOf::<T>::contains_key(&account_id) {
				return Err(Error::<T>::AlreadyRegistered.into())
			}

			if NameFor::<T>::contains_key(&name) {
				return Err(Error::<T>::UserNameTaken.into())
			}

			if PhoneNumberFor::<T>::contains_key(&phone_number) {
				return Err(Error::<T>::PhoneNumberTaken.into())
			}

			NameFor::<T>::insert(&name, account_id.clone());
			PhoneNumberFor::<T>::insert(&phone_number, account_id.clone());
			IdentityOf::<T>::insert(
				&account_id,
				IdentityStore { name: name.clone(), phone_number: phone_number.clone() },
			);

			T::Hooks::on_new_user(who, account_id, name, phone_number)?;

			Ok(())
		}
	}
}

impl<T: Config> IdentityProvider<T::AccountId, T::NameLimit, T::PhoneNumberLimit> for Pallet<T> {
	fn exist_by_identity(
		account_identity: &AccountIdentity<T::AccountId, T::NameLimit, T::PhoneNumberLimit>,
	) -> bool {
		match account_identity {
			AccountIdentity::AccountId(account_id) => IdentityOf::<T>::get(account_id).is_some(),
			AccountIdentity::PhoneNumber(number) => PhoneNumberFor::<T>::get(number).is_some(),
			AccountIdentity::Name(name) => NameFor::<T>::get(name).is_some(),
		}
	}

	fn identity_by_id(
		account_id: T::AccountId,
	) -> Option<IdentityInfo<T::AccountId, T::NameLimit, T::PhoneNumberLimit>> {
		<IdentityOf<T>>::get(&account_id).map(|v| IdentityInfo {
			account_id,
			name: v.name,
			number: v.phone_number,
		})
	}

	fn identity_by_name(
		name: BoundedVec<u8, T::NameLimit>,
	) -> Option<IdentityInfo<T::AccountId, T::NameLimit, T::PhoneNumberLimit>> {
		<NameFor<T>>::get(name).and_then(Self::identity_by_id)
	}

	fn identity_by_number(
		number: BoundedVec<u8, T::PhoneNumberLimit>,
	) -> Option<IdentityInfo<T::AccountId, T::NameLimit, T::PhoneNumberLimit>> {
		<PhoneNumberFor<T>>::get(number).and_then(Self::identity_by_id)
	}
}

impl<T: Config> Pallet<T> {
	/// Search for registered user who's username start with given `prefix`
	pub fn get_contacts(
		prefix: BoundedVec<u8, T::NameLimit>,
	) -> Vec<(T::AccountId, IdentityStore<T::NameLimit, T::PhoneNumberLimit>)> {
		IdentityOf::<T>::iter()
			.filter(|(_key, value)| value.name.starts_with(&prefix))
			.collect()
	}
}
