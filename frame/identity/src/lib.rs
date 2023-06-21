#![cfg_attr(not(feature = "std"), no_std)]

pub mod types;

use crate::types::{IdentityStore, VerificationResult};
use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::{
	pallet_prelude::DispatchResult,
	traits::{Currency, ExistenceRequirement, Get},
	BoundedVec,
};
pub use pallet::*;
use sp_common::{
	hooks::Hooks,
	identity::{AccountIdentity, IdentityInfo},
	traits::IdentityProvider,
	BoundedString,
};
use sp_rpc::VerificationEvidence;
use sp_runtime::traits::{IdentifyAccount, Verify, Zero};
use sp_std::{prelude::*, vec};

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::{pallet_prelude::*, traits::Currency};
	use frame_system::pallet_prelude::*;
	use sp_common::hooks::Hooks;
	use sp_std::fmt::Debug;

	#[pallet::config]
	pub trait Config:
		frame_system::Config + pallet_balances::Config + pallet_timestamp::Config
	{
		/// The overarching event type.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		/// Username type
		type Username: Parameter + Member + MaybeSerializeDeserialize + Debug + Ord + MaxEncodedLen;
		/// Phone number hash type
		type PhoneNumberHash: Parameter
			+ Member
			+ MaybeSerializeDeserialize
			+ Debug
			+ Ord
			+ MaxEncodedLen;
		/// Max number of phone verifiers allowed
		type MaxPhoneVerifiers: Get<u32>;
		/// Handler for when a new user has just been registered
		type Hooks: Hooks<Self::AccountId, Self::Balance, Self::Username, Self::PhoneNumberHash>;
		/// The currency mechanism.
		type Currency: Currency<Self::AccountId, Balance = Self::Balance>;

		/// A Signature can be verified with a specific `PublicKey`.
		/// The additional traits are boilerplate.
		type Signature: Verify<Signer = Self::PublicKey> + Encode + Decode + Parameter;

		/// A PublicKey can be converted into an `AccountId`. This is required by the
		/// `Signature` type.
		/// The additional traits are boilerplate.
		type PublicKey: IdentifyAccount<AccountId = Self::PublicKey>
			+ Encode
			+ Decode
			+ Parameter
			+ Into<Self::AccountId>;
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
		IdentityStore<T::Username, T::PhoneNumberHash, T::Moment>,
	>;

	#[pallet::storage]
	pub type UsernameFor<T: Config> = StorageMap<_, Blake2_128Concat, T::Username, T::AccountId>;

	#[pallet::storage]
	pub type PhoneNumberFor<T: Config> =
		StorageMap<_, Blake2_128Concat, T::PhoneNumberHash, T::AccountId>;

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
		/// Passed `PublicKey` do not belong to any `PhoneVerifier`
		NotVerifier,
		/// Signature `AccountId` do not match `AccountId` from params
		AccountIdMismatch,
		/// Signature do not match to passed parameters
		InvalidSignature,
		/// Provided `Username` or `PhoneNumber` to update user data must be different from
		/// existed one. Or missed parameters for update
		InvalidArguments,
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// New user registered
		NewUser {
			phone_verifier: T::AccountId,
			account_id: T::AccountId,
			username: T::Username,
			phone_number_hash: T::PhoneNumberHash,
		},
		/// User change its `AccountId`
		AccountMigrated {
			phone_verifier: T::AccountId,
			old_account_id: T::AccountId,
			new_account_id: T::AccountId,
		},
		/// User change its `Username` and/or `PhoneNumber`
		AccountUpdated {
			account_id: T::AccountId,
			username: T::Username,
			new_username: Option<T::Username>,
			phone_number_hash: T::PhoneNumberHash,
			new_phone_number_hash: Option<T::PhoneNumberHash>,
		},
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(3, 1).ref_time())]
		pub fn new_user(
			origin: OriginFor<T>,
			// verifier_public_key: T::PublicKey,
			// verifier_signature: T::Signature,
			account_id: T::AccountId,
			username: T::Username,
			phone_number_hash: T::PhoneNumberHash,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			ensure!(who == account_id, Error::<T>::AccountIdMismatch);

			// let verifier_account_id = verifier_public_key.clone().into();
			// Check verification
			// ensure!(
			// 	PhoneVerifiers::<T>::get().contains(&verifier_account_id),
			// 	Error::<T>::NotVerifier
			// );
			// ensure!(
			// 	Self::verify_signature(
			// 		verifier_public_key,
			// 		verifier_signature,
			// 		account_id.clone(),
			// 		username.clone(),
			// 		phone_number_hash.clone()
			// 	),
			// 	Error::<T>::InvalidSignature
			// );

			let verifier_account_id =
				PhoneVerifiers::<T>::get().pop().ok_or(Error::<T>::NotVerifier)?;

			match Self::verify(&account_id, &username, &phone_number_hash) {
				VerificationResult::Valid => Self::register_user(
					verifier_account_id,
					account_id,
					username,
					phone_number_hash,
				),
				VerificationResult::Migration =>
					Self::migrate_user(verifier_account_id, account_id, phone_number_hash),
				VerificationResult::AccountIdExists => Err(Error::<T>::AlreadyRegistered.into()),
				VerificationResult::UsernameExists => Err(Error::<T>::UserNameTaken.into()),
			}
		}

		#[pallet::call_index(1)]
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(3, 1).ref_time())]
		pub fn update_user(
			origin: OriginFor<T>,
			username: Option<T::Username>,
			phone_number_hash: Option<T::PhoneNumberHash>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			ensure!(
				username.is_some() || phone_number_hash.is_some(),
				Error::<T>::InvalidArguments
			);
			ensure!(IdentityOf::<T>::contains_key(&who), Error::<T>::NotFound);
			// Safety: because of check above unwrap do not panics
			let mut identity = IdentityOf::<T>::get(&who).unwrap();

			if let Some(username) = username.clone() {
				ensure!(identity.username != username, Error::<T>::InvalidArguments);
				// Check username for uniqueness
				ensure!(!UsernameFor::<T>::contains_key(&username), Error::<T>::UserNameTaken);
				// Remove old `Username` <-> `AccountId` relation
				UsernameFor::<T>::remove(identity.username);
				UsernameFor::<T>::insert(&username, &who);
				// Set new username
				identity.username = username;
			}

			if let Some(phone_number_hash) = phone_number_hash.clone() {
				ensure!(
					identity.phone_number_hash != phone_number_hash,
					Error::<T>::InvalidArguments
				);
				// Check phone number for uniqueness
				ensure!(
					!PhoneNumberFor::<T>::contains_key(&phone_number_hash),
					Error::<T>::PhoneNumberTaken
				);
				// Remove old `PhoneNumber` <-> `AccountId` relation
				PhoneNumberFor::<T>::remove(identity.phone_number_hash);
				PhoneNumberFor::<T>::insert(&phone_number_hash, &who);
				// Set new phone number
				identity.phone_number_hash = phone_number_hash;
			}

			// Save identity changes
			IdentityOf::<T>::insert(&who, &identity);

			// TODO: call hook

			Self::deposit_event(Event::<T>::AccountUpdated {
				account_id: who,
				username: identity.username,
				new_username: username,
				phone_number_hash: identity.phone_number_hash,
				new_phone_number_hash: phone_number_hash,
			});

			Ok(())
		}

		#[pallet::call_index(2)]
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(3, 1).ref_time())]
		pub fn delete_user(origin: OriginFor<T>) -> DispatchResult {
			let who = ensure_signed(origin)?;

			let identity_info = IdentityOf::<T>::take(&who).ok_or(Error::<T>::NotFound)?;
			UsernameFor::<T>::remove(&identity_info.username);
			PhoneNumberFor::<T>::remove(&identity_info.phone_number_hash);

			T::Currency::make_free_balance_be(&who, T::Balance::zero());

			T::Hooks::on_delete_user(who, identity_info.username, identity_info.phone_number_hash)?;

			Ok(())
		}
	}
}

