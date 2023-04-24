#![cfg_attr(not(feature = "std"), no_std)]
use frame_support::{
	pallet_prelude::*,
	traits::{Currency, ExistenceRequirement, Get},
	BoundedVec,
};
use sp_common::{hooks::Hooks, identity::AccountIdentity, traits::IdentityProvider};

mod types;

pub use crate::types::*;
pub use pallet::*;
use sp_common::types::{CharTraitId, CommunityId};

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_system::pallet_prelude::*;
	use sp_common::{
		types::{CharTraitId, CommunityId},
		*,
	};

	#[pallet::config]
	pub trait Config:
		frame_system::Config + pallet_balances::Config + pallet_identity::Config
	{
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		/// Max number of `CharTrait`
		type MaxCharTrait: Get<u32>;
		/// Max length of `CharTrait`'s name
		type CharNameLimit: Get<u32>;
		/// Max number of `Communities`
		type MaxCommunities: Get<u32>;
		/// Max length of `Community`'s name
		type CommunityNameLimit: Get<u32>;
		/// Max length of `Community`'s description
		type CommunityDescLimit: Get<u32>;
		/// Max length of emoji
		type EmojiLimit: Get<u32>;
		/// Max length of `Community`'s urls
		type CommunityUrlLimit: Get<u32>;
		/// The currency mechanism.
		type Currency: Currency<Self::AccountId, Balance = Self::Balance>;

		type IdentityProvider: IdentityProvider<Self::AccountId, Self::Username, Self::PhoneNumber>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config> {
		pub char_traits: Vec<(CharTraitId, String, String)>,
		pub no_char_trait_id: CharTraitId,
		pub signup_char_trait_id: CharTraitId,
		pub spender_char_trait_id: CharTraitId,
		pub ambassador_char_trait_id: CharTraitId,

		pub communities: Vec<GenesisCommunity>,
		pub community_membership: Vec<(T::PhoneNumber, CommunityId, CommunityRole)>,
		pub no_community_id: CommunityId,
	}

	#[cfg(feature = "std")]
	impl<T: Config> Default for GenesisConfig<T> {
		fn default() -> Self {
			Self {
				char_traits: vec![],
				no_char_trait_id: 0,
				signup_char_trait_id: 1,
				spender_char_trait_id: 2,
				ambassador_char_trait_id: 41,
				communities: vec![],
				community_membership: vec![],
				no_community_id: 0,
			}
		}
	}

	#[allow(clippy::type_complexity)]
	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
		fn build(&self) {
			let bounded_char_traits: BoundedVec<
				CharTrait<T::CharNameLimit, T::EmojiLimit>,
				T::MaxCharTrait,
			> = self
				.char_traits
				.clone()
				.into_iter()
				.map(|(id, name, emoji)| CharTrait {
					id,
					name: name.try_into().expect(
						"Max length of character trait name should be lower than T::CharNameLimit",
					),
					emoji: emoji.try_into().expect(
						"Max length of character trait name should be lower than T::EmojiLimit",
					),
				})
				.collect::<Vec<_>>()
				.try_into()
				.expect("Initial number of char_traits should be lower than T::MaxCharTrait");
			CharTraits::<T>::put(bounded_char_traits);

			NoCharTraitId::<T>::put(self.no_char_trait_id);
			SignupCharTraitId::<T>::put(self.signup_char_trait_id);
			SpenderCharTraitId::<T>::put(self.spender_char_trait_id);
			AmbassadorCharTraitId::<T>::put(self.ambassador_char_trait_id);

			let bounded_communities: BoundedVec<
				Community<
					T::CommunityNameLimit,
					T::CommunityDescLimit,
					T::EmojiLimit,
					T::CommunityUrlLimit,
					T::MaxCharTrait,
				>,
				T::MaxCommunities,
			> = self
				.communities
				.clone()
				.into_iter()
				.map(|(id, name, desc, emoji, website_url, twitter_url, insta_url, face_url, discord_url, char_traits, closed)| {
					Community {
						id,
						name: name.try_into().expect("Max length of community name should be lower than T::CommunityNameLimit"),
						desc: desc.try_into().expect("Max length of community desc should be lower than T::CommunityDescLimit"),
						emoji: emoji.try_into().expect("Max length of community emoji should be lower than T::CommunityEmojiLimit"),
						website_url: website_url.try_into().expect("Max length of community website url should be lower than T::CommunityUrlLimit"),
						twitter_url: twitter_url.try_into().expect("Max length of community twitter url should be lower than T::CommunityUrlLimit"),
						insta_url: insta_url.try_into().expect("Max length of community insta url should be lower than T::CommunityUrlLimit"),
						face_url: face_url.try_into().expect("Max length of community face url should be lower than T::CommunityUrlLimit"),
						discord_url: discord_url.try_into().expect("Max length of community discord url should be lower than T::CommunityUrlLimit"),
						char_traits: char_traits.try_into().expect("Max length of community character traits should be lower that T::MaxCharTrait"),
						closed,
					}
				})
				.collect::<Vec<_>>()
				.try_into()
				.expect("Initial number of communities should be lower than T::MaxCommunities");
			Communities::<T>::put(bounded_communities);

			self.community_membership.iter().for_each(|(phone_number, community_id, role)| {
				CommunityMembership::<T>::insert(phone_number, community_id, role);
			});

			NoCommunityId::<T>::put(self.no_community_id);
		}
	}

	#[cfg(feature = "std")]
	impl<T: Config> GenesisConfig<T> {
		pub fn build_storage(&self) -> Result<sp_runtime::Storage, std::string::String> {
			<Self as GenesisBuild<T>>::build_storage(self)
		}
	}

	#[pallet::storage]
	#[pallet::getter(fn no_char_trait_id)]
	pub type NoCharTraitId<T: Config> =
		StorageValue<_, CharTraitId, ResultQuery<Error<T>::NonExistentStorageValue>>;

	/// This is the signup trait - user gets it for signing up
	#[pallet::storage]
	pub type SignupCharTraitId<T: Config> =
		StorageValue<_, CharTraitId, ResultQuery<Error<T>::NonExistentStorageValue>>;

	/// User gets a point in this trait for each sent appreciation / payment
	#[pallet::storage]
	pub type SpenderCharTraitId<T: Config> =
		StorageValue<_, CharTraitId, ResultQuery<Error<T>::NonExistentStorageValue>>;

	/// User gets one for each referral who signed up
	#[pallet::storage]
	pub type AmbassadorCharTraitId<T: Config> =
		StorageValue<_, CharTraitId, ResultQuery<Error<T>::NonExistentStorageValue>>;

	#[pallet::storage]
	#[pallet::getter(fn char_traits)]
	pub type CharTraits<T: Config> = StorageValue<
		_,
		BoundedVec<CharTrait<T::CharNameLimit, T::EmojiLimit>, T::MaxCharTrait>,
		ValueQuery,
	>;

	#[pallet::storage]
	pub type NoCommunityId<T: Config> =
		StorageValue<_, CharTraitId, ResultQuery<Error<T>::NonExistentStorageValue>>;

	#[pallet::storage]
	pub type Communities<T: Config> = StorageValue<
		_,
		BoundedVec<
			Community<
				T::CommunityNameLimit,
				T::CommunityDescLimit,
				T::EmojiLimit,
				T::CommunityUrlLimit,
				T::MaxCharTrait,
			>,
			T::MaxCommunities,
		>,
		ValueQuery,
	>;

	#[pallet::storage]
	pub type CommunityMembership<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		T::PhoneNumber,
		Blake2_128Concat,
		CommunityId,
		CommunityRole,
		OptionQuery,
	>;

	#[pallet::storage]
	pub type TraitScores<T: Config> = StorageNMap<
		_,
		(
			NMapKey<Blake2_128Concat, T::PhoneNumber>,
			NMapKey<Blake2_128Concat, CommunityId>,
			NMapKey<Blake2_128Concat, CharTraitId>,
		),
		Score,
		OptionQuery,
	>;

	#[pallet::storage]
	pub type Referrals<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		T::PhoneNumber,
		Blake2_128Concat,
		AccountIdentity<T::AccountId, T::Username, T::PhoneNumber>,
		(),
	>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		NewCommunityAdmin {
			community_id: CommunityId,
			community_name: BoundedVec<u8, T::CommunityNameLimit>,
			account_id: T::AccountId,
			username: T::Username,
			phone_number: T::PhoneNumber,
		},
	}

	#[pallet::error]
	pub enum Error<T> {
		/// Missing required storage value, practically impossible
		/// if happened mean that one of the storages were not configured
		/// throw chain spec file
		NonExistentStorageValue,
		/// Account didn't found.
		NotFound,
		/// No such character trait
		CharTraitNotFound,
		/// No such community
		CommunityNotFound,
		/// Payer non a member of the community
		NotMember,
		/// Closed community - only community admin can invite new members
		/// and only members can appreciate each other in the community
		CommunityClosed,
		///
		NotEnoughPermission,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn appreciation(
			origin: OriginFor<T>,
			to: AccountIdentity<T::AccountId, T::Username, T::PhoneNumber>,
			amount: T::Balance,
			community_id: Option<CommunityId>,
			char_trait_id: Option<CharTraitId>,
		) -> DispatchResult {
			let payer = ensure_signed(origin)?;
			let payer_phone_number =
				T::IdentityProvider::identity_by_id(&payer).ok_or(Error::<T>::NotFound)?.number;
			let referral = Referrals::<T>::take(&payer_phone_number, &to).is_some();
			let payee = Self::get_account_id(to).ok_or(Error::<T>::NotFound)?;
			let community_id = community_id.unwrap_or(NoCommunityId::<T>::get()?);
			let char_trait_id = char_trait_id.unwrap_or(NoCharTraitId::<T>::get()?);

			Self::process_appreciation(&payer, &payee, community_id, char_trait_id, referral)?;

			T::Currency::transfer(&payer, &payee, amount, ExistenceRequirement::KeepAlive)?;

			T::Hooks::on_appreciation(payer, payee, amount, community_id, char_trait_id)?;
			// TODO: events

			Ok(())
		}

		#[pallet::call_index(1)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn set_admin(
			origin: OriginFor<T>,
			community_id: CommunityId,
			new_admin: AccountIdentity<T::AccountId, T::Username, T::PhoneNumber>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let phone_number =
				T::IdentityProvider::identity_by_id(&who).ok_or(Error::<T>::NotFound)?.number;
			ensure!(
				matches!(
					CommunityMembership::<T>::get(&phone_number, community_id),
					Some(CommunityRole::Admin)
				),
				Error::<T>::NotEnoughPermission
			);

			let community = Communities::<T>::get()
				.into_iter()
				.find(|community| community.id == community_id)
				.ok_or(Error::<T>::NotFound)?;
			let new_admin_identity =
				T::IdentityProvider::get_identity_info(&new_admin).ok_or(Error::<T>::NotFound)?;
			CommunityMembership::<T>::insert(
				&new_admin_identity.number,
				community_id,
				CommunityRole::Admin,
			);

			T::Hooks::on_set_admin(who, new_admin_identity.account_id.clone())?;

			Self::deposit_event(Event::<T>::NewCommunityAdmin {
				community_id: community.id,
				community_name: community.name,
				account_id: new_admin_identity.account_id,
				username: new_admin_identity.name,
				phone_number: new_admin_identity.number,
			});

			Ok(())
		}
	}
}

