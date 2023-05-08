#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Codec, Decode, Encode, MaxEncodedLen};
use frame_support::{pallet_prelude::DispatchResult, traits::Get, BoundedVec};
use scale_info::TypeInfo;
use sp_std::{prelude::*, vec};

pub use pallet::*;
use sp_common::{
	identity::{AccountIdentity, IdentityInfo},
	traits::IdentityProvider,
};

#[derive(Clone, Encode, Decode, Eq, MaxEncodedLen, PartialEq, Debug, TypeInfo)]
pub struct IdentityStore<Username, PhoneNumber>
where
	Username: Codec,
	PhoneNumber: Codec,
{
	pub name: Username,
	pub phone_number: PhoneNumber,
}

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::{
		pallet_prelude::*,
		traits::{Currency, ExistenceRequirement},
	};
	use frame_system::pallet_prelude::*;
	use sp_common::hooks::Hooks;
	use sp_std::fmt::Debug;

	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_balances::Config {
		/// The overarching event type.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		/// Max length of username
		type UsernameLimit: Get<u32>;
		/// Username type
		type Username: Parameter + Member + MaybeSerializeDeserialize + Debug + Ord + MaxEncodedLen;
		/// Max length of phone number
		type PhoneNumberLimit: Get<u32>;
		/// Phone number type
		type PhoneNumber: Parameter
			+ Member
			+ MaybeSerializeDeserialize
			+ Debug
			+ Ord
			+ MaxEncodedLen;
		/// Max number of phone verifiers allowed
		type MaxPhoneVerifiers: Get<u32>;
		/// Handler for when a new user has just been registered
		type Hooks: Hooks<Self::AccountId, Self::Balance, Self::Username, Self::PhoneNumber>;
		/// The currency mechanism.
		type Currency: Currency<Self::AccountId, Balance = Self::Balance>;
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
		IdentityStore<T::Username, T::PhoneNumber>,
	>;

	#[pallet::storage]
	pub type NameFor<T: Config> = StorageMap<_, Blake2_128Concat, T::Username, T::AccountId>;

	#[pallet::storage]
	pub type PhoneNumberFor<T: Config> =
		StorageMap<_, Blake2_128Concat, T::PhoneNumber, T::AccountId>;

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
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// New user registered
		NewUser {
			phone_verifier: T::AccountId,
			account_id: T::AccountId,
			name: T::Username,
			phone_number: T::PhoneNumber,
		},
		/// User updated `AccountId`
		UpdateAccountId {
			phone_verifier: T::AccountId,
			old_account_id: T::AccountId,
			new_account_id: T::AccountId,
		},
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(3,1).ref_time())]
		pub fn new_user(
			origin: OriginFor<T>,
			account_id: T::AccountId,
			name: T::Username,
			phone_number: T::PhoneNumber,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			ensure!(PhoneVerifiers::<T>::get().contains(&who), Error::<T>::NotAllowed);

			if PhoneNumberFor::<T>::contains_key(&phone_number) {
				// If such phone number exists migrate those account
				// balance, trait score, etc to this new account

				// Remove old account date
				// Save unwrap because of check above
				let old_account_id = PhoneNumberFor::<T>::take(&phone_number).unwrap();
				let old_identity_store = IdentityOf::<T>::take(&old_account_id).unwrap();
				NameFor::<T>::remove(&old_identity_store.name);

				// Save old nickname and new `AccountId`
				NameFor::<T>::insert(&old_identity_store.name, account_id.clone());
				PhoneNumberFor::<T>::insert(&phone_number, account_id.clone());
				IdentityOf::<T>::insert(
					&account_id,
					IdentityStore { name: old_identity_store.name, phone_number },
				);

				// No need to transfer trait score and reward info
				// because of they are indexed by `PhoneNumber`

				// Transfer balance
				let amount = T::Currency::free_balance(&old_account_id);
				T::Currency::transfer(
					&old_account_id,
					&account_id,
					amount,
					ExistenceRequirement::AllowDeath,
				)?;

				Self::deposit_event(Event::<T>::UpdateAccountId {
					phone_verifier: who,
					old_account_id,
					new_account_id: account_id,
				});

				return Ok(())
			}

			if IdentityOf::<T>::contains_key(&account_id) {
				return Err(Error::<T>::AlreadyRegistered.into())
			}

			if NameFor::<T>::contains_key(&name) {
				return Err(Error::<T>::UserNameTaken.into())
			}

			NameFor::<T>::insert(&name, account_id.clone());
			PhoneNumberFor::<T>::insert(&phone_number, account_id.clone());
			IdentityOf::<T>::insert(
				&account_id,
				IdentityStore { name: name.clone(), phone_number: phone_number.clone() },
			);

			T::Hooks::on_new_user(
				who.clone(),
				account_id.clone(),
				name.clone(),
				phone_number.clone(),
			)?;

			Self::deposit_event(Event::<T>::NewUser {
				phone_verifier: who,
				account_id,
				name,
				phone_number,
			});

			Ok(())
		}
	}
}

impl<T: Config> IdentityProvider<T::AccountId, T::Username, T::PhoneNumber> for Pallet<T> {
	fn exist_by_identity(
		account_identity: &AccountIdentity<T::AccountId, T::Username, T::PhoneNumber>,
	) -> bool {
		match account_identity {
			AccountIdentity::AccountId(account_id) => IdentityOf::<T>::get(account_id).is_some(),
			AccountIdentity::PhoneNumber(number) => PhoneNumberFor::<T>::get(number).is_some(),
			AccountIdentity::Name(name) => NameFor::<T>::get(name).is_some(),
		}
	}

	fn identity_by_id(
		account_id: &T::AccountId,
	) -> Option<IdentityInfo<T::AccountId, T::Username, T::PhoneNumber>> {
		<IdentityOf<T>>::get(account_id).map(|v| IdentityInfo {
			account_id: account_id.clone(),
			name: v.name,
			number: v.phone_number,
		})
	}

	fn identity_by_name(
		name: &T::Username,
	) -> Option<IdentityInfo<T::AccountId, T::Username, T::PhoneNumber>> {
		<NameFor<T>>::get(name).as_ref().and_then(Self::identity_by_id)
	}

	fn identity_by_number(
		number: &T::PhoneNumber,
	) -> Option<IdentityInfo<T::AccountId, T::Username, T::PhoneNumber>> {
		<PhoneNumberFor<T>>::get(number).as_ref().and_then(Self::identity_by_id)
	}
}

impl<T, UsernameLimit> Pallet<T>
where
	UsernameLimit: Get<u32> + 'static,
	T: Config<Username = BoundedVec<u8, UsernameLimit>>,
{
	/// Search for registered user who's username start with given `prefix`
	pub fn get_contacts(
		prefix: T::Username,
	) -> Vec<(T::AccountId, IdentityStore<T::Username, T::PhoneNumber>)> {
		IdentityOf::<T>::iter()
			.filter(|(_key, value)| value.name.starts_with(&prefix))
			.collect()
	}
}