impl<T: Config> IdentityProvider<T::AccountId, T::Username, T::PhoneNumberHash> for Pallet<T> {
	fn exist_by_identity(
		account_identity: &AccountIdentity<T::AccountId, T::Username, T::PhoneNumberHash>,
	) -> bool {
		match account_identity {
			AccountIdentity::AccountId(account_id) => IdentityOf::<T>::get(account_id).is_some(),
			AccountIdentity::PhoneNumberHash(phone_number_hash) =>
				PhoneNumberFor::<T>::get(phone_number_hash).is_some(),
			AccountIdentity::Username(username) => UsernameFor::<T>::get(username).is_some(),
		}
	}

	fn identity_by_id(
		account_id: &T::AccountId,
	) -> Option<IdentityInfo<T::AccountId, T::Username, T::PhoneNumberHash>> {
		<IdentityOf<T>>::get(account_id).map(|v| IdentityInfo {
			account_id: account_id.clone(),
			username: v.username,
			phone_number_hash: v.phone_number_hash,
		})
	}

	fn identity_by_name(
		username: &T::Username,
	) -> Option<IdentityInfo<T::AccountId, T::Username, T::PhoneNumberHash>> {
		<UsernameFor<T>>::get(username).as_ref().and_then(Self::identity_by_id)
	}

	fn identity_by_number(
		phone_number_hash: &T::PhoneNumberHash,
	) -> Option<IdentityInfo<T::AccountId, T::Username, T::PhoneNumberHash>> {
		<PhoneNumberFor<T>>::get(phone_number_hash)
			.as_ref()
			.and_then(Self::identity_by_id)
	}
}

impl<T: Config> Pallet<T> {
	/// Set user registration time
	pub fn set_registration_time(account_id: &T::AccountId, time: T::Moment) -> bool {
		IdentityOf::<T>::mutate(account_id, |query| {
			query.as_mut().map(|identity| identity.registration_time = Some(time)).is_some()
		})
	}

