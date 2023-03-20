#![cfg_attr(not(feature = "std"), no_std)]
use frame_support::{
	pallet_prelude::*,
	traits::{Currency, ExistenceRequirement, Get},
	BoundedVec,
};
use pallet_identity::IdentityProvider;

mod types;

pub use crate::types::*;
pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_system::pallet_prelude::*;

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
		/// Max length of `Community`'s emoji
		type CommunityEmojiLimit: Get<u32>;
		/// Max length of `Community`'s urls
		type CommunityUrlLimit: Get<u32>;
		/// The currency mechanism.
		type Currency: Currency<Self::AccountId, Balance = Self::Balance>;

		type IdentityProvider: IdentityProvider<Self::AccountId, Self::NameLimit, Self::NumberLimit>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config> {
		pub char_traits: Vec<(CharTraitId, Vec<u8>)>,
		pub no_char_trait_id: CharTraitId,
		pub signup_char_trait_id: CharTraitId,
		pub spender_char_trait_id: CharTraitId,
		pub ambassador_char_trait_id: CharTraitId,

		pub communities: Vec<(
			CommunityId,
			Vec<u8>,
			Vec<u8>,
			Vec<u8>,
			Vec<u8>,
			Vec<u8>,
			Vec<u8>,
			Vec<u8>,
			Vec<u8>,
			bool,
		)>,
		pub community_membership: Vec<(T::AccountId, CommunityId, CommunityRole)>,
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

	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
		fn build(&self) {
			let bounded_char_traits: BoundedVec<CharTrait<T::CharNameLimit>, T::MaxCharTrait> =
				self.char_traits
					.clone()
					.into_iter()
					.flat_map(|(id, name)| name.try_into().map(|name| CharTrait { id, name }))
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
					T::CommunityEmojiLimit,
					T::CommunityUrlLimit,
				>,
				T::MaxCommunities,
			> = self
				.communities
				.clone()
				.into_iter()
				.map(|(id, name, desc, emoji, website_url, twitter_url, insta_url, face_url, discord_url, closed)| {
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
						closed,
					}
				})
				.collect::<Vec<_>>()
				.try_into()
				.expect("Initial number of communities should be lower than T::MaxCommunities");
			Communities::<T>::put(bounded_communities);

			self.community_membership.iter().for_each(|(account_id, community_id, role)| {
				CommunityMembership::<T>::insert(account_id, community_id, role);
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
	pub(super) type NoCharTraitId<T: Config> =
		StorageValue<_, CharTraitId, ResultQuery<Error<T>::NonExistentStorageValue>>;

	/// This is the signup trait - user gets it for signing up
	#[pallet::storage]
	pub(super) type SignupCharTraitId<T: Config> =
		StorageValue<_, CharTraitId, ResultQuery<Error<T>::NonExistentStorageValue>>;

	/// User gets a point in this trait for each sent appreciation / payment
	#[pallet::storage]
	pub(super) type SpenderCharTraitId<T: Config> =
		StorageValue<_, CharTraitId, ResultQuery<Error<T>::NonExistentStorageValue>>;

	/// User gets one for each referral who signed up
	#[pallet::storage]
	pub(super) type AmbassadorCharTraitId<T: Config> =
		StorageValue<_, CharTraitId, ResultQuery<Error<T>::NonExistentStorageValue>>;

	#[pallet::storage]
	pub(super) type CharTraits<T: Config> =
		StorageValue<_, BoundedVec<CharTrait<T::CharNameLimit>, T::MaxCharTrait>, ValueQuery>;

	#[pallet::storage]
	pub(super) type NoCommunityId<T: Config> =
		StorageValue<_, CharTraitId, ResultQuery<Error<T>::NonExistentStorageValue>>;

	#[pallet::storage]
	pub(super) type Communities<T: Config> = StorageValue<
		_,
		BoundedVec<
			Community<
				T::CommunityNameLimit,
				T::CommunityDescLimit,
				T::CommunityEmojiLimit,
				T::CommunityUrlLimit,
			>,
			T::MaxCommunities,
		>,
		ValueQuery,
	>;

	#[pallet::storage]
	pub(super) type CommunityMembership<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		T::AccountId,
		Blake2_128Concat,
		CommunityId,
		CommunityRole,
		OptionQuery,
	>;

	#[pallet::storage]
	pub(super) type TraitScores<T: Config> = StorageNMap<
		_,
		(
			NMapKey<Blake2_128Concat, T::AccountId>,
			NMapKey<Blake2_128Concat, CommunityId>,
			NMapKey<Blake2_128Concat, CharTraitId>,
		),
		Score,
		OptionQuery,
	>;

	#[pallet::event]
	pub enum Event<T: Config> {}

	#[pallet::error]
	pub enum Error<T> {
		/// Missing required storage value, practically impossible
		/// if happened mean that one of the storages were not configured
		/// throw chain spec file
		NonExistentStorageValue,
		/// Account didn't found.
		NotFound,
		/// No such community
		CommunityNotFound,
		/// Payer non a member of the community
		NotMember,
		/// Closed community - only community admin can invite new members
		/// and only members can appreciate each other in the community
		CommunityClosed,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn appreciation(
			origin: OriginFor<T>,
			to: BoundedVec<u8, T::NumberLimit>,
			amount: T::Balance,
			community_id: Option<CommunityId>,
			char_trait_id: Option<CharTraitId>,
		) -> DispatchResult {
			let payer = ensure_signed(origin)?;
			let payee = T::IdentityProvider::identity_by_number(to)
				.ok_or(Error::<T>::NotFound)?
				.account_id;
			let community_id = community_id.unwrap_or(NoCommunityId::<T>::get()?);
			let char_trait_id = char_trait_id.unwrap_or(NoCharTraitId::<T>::get()?);

			Self::process_appreciation(&payer, &payee, community_id, char_trait_id)?;

			T::Currency::transfer(&payer, &payee, amount, ExistenceRequirement::KeepAlive)?;

			// TODO: events

			Ok(())
		}
	}
}

impl<T: pallet::Config> Pallet<T> {
	pub fn increment_trait_score(
		account_id: &T::AccountId,
		community_id: CommunityId,
		char_trait_id: CharTraitId,
	) {
		TraitScores::<T>::mutate((account_id, community_id, char_trait_id), |value| {
			*value = Some(value.unwrap_or_default() + 1)
		});
	}

	pub fn process_appreciation(
		payer: &T::AccountId,
		payee: &T::AccountId,
		community_id: CommunityId,
		char_trait_id: CharTraitId,
	) -> DispatchResult {
		if NoCharTraitId::<T>::get()? == char_trait_id {
			return Ok(())
		}

		// TODO: whether to check `char_trait_id` for existence?

		// TODO: if this transfer lead to user signup set `true`
		let sign_ups = false;
		if sign_ups {
			// Give payer karma points for helping to grow the network
			Self::increment_trait_score(
				payer,
				NoCommunityId::<T>::get()?,
				AmbassadorCharTraitId::<T>::get()?,
			);
		}

		// Standard appreciation w/o a community context
		if NoCommunityId::<T>::get()? == community_id {
			Self::increment_trait_score(payer, community_id, SpenderCharTraitId::<T>::get()?);
			Self::increment_trait_score(payee, community_id, char_trait_id);
			return Ok(())
		}

		let is_community_closed = Communities::<T>::get()
			.iter()
			.find(|v| v.id == community_id)
			.map(|v| v.closed)
			.ok_or(Error::<T>::CommunityNotFound)?;

		let payer_membership =
			CommunityMembership::<T>::get(payer, community_id).unwrap_or_default();
		let payee_membership =
			CommunityMembership::<T>::get(payee, community_id).unwrap_or_default();

		match (payer_membership, payee_membership) {
			(CommunityRole::None, _) => return Err(Error::<T>::NotMember.into()),
			(_, CommunityRole::Admin) | (_, CommunityRole::Member) => {
				Self::increment_trait_score(payer, community_id, SpenderCharTraitId::<T>::get()?);
				Self::increment_trait_score(payee, community_id, char_trait_id);
			},
			(CommunityRole::Admin, CommunityRole::None) => {
				Self::increment_trait_score(payer, community_id, SpenderCharTraitId::<T>::get()?);
				Self::increment_trait_score(
					payer,
					community_id,
					AmbassadorCharTraitId::<T>::get()?,
				);
				Self::increment_trait_score(payee, community_id, char_trait_id);
				CommunityMembership::<T>::insert(payee, community_id, CommunityRole::Member);
			},
			(CommunityRole::Member, CommunityRole::None) if !is_community_closed => {
				Self::increment_trait_score(payer, community_id, SpenderCharTraitId::<T>::get()?);
				Self::increment_trait_score(
					payer,
					community_id,
					AmbassadorCharTraitId::<T>::get()?,
				);
				Self::increment_trait_score(payee, community_id, char_trait_id);
				CommunityMembership::<T>::insert(payee, community_id, CommunityRole::Member);
			},
			(CommunityRole::Member, CommunityRole::None) =>
				return Err(Error::<T>::CommunityClosed.into()),
		}

		Ok(())
	}
}