impl<T: pallet::Config> Pallet<T> {
	fn get_account_id(
		to: AccountIdentity<T::AccountId, T::Username, T::PhoneNumber>,
	) -> Option<T::AccountId> {
		match to {
			AccountIdentity::AccountId(account_id) => Some(account_id),
			AccountIdentity::PhoneNumber(number) =>
				T::IdentityProvider::identity_by_number(&number).map(|v| v.account_id),
			AccountIdentity::Name(name) =>
				T::IdentityProvider::identity_by_name(&name).map(|v| v.account_id),
		}
	}

	pub fn increment_trait_score(
		phone_number: &T::PhoneNumber,
		community_id: CommunityId,
		char_trait_id: CharTraitId,
	) {
		TraitScores::<T>::mutate((phone_number, community_id, char_trait_id), |value| {
			*value = Some(value.unwrap_or_default() + 1)
		});
	}

	pub fn process_appreciation(
		payer: &T::AccountId,
		payee: &T::AccountId,
		community_id: CommunityId,
		char_trait_id: CharTraitId,
		referral: bool,
	) -> DispatchResult {
		if NoCharTraitId::<T>::get()? == char_trait_id {
			return Ok(())
		}

		let payer = T::IdentityProvider::identity_by_id(payer).ok_or(Error::<T>::NotFound)?.number;
		let payee = T::IdentityProvider::identity_by_id(payee).ok_or(Error::<T>::NotFound)?.number;

		// TODO: whether to check `char_trait_id` for existence?

		// TODO: if this transfer lead to user signup set `true`
		if referral {
			// Give payer karma points for helping to grow the network
			Self::increment_trait_score(
				&payer,
				NoCommunityId::<T>::get()?,
				AmbassadorCharTraitId::<T>::get()?,
			);
		}

		// Standard appreciation w/o a community context
		if NoCommunityId::<T>::get()? == community_id {
			Self::increment_trait_score(&payer, community_id, SpenderCharTraitId::<T>::get()?);
			Self::increment_trait_score(&payee, community_id, char_trait_id);
			return Ok(())
		}

		let community = Communities::<T>::get()
			.into_iter()
			.find(|v| v.id == community_id)
			.ok_or(Error::<T>::CommunityNotFound)?;

		ensure!(community.char_traits.contains(&char_trait_id), Error::<T>::CharTraitNotFound,);

		let is_community_closed = community.closed;

		let payer_membership =
			CommunityMembership::<T>::get(&payer, community_id).unwrap_or_default();
		let payee_membership =
			CommunityMembership::<T>::get(&payee, community_id).unwrap_or_default();

		match (payer_membership, payee_membership) {
			(CommunityRole::None, _) => return Err(Error::<T>::NotMember.into()),
			(_, CommunityRole::Admin) | (_, CommunityRole::Member) => {
				Self::increment_trait_score(&payer, community_id, SpenderCharTraitId::<T>::get()?);
				Self::increment_trait_score(&payee, community_id, char_trait_id);
			},
			(CommunityRole::Admin, CommunityRole::None) => {
				Self::increment_trait_score(&payer, community_id, SpenderCharTraitId::<T>::get()?);
				Self::increment_trait_score(
					&payer,
					community_id,
					AmbassadorCharTraitId::<T>::get()?,
				);
				Self::increment_trait_score(&payee, community_id, char_trait_id);
				CommunityMembership::<T>::insert(&payee, community_id, CommunityRole::Member);
			},
			(CommunityRole::Member, CommunityRole::None) if !is_community_closed => {
				Self::increment_trait_score(&payer, community_id, SpenderCharTraitId::<T>::get()?);
				Self::increment_trait_score(
					&payer,
					community_id,
					AmbassadorCharTraitId::<T>::get()?,
				);
				Self::increment_trait_score(&payee, community_id, char_trait_id);
				CommunityMembership::<T>::insert(&payee, community_id, CommunityRole::Member);
			},
			(CommunityRole::Member, CommunityRole::None) =>
				return Err(Error::<T>::CommunityClosed.into()),
		}

		Ok(())
	}