	pub fn get_registration_time(account_id: &T::AccountId) -> Option<T::Moment> {
		IdentityOf::<T>::get(account_id).and_then(|identity| identity.registration_time)
	}

	/// Perform validation for input parameters of `new_user` tx
	pub fn verify(
		account_id: &T::AccountId,
		username: &T::Username,
		phone_number_hash: &T::PhoneNumberHash,
	) -> VerificationResult {
		// If such phone number hash exists migrate those account
		// balance, trait score, etc to this new account
		if PhoneNumberFor::<T>::contains_key(phone_number_hash) {
			return VerificationResult::Migration
		}

		// Such `AccountId` registered by other account
		if IdentityOf::<T>::contains_key(account_id) {
			return VerificationResult::AccountIdExists
		}

		// Such `Username` registered by other account
		if UsernameFor::<T>::contains_key(username) {
			return VerificationResult::UsernameExists
		}

		VerificationResult::Valid
	}

	/// Checks that signature match passed data
	///
	/// # Returns
	/// `true` - if signature match passed data
	/// `false` - otherwise
	pub fn verify_signature(
		verifier_public_key: T::PublicKey,
		verifier_signature: T::Signature,
		account_id: T::AccountId,
		username: T::Username,
		phone_number_hash: T::PhoneNumberHash,
	) -> bool {
		let data = VerificationEvidence {
			verifier_public_key: verifier_public_key.clone(),
			account_id,
			username,
			phone_number_hash,
		}
		.encode();

		verifier_signature.verify(&*data, &verifier_public_key)
	}

	/// Add information about new user into storage, call `on_new_user` hook and deposit event
	pub(crate) fn register_user(
		phone_verifier: T::AccountId,
		account_id: T::AccountId,
		username: T::Username,
		phone_number_hash: T::PhoneNumberHash,
	) -> DispatchResult {
		UsernameFor::<T>::insert(&username, account_id.clone());
		PhoneNumberFor::<T>::insert(&phone_number_hash, account_id.clone());
		IdentityOf::<T>::insert(
			&account_id,
			IdentityStore {
				username: username.clone(),
				phone_number_hash: phone_number_hash.clone(),
				registration_time: None,
			},
		);

		T::Hooks::on_new_user(
			phone_verifier.clone(),
			account_id.clone(),
			username.clone(),
			phone_number_hash.clone(),
		)?;

		Self::deposit_event(Event::<T>::NewUser {
			phone_verifier,
			account_id,
			username,
			phone_number_hash,
		});

		Ok(())
	}

	/// Migrate user data to new `AccountId` and deposit event
	pub(crate) fn migrate_user(
		phone_verifier: T::AccountId,
		new_account_id: T::AccountId,
		phone_number_hash: T::PhoneNumberHash,
	) -> DispatchResult {
		// If such phone number exists migrate those account
		// balance, trait score, etc to this new account

		// Remove old account date
		// Save unwrap because of check above
		let old_account_id = PhoneNumberFor::<T>::take(&phone_number_hash).unwrap();
		let identity = IdentityOf::<T>::take(&old_account_id).unwrap();
		UsernameFor::<T>::remove(&identity.username);

		// Save old nickname and new `AccountId`
		UsernameFor::<T>::insert(&identity.username, new_account_id.clone());
		PhoneNumberFor::<T>::insert(&phone_number_hash, new_account_id.clone());
		IdentityOf::<T>::insert(
			&new_account_id,
			IdentityStore {
				username: identity.username.clone(),
				phone_number_hash,
				registration_time: identity.registration_time,
			},
		);

		// Transfer balance
		let amount = T::Currency::free_balance(&old_account_id);
		T::Currency::transfer(
			&old_account_id,
			&new_account_id,
			amount,
			ExistenceRequirement::AllowDeath,
		)?;

		// Appreciation pallet will migrate traits score and communities membership
		// Transaction indexing pallet will migrate transactions
		T::Hooks::on_update_user(
			old_account_id.clone(),
			Some(new_account_id.clone()),
			identity.username,
			None,
			identity.phone_number_hash,
			None,
		)?;

		Self::deposit_event(Event::<T>::AccountMigrated {
			phone_verifier,
			old_account_id,
			new_account_id,
		});

		Ok(())
	}
}

impl<T, UsernameLimit> Pallet<T>
where
	UsernameLimit: Get<u32> + 'static,
	T: Config<Username = BoundedString<UsernameLimit>>,
{
	/// Search for registered user who's username start with given `prefix`
	pub fn get_contacts(
		prefix: T::Username,
	) -> Vec<(T::AccountId, IdentityStore<T::Username, T::PhoneNumberHash, T::Moment>)> {
		IdentityOf::<T>::iter()
			.filter(|(_key, value)| value.username.0.starts_with(&prefix.0))
			.collect()
	}
}
