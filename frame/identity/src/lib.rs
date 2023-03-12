#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::{traits::Get, BoundedVec, CloneNoBound, PartialEqNoBound, RuntimeDebugNoBound};
use scale_info::TypeInfo;
use sp_std::{prelude::*, vec};

pub use pallet::*;

#[derive(
	CloneNoBound, Encode, Decode, Eq, MaxEncodedLen, PartialEqNoBound, RuntimeDebugNoBound, TypeInfo,
)]
#[codec(mel_bound())]
#[scale_info(skip_type_params(NameLimit, NumberLimit))]
pub struct IdentityStore<NameLimit: Get<u32>, NumberLimit: Get<u32>> {
	name: BoundedVec<u8, NameLimit>,
	number: BoundedVec<u8, NumberLimit>,
}

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// The overarching event type.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		/// Max length of name
		type NameLimit: Get<u32>;
		/// Max length of number
		type NumberLimit: Get<u32>;
		/// Max number of phone verifiers allowed
		type MaxPhoneVerifiers: Get<u32>;
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

	/// Information that is pertinent to identify the entity behind an account.
	///
	/// TWOX-NOTE: OK ― `AccountId` is a secure hash.
	#[pallet::storage]
	#[pallet::getter(fn identity)]
	pub(super) type IdentityOf<T: Config> =
		StorageMap<_, Blake2_128Concat, T::AccountId, IdentityStore<T::NameLimit, T::NumberLimit>>;

	#[pallet::storage]
	pub(super) type NameFor<T: Config> =
		StorageMap<_, Blake2_128Concat, BoundedVec<u8, T::NameLimit>, T::AccountId>;

	#[pallet::storage]
	pub(super) type NumberFor<T: Config> =
		StorageMap<_, Blake2_128Concat, BoundedVec<u8, T::NumberLimit>, T::AccountId>;

	#[pallet::storage]
	pub(super) type PhoneVerifiers<T: Config> =
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
	pub enum Event<T: Config> {}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(3,1).ref_time())]
		pub fn new_user(
			origin: OriginFor<T>,
			account_id: T::AccountId,
			name: BoundedVec<u8, T::NameLimit>,
			number: BoundedVec<u8, T::NumberLimit>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			ensure!(PhoneVerifiers::<T>::get().contains(&who), Error::<T>::NotAllowed);

			if <IdentityOf<T>>::contains_key(&account_id) {
				return Err(Error::<T>::AlreadyRegistered.into())
			}

			if <NameFor<T>>::contains_key(&name) {
				return Err(Error::<T>::UserNameTaken.into())
			}

			if <NumberFor<T>>::contains_key(&number) {
				return Err(Error::<T>::PhoneNumberTaken.into())
			}

			<NameFor<T>>::insert(&name, account_id.clone());
			<NumberFor<T>>::insert(&number, account_id.clone());
			<IdentityOf<T>>::insert(account_id, IdentityStore { name, number });

			Ok(())
		}
	}
}

pub struct IdentityInfo<AccountId, NameLimit: Get<u32>, NumberLimit: Get<u32>> {
	pub account_id: AccountId,
	pub name: BoundedVec<u8, NameLimit>,
	pub number: BoundedVec<u8, NumberLimit>,
}

pub trait IdentityProvider<AccountId, NameLimit: Get<u32>, NumberLimit: Get<u32>> {
	fn identity_by_id(
		account_id: AccountId,
	) -> Option<IdentityInfo<AccountId, NameLimit, NumberLimit>>;
	fn identity_by_name(
		name: BoundedVec<u8, NameLimit>,
	) -> Option<IdentityInfo<AccountId, NameLimit, NumberLimit>>;
	fn identity_by_number(
		number: BoundedVec<u8, NumberLimit>,
	) -> Option<IdentityInfo<AccountId, NameLimit, NumberLimit>>;
}

impl<T: Config> IdentityProvider<T::AccountId, T::NameLimit, T::NumberLimit> for Pallet<T> {
	fn identity_by_id(
		account_id: T::AccountId,
	) -> Option<IdentityInfo<T::AccountId, T::NameLimit, T::NumberLimit>> {
		<IdentityOf<T>>::get(&account_id).map(|v| IdentityInfo {
			account_id,
			name: v.name,
			number: v.number,
		})
	}

	fn identity_by_name(
		name: BoundedVec<u8, T::NameLimit>,
	) -> Option<IdentityInfo<T::AccountId, T::NameLimit, T::NumberLimit>> {
		<NameFor<T>>::get(&name).and_then(Self::identity_by_id)
	}

	fn identity_by_number(
		number: BoundedVec<u8, T::NumberLimit>,
	) -> Option<IdentityInfo<T::AccountId, T::NameLimit, T::NumberLimit>> {
		<NumberFor<T>>::get(&number).and_then(Self::identity_by_id)
	}
}