	pub fn trait_scores_of(
		account_id: &T::AccountId,
	) -> scale_info::prelude::vec::Vec<(CommunityId, CharTraitId, Score)> {
		let no_community_id = NoCommunityId::<T>::get().unwrap();
		T::IdentityProvider::identity_by_id(account_id)
			.map(|identity_info| {
				CommunityMembership::<T>::iter_prefix(&identity_info.number)
					.map(|(community_id, _)| community_id)
					.chain([no_community_id])
					.flat_map(|community_id| {
						TraitScores::<T>::iter_prefix((&identity_info.number, community_id))
							.map(move |(char_trait_id, score)| (community_id, char_trait_id, score))
					})
					.collect()
			})
			.unwrap_or_default()
	}

	pub fn community_membership_of(
		account_id: &T::AccountId,
	) -> scale_info::prelude::vec::Vec<(CommunityId, Score, bool)> {
		T::IdentityProvider::identity_by_id(account_id)
			.map(|identity_info| {
				CommunityMembership::<T>::iter_prefix(&identity_info.number)
					.map(|(community_id, role)| {
						let score =
							TraitScores::<T>::iter_prefix((&identity_info.number, community_id))
								.map(|(_, score)| score)
								.sum::<u32>();
						let is_admin = role.is_admin();

						(community_id, score, is_admin)
					})
					.collect()
			})
			.unwrap_or_default()
	}
}

impl<T: Config> Hooks<T::AccountId, T::Balance, T::Username, T::PhoneNumber> for Pallet<T> {
	fn on_new_user(
		_verifier: T::AccountId,
		who: T::AccountId,
		_name: T::Username,
		_phone_number: T::PhoneNumber,
	) -> DispatchResult {
		let no_community_id = NoCommunityId::<T>::get()?;
		let signup_char_trait_id = SignupCharTraitId::<T>::get()?;

		let who = T::IdentityProvider::identity_by_id(&who).ok_or(Error::<T>::NotFound)?.number;

		Self::increment_trait_score(&who, no_community_id, signup_char_trait_id);

		Ok(())
	}
}
