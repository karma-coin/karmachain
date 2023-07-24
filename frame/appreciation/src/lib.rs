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
use scale_info::prelude::string::String;
use sp_common::{
	traits::ScoreProvider,
	types::{CharTraitId, CommunityId, Score},
};
use sp_std::vec::Vec;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_system::pallet_prelude::*;
	use sp_common::{
		types::{CharTraitId, CommunityId},
		BoundedString,
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

		type IdentityProvider: IdentityProvider<
			Self::AccountId,
			Self::Username,
			Self::PhoneNumberHash,
		>;
	}

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config> {
		pub char_traits: Vec<(CharTraitId, String, String)>,
		pub no_char_trait_id: CharTraitId,
		pub signup_char_trait_id: CharTraitId,
		pub spender_char_trait_id: CharTraitId,
		pub ambassador_char_trait_id: CharTraitId,

		pub communities: Vec<GenesisCommunity>,
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
		T::AccountId,
		Blake2_128Concat,
		CommunityId,
		CommunityRole,
		OptionQuery,
	>;

	#[pallet::storage]
	pub type TraitScores<T: Config> = StorageNMap<
		_,
		(
			NMapKey<Blake2_128Concat, T::AccountId>,
			NMapKey<Blake2_128Concat, CommunityId>,
			NMapKey<Blake2_128Concat, CharTraitId>,
		),
		Score,
		OptionQuery,
	>;

	#[pallet::storage]
	pub type Referral<T: Config> = StorageValue<_, bool, ValueQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Happens when `appreciation` tx happen
		Appreciation {
			/// Sender of appreciation
			payer: T::AccountId,
			/// Receiver of appreciation
			payee: T::AccountId,
			/// Amount of tokens sent with appreciation
			amount: T::Balance,
			/// Community in which appreciation happened
			community_id: CommunityId,
			/// Character trait
			char_trait_id: CharTraitId,
		},
		/// May happens multiply times when `appreciation` tx happen and ones per `new_user` tx.
		/// Happens even if transaction execution fails
		CharTraitScoreIncreased {
			/// Receiver of appreciation
			who: T::AccountId,
			/// Community in which appreciation happened
			community_id: CommunityId,
			/// Character trait
			char_trait_id: CharTraitId,
		},
		/// May happen if `Admin` of closed community or any `Member` of opened community
		/// appreciate someone with `appreciation` tx, whose not a part of this community
		NewCommunityMember {
			community_id: CommunityId,
			/// Community member whose appreciation leads to expansion of the community
			payer: T::AccountId,
			/// The account who was add to community
			payee: T::AccountId,
		},
		/// Happens when `set_admin` tx happen
		NewCommunityAdmin {
			community_id: CommunityId,
			community_name: BoundedString<T::CommunityNameLimit>,
			account_id: T::AccountId,
			username: T::Username,
			phone_number_hash: T::PhoneNumberHash,
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
		/// Try to add character trait with existed property
		/// (same `id` or `name` or `emoji`)
		CharTraitAlreadyExists,
		/// No more char traits can be added
		CharTraitLimitExceeded,
		/// Try to add character trait with existed property
		/// (same `id` or `name` or `emoji`)
		CommunityAlreadyExists,
		/// No more communities can be added
		CommunityLimitExceeded,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn appreciation(
			origin: OriginFor<T>,
			to: AccountIdentity<T::AccountId, T::Username, T::PhoneNumberHash>,
			amount: T::Balance,
			community_id: Option<CommunityId>,
			char_trait_id: Option<CharTraitId>,
		) -> DispatchResult {
			let payer = ensure_signed(origin)?;
			let payee = Self::get_account_id(to).ok_or(Error::<T>::NotFound)?;
			let community_id = community_id.unwrap_or(NoCommunityId::<T>::get()?);
			let char_trait_id = char_trait_id.unwrap_or(NoCharTraitId::<T>::get()?);
			let referral = Referral::<T>::take();

			let new_member =
				Self::process_appreciation(&payer, &payee, community_id, char_trait_id, referral)?;

			T::Currency::transfer(&payer, &payee, amount, ExistenceRequirement::KeepAlive)?;

			T::Hooks::on_appreciation(
				payer.clone(),
				payee.clone(),
				amount,
				community_id,
				char_trait_id,
			)?;

			if new_member {
				Self::deposit_event(Event::<T>::NewCommunityMember {
					community_id,
					payer: payer.clone(),
					payee: payee.clone(),
				});
			}

			Self::deposit_event(Event::<T>::Appreciation {
				payer,
				payee,
				amount,
				community_id,
				char_trait_id,
			});

			Ok(())
		}

		#[pallet::call_index(1)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn set_admin(
			origin: OriginFor<T>,
			community_id: CommunityId,
			new_admin: AccountIdentity<T::AccountId, T::Username, T::PhoneNumberHash>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			ensure!(
				matches!(
					CommunityMembership::<T>::get(&who, community_id),
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
				&new_admin_identity.account_id,
				community_id,
				CommunityRole::Admin,
			);

			T::Hooks::on_set_admin(who, new_admin_identity.account_id.clone())?;

			Self::deposit_event(Event::<T>::NewCommunityAdmin {
				community_id: community.id,
				community_name: community.name,
				account_id: new_admin_identity.account_id,
				username: new_admin_identity.username,
				phone_number_hash: new_admin_identity.phone_number_hash,
			});

			Ok(())
		}

		#[pallet::call_index(2)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn add_char_trait(
			origin: OriginFor<T>,
			id: CharTraitId,
			name: BoundedString<T::CharNameLimit>,
			emoji: BoundedString<T::EmojiLimit>,
		) -> DispatchResult {
			// Only sudo can call
			ensure_root(origin)?;

			ensure!(
				id != NoCharTraitId::<T>::get()?,
				Error::<T>::CharTraitAlreadyExists
			);

			let mut traits = CharTraits::<T>::get();
			ensure!(
				!traits.iter().any(|t| t.id == id || t.name == name || t.emoji == emoji),
				Error::<T>::CharTraitAlreadyExists
			);

			let char_trait = CharTrait { id, name, emoji };
			traits.try_push(char_trait).map_err(|_| Error::<T>::CharTraitLimitExceeded)?;
			CharTraits::<T>::put(traits);

			Ok(())
		}

		#[pallet::call_index(3)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn add_community(
			origin: OriginFor<T>,
			id: CommunityId,
			name: BoundedString<T::CommunityNameLimit>,
			desc: BoundedString<T::CommunityDescLimit>,
			emoji: BoundedString<T::EmojiLimit>,
			website_url: BoundedString<T::CommunityUrlLimit>,
			twitter_url: BoundedString<T::CommunityUrlLimit>,
			insta_url: BoundedString<T::CommunityUrlLimit>,
			face_url: BoundedString<T::CommunityUrlLimit>,
			discord_url: BoundedString<T::CommunityUrlLimit>,
			char_traits: BoundedVec<CharTraitId, T::MaxCharTrait>,
			closed: bool,
			admin: T::AccountId
		) -> DispatchResult {
			// Only sudo can call
			ensure_root(origin)?;

			ensure!(
				id != NoCommunityId::<T>::get()?,
				Error::<T>::CommunityAlreadyExists
			);

			let mut communties = Communities::<T>::get();
			ensure!(
				!communties.iter().any(|c| c.id == id || c.name == name),
				Error::<T>::CommunityAlreadyExists
			);

			let community = Community {
				id,
				name,
				desc,
				emoji,
				website_url,
				twitter_url,
				insta_url,
				face_url,
				discord_url,
				char_traits,
				closed,
			};
			communties.try_push(community).map_err(|_| Error::<T>::CommunityLimitExceeded)?;
			Communities::<T>::put(communties);

			CommunityMembership::<T>::insert(admin, id, CommunityRole::Admin);

			Ok(())
		}
	}
}

impl<T: pallet::Config> Pallet<T> {
	fn get_account_id(
		to: AccountIdentity<T::AccountId, T::Username, T::PhoneNumberHash>,
	) -> Option<T::AccountId> {
		match to {
			AccountIdentity::AccountId(account_id) => Some(account_id),
			AccountIdentity::PhoneNumberHash(phone_number_hash) =>
				T::IdentityProvider::identity_by_number(&phone_number_hash).map(|v| v.account_id),
			AccountIdentity::Username(username) =>
				T::IdentityProvider::identity_by_name(&username).map(|v| v.account_id),
		}
	}

	pub fn set_referral_flag(flag: bool) {
		Referral::<T>::put(flag);
	}

	pub fn increment_trait_score(
		account_id: &T::AccountId,
		community_id: CommunityId,
		char_trait_id: CharTraitId,
	) {
		TraitScores::<T>::mutate((account_id, community_id, char_trait_id), |value| {
			*value = Some(value.unwrap_or_default() + 1)
		});

		Self::deposit_event(Event::<T>::CharTraitScoreIncreased {
			who: account_id.clone(),
			community_id,
			char_trait_id,
		})
	}

	/// # Returns
	///
	/// `true` - if appreciation lead to adding `payee` to community as a new member,
	///     otherwise `false`
	pub fn process_appreciation(
		payer: &T::AccountId,
		payee: &T::AccountId,
		community_id: CommunityId,
		char_trait_id: CharTraitId,
		referral: bool,
	) -> Result<bool, DispatchError> {
		if NoCharTraitId::<T>::get()? == char_trait_id {
			return Ok(false)
		}

		// TODO: whether to check `char_trait_id` for existence?
		if referral {
			// Give payer karma points for helping to grow the network
			Self::increment_trait_score(
				payer,
				NoCommunityId::<T>::get()?,
				AmbassadorCharTraitId::<T>::get()?,
			);

			T::Hooks::on_referral(payer.clone(), payee.clone())?;
		}

		// Standard appreciation w/o a community context
		if NoCommunityId::<T>::get()? == community_id {
			Self::increment_trait_score(payer, community_id, SpenderCharTraitId::<T>::get()?);
			Self::increment_trait_score(payee, community_id, char_trait_id);
			return Ok(false)
		}

		let community = Communities::<T>::get()
			.into_iter()
			.find(|v| v.id == community_id)
			.ok_or(Error::<T>::CommunityNotFound)?;

		ensure!(community.char_traits.contains(&char_trait_id), Error::<T>::CharTraitNotFound,);

		let is_community_closed = community.closed;

		let payer_membership =
			CommunityMembership::<T>::get(payer, community_id).unwrap_or_default();
		let payee_membership =
			CommunityMembership::<T>::get(payee, community_id).unwrap_or_default();

		let new_member = match (payer_membership, payee_membership) {
			(CommunityRole::None, _) => return Err(Error::<T>::NotMember.into()),
			(_, CommunityRole::Admin) | (_, CommunityRole::Member) => {
				Self::increment_trait_score(payer, community_id, SpenderCharTraitId::<T>::get()?);
				Self::increment_trait_score(payee, community_id, char_trait_id);
				false
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
				true
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
				true
			},
			(CommunityRole::Member, CommunityRole::None) =>
				return Err(Error::<T>::CommunityClosed.into()),
		};

		Ok(new_member)
	}

	pub fn trait_scores_of(
		account_id: &T::AccountId,
	) -> scale_info::prelude::vec::Vec<(CommunityId, CharTraitId, Score)> {
		let no_community_id = NoCommunityId::<T>::get().unwrap();
		CommunityMembership::<T>::iter_prefix(account_id)
			.map(|(community_id, _)| community_id)
			.chain([no_community_id])
			.flat_map(|community_id| {
				TraitScores::<T>::iter_prefix((&account_id, community_id))
					.map(move |(char_trait_id, score)| (community_id, char_trait_id, score))
			})
			.collect()
	}

	pub fn community_membership_of(
		account_id: &T::AccountId,
	) -> scale_info::prelude::vec::Vec<(CommunityId, Score, bool)> {
		CommunityMembership::<T>::iter_prefix(account_id)
			.map(|(community_id, role)| {
				let score = TraitScores::<T>::iter_prefix((&account_id, community_id))
					.map(|(_, score)| score)
					.sum::<u32>();
				let is_admin = role.is_admin();

				(community_id, score, is_admin)
			})
			.collect()
	}
}

impl<T: Config> Hooks<T::AccountId, T::Balance, T::Username, T::PhoneNumberHash> for Pallet<T> {
	fn on_new_user(
		_verifier: T::AccountId,
		account_id: T::AccountId,
		_name: T::Username,
		_phone_number: T::PhoneNumberHash,
	) -> DispatchResult {
		let no_community_id = NoCommunityId::<T>::get()?;
		let signup_char_trait_id = SignupCharTraitId::<T>::get()?;

		Self::increment_trait_score(&account_id, no_community_id, signup_char_trait_id);

		Ok(())
	}

	fn on_update_user(
		old_account_id: T::AccountId,
		new_account_id: Option<T::AccountId>,
		_username: T::Username,
		_new_username: Option<T::Username>,
		_phone_numbe_hashr: T::PhoneNumberHash,
		_new_phone_number_hash: Option<T::PhoneNumberHash>,
	) -> DispatchResult {
		if let Some(new_account_id) = new_account_id {
			// Migrate user community membership
			// Drain first to do not get undefined behavior from storage while simultaneously drain
			// and insert
			let communities_membership: Vec<_> =
				CommunityMembership::<T>::drain_prefix(&old_account_id).collect();
			communities_membership.iter().for_each(|(community_id, community_role)| {
				CommunityMembership::<T>::insert(&new_account_id, community_id, community_role);
			});

			// Migrate user trait score
			let no_community_id = NoCommunityId::<T>::get()?;
			communities_membership
				.iter()
				.map(|(community_id, _)| community_id)
				.chain(Some(&no_community_id))
				.for_each(|community_id| {
					// Drain first to do not get undefined behavior from storage while
					// simultaneously drain and insert
					let traits_score: Vec<_> =
						TraitScores::<T>::drain_prefix((&old_account_id, community_id)).collect();
					traits_score.iter().for_each(|(char_trait_id, score)| {
						TraitScores::<T>::insert(
							(&new_account_id, community_id, char_trait_id),
							score,
						);
					})
				});
		}

		Ok(())
	}
}

impl<T: Config> ScoreProvider<T::AccountId> for Pallet<T> {
	fn score_of(account_id: &T::AccountId) -> Score {
		Self::trait_scores_of(account_id).iter().map(|(_, _, score)| score).sum()
	}
}
