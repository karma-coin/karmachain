#![cfg_attr(not(feature = "std"), no_std)]

mod types;

pub use pallet::*;
pub use types::*;

use frame_support::{
	pallet_prelude::*,
	traits::{Currency, Randomness},
};
use sp_common::{hooks::Hooks as KarmaHooks, traits::ScoreProvider};
use sp_std::vec::Vec;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::{traits::Randomness, PalletId};
	use frame_system::pallet_prelude::*;

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config:
		frame_system::Config + pallet_balances::Config + pallet_identity::Config
	{
		/// The Reward's pallet id
		#[pallet::constant]
		type PalletId: Get<PalletId>;

		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		type ScoreProvider: ScoreProvider<Self::AccountId>;

		/// Number of time we should try to generate a random number that has no modulo bias.
		/// The larger this number, the more potential computation is used for picking the winner,
		/// but also the more likely that the chosen winner is done fairly.
		#[pallet::constant]
		type MaxGenerateRandom: Get<u32>;

		/// Something that provides randomness in the runtime.
		type Randomness: Randomness<Self::Hash, Self::BlockNumber>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config> {
		pub signup_reward_phase1_alloc: T::Balance,
		pub signup_reward_phase2_alloc: T::Balance,

		pub signup_reward_phase1_amount: T::Balance,
		pub signup_reward_phase2_amount: T::Balance,
		pub signup_reward_phase3_amount: T::Balance,

		pub referral_reward_phase1_alloc: T::Balance,
		pub referral_reward_phase2_alloc: T::Balance,

		pub referral_reward_phase1_amount: T::Balance,
		pub referral_reward_phase2_amount: T::Balance,

		pub tx_fee_subsidy_max_per_user: u32,
		pub tx_fee_subsidies_alloc: T::Balance,
		pub tx_fee_subsidy_max_amount: T::Balance,

		pub karma_reward_amount: T::Balance,
		pub karma_reward_alloc: T::Balance,
		pub karma_reward_users_participates: u32,
		pub karma_reward_users_win: u32,
	}

	#[cfg(feature = "std")]
	impl<T: Config> Default for GenesisConfig<T> {
		fn default() -> Self {
			Self {
				signup_reward_phase1_alloc: 100_000_000_000_000_u128.try_into().ok().unwrap(),
				signup_reward_phase2_alloc: 200_000_000_000_000_u128.try_into().ok().unwrap(),

				signup_reward_phase1_amount: 10_000_000_u128.try_into().ok().unwrap(),
				signup_reward_phase2_amount: 1_000_000_u128.try_into().ok().unwrap(),
				signup_reward_phase3_amount: 1_000_u128.try_into().ok().unwrap(),

				referral_reward_phase1_alloc: 100_000_000_000_000_u128.try_into().ok().unwrap(),
				referral_reward_phase2_alloc: 200_000_000_000_000_u128.try_into().ok().unwrap(),

				referral_reward_phase1_amount: 10_000_000_u128.try_into().ok().unwrap(),
				referral_reward_phase2_amount: 1_000_000_u128.try_into().ok().unwrap(),

				tx_fee_subsidy_max_per_user: 10,
				tx_fee_subsidies_alloc: 250_000_000_000_000_u128.try_into().ok().unwrap(),
				tx_fee_subsidy_max_amount: 1_000_u128.try_into().ok().unwrap(),

				karma_reward_amount: 10_000_000_u128.try_into().ok().unwrap(),
				karma_reward_alloc: 300_000_000_000_000_u128.try_into().ok().unwrap(),
				karma_reward_users_participates: 1000,
				karma_reward_users_win: 100,
			}
		}
	}

	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
		fn build(&self) {
			SignupRewardPhase1Alloc::<T>::put(self.signup_reward_phase1_alloc);
			SignupRewardPhase2Alloc::<T>::put(self.signup_reward_phase2_alloc);

			SignupRewardPhase1Amount::<T>::put(self.signup_reward_phase1_amount);
			SignupRewardPhase2Amount::<T>::put(self.signup_reward_phase2_amount);
			SignupRewardPhase3Amount::<T>::put(self.signup_reward_phase3_amount);

			ReferralRewardPhase1Alloc::<T>::put(self.referral_reward_phase1_alloc);
			ReferralRewardPhase2Alloc::<T>::put(self.referral_reward_phase2_alloc);

			ReferralRewardPhase1Amount::<T>::put(self.referral_reward_phase1_amount);
			ReferralRewardPhase2Amount::<T>::put(self.referral_reward_phase2_amount);

			TxFeeSubsidyMaxPerUser::<T>::put(self.tx_fee_subsidy_max_per_user);
			TxFeeSubsidyMaxAmount::<T>::put(self.tx_fee_subsidy_max_amount);
			TxFeeSubsidiesAlloc::<T>::put(self.tx_fee_subsidies_alloc);

			KarmaRewardAmount::<T>::put(self.karma_reward_amount);
			MaxKarmaRewardAlloc::<T>::put(self.karma_reward_alloc);
			KarmaRewardUsersParticipates::<T>::put(self.karma_reward_users_participates);
			KarmaRewardUsersWin::<T>::put(self.karma_reward_users_win);
		}
	}

	#[pallet::storage]
	pub type SignupRewardTotalAllocated<T: Config> = StorageValue<_, T::Balance, ValueQuery>;
	#[pallet::storage]
	pub type SignupRewardPhase1Alloc<T: Config> = StorageValue<_, T::Balance, ValueQuery>;
	#[pallet::storage]
	pub type SignupRewardPhase2Alloc<T: Config> = StorageValue<_, T::Balance, ValueQuery>;

	#[pallet::storage]
	pub type SignupRewardPhase1Amount<T: Config> = StorageValue<_, T::Balance, ValueQuery>;
	#[pallet::storage]
	pub type SignupRewardPhase2Amount<T: Config> = StorageValue<_, T::Balance, ValueQuery>;
	#[pallet::storage]
	pub type SignupRewardPhase3Amount<T: Config> = StorageValue<_, T::Balance, ValueQuery>;

	#[pallet::storage]
	pub type ReferralRewardTotalAllocated<T: Config> = StorageValue<_, T::Balance, ValueQuery>;
	#[pallet::storage]
	pub type ReferralRewardPhase1Alloc<T: Config> = StorageValue<_, T::Balance, ValueQuery>;
	#[pallet::storage]
	pub type ReferralRewardPhase2Alloc<T: Config> = StorageValue<_, T::Balance, ValueQuery>;

	#[pallet::storage]
	pub type ReferralRewardPhase1Amount<T: Config> = StorageValue<_, T::Balance, ValueQuery>;
	#[pallet::storage]
	pub type ReferralRewardPhase2Amount<T: Config> = StorageValue<_, T::Balance, ValueQuery>;

	#[pallet::storage]
	pub type TxFeeSubsidyMaxPerUser<T: Config> = StorageValue<_, u32, ValueQuery>;
	#[pallet::storage]
	pub type TxFeeSubsidyMaxAmount<T: Config> = StorageValue<_, T::Balance, ValueQuery>;

	#[pallet::storage]
	pub type TxFeeSubsidiesTotalAllocated<T: Config> = StorageValue<_, T::Balance, ValueQuery>;
	#[pallet::storage]
	pub type TxFeeSubsidiesAlloc<T: Config> = StorageValue<_, T::Balance, ValueQuery>;

	#[pallet::storage]
	pub type KarmaRewardTotalAllocated<T: Config> = StorageValue<_, T::Balance, ValueQuery>;
	#[pallet::storage]
	pub type MaxKarmaRewardAlloc<T: Config> = StorageValue<_, T::Balance, ValueQuery>;
	#[pallet::storage]
	pub type KarmaRewardAmount<T: Config> = StorageValue<_, T::Balance, ValueQuery>;
	#[pallet::storage]
	pub type KarmaRewardUsersParticipates<T: Config> = StorageValue<_, u32, ValueQuery>;
	#[pallet::storage]
	pub type KarmaRewardUsersWin<T: Config> = StorageValue<_, u32, ValueQuery>;

	#[pallet::storage]
	pub type AccountRewardInfo<T: Config> =
		StorageMap<_, Blake2_128Concat, T::AccountId, AccountRewardsData, ValueQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		RewardIssued { who: T::AccountId, amount: T::Balance, reward_type: RewardType },
	}

	#[pallet::error]
	pub enum Error<T> {
		AlreadyRewarded,
		/// Account isn't found
		NotFound,
		/// Account ID is already use
		AlreadyInUse,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn on_initialize(_n: BlockNumberFor<T>) -> Weight {
			let _result = Self::distribute_karma_rewards();

			Weight::zero()
		}
	}
}

impl<T: Config> Pallet<T> {
	pub(crate) fn issue_signup_reward(who: &T::AccountId, amount: T::Balance) -> DispatchResult {
		// Check that user do not get the reward earlier
		let mut account_reward_info = AccountRewardInfo::<T>::get(&who);
		ensure!(!account_reward_info.signup_reward, Error::<T>::AlreadyRewarded);

		// Mark that user get the reward
		account_reward_info.signup_reward = true;
		AccountRewardInfo::<T>::set(&who, account_reward_info);

		// Increase total allocated amount of the reward and deposit the reward to user
		SignupRewardTotalAllocated::<T>::mutate(|value| *value += amount);
		T::Currency::deposit_creating(who, amount);

		Self::deposit_event(Event::<T>::RewardIssued {
			who: who.clone(),
			amount: amount.clone(),
			reward_type: RewardType::Signup,
		});

		Ok(())
	}

	pub(crate) fn issue_referral_reward(who: &T::AccountId, amount: T::Balance) -> DispatchResult {
		// Check that user do not get the reward earlier
		let mut account_reward_info = AccountRewardInfo::<T>::get(&who);
		ensure!(!account_reward_info.referral_reward, Error::<T>::AlreadyRewarded);

		// Mark that user get the reward
		account_reward_info.referral_reward = true;
		AccountRewardInfo::<T>::set(&who, account_reward_info);

		// Increase total allocated amount of the reward and deposit the reward to user
		ReferralRewardTotalAllocated::<T>::mutate(|value| *value += amount);
		T::Currency::deposit_creating(who, amount);

		Self::deposit_event(Event::<T>::RewardIssued {
			who: who.clone(),
			amount: amount.clone(),
			reward_type: RewardType::Referral,
		});

		Ok(())
	}

	pub(crate) fn issue_karma_reward(who: &T::AccountId, amount: T::Balance) -> DispatchResult {
		// Check that user do not get the reward earlier
		let mut account_reward_info = AccountRewardInfo::<T>::get(&who);
		ensure!(!account_reward_info.karma_reward, Error::<T>::AlreadyRewarded);

		// Mark that user get the reward
		account_reward_info.karma_reward = true;
		AccountRewardInfo::<T>::set(&who, account_reward_info);

		// Increase total allocated amount of the reward and deposit the reward to user
		KarmaRewardTotalAllocated::<T>::mutate(|value| *value += amount);
		T::Currency::deposit_creating(who, amount);

		Self::deposit_event(Event::<T>::RewardIssued {
			who: who.clone(),
			amount: amount.clone(),
			reward_type: RewardType::Karma,
		});

		Ok(())
	}

	/// Move reward information from one `AccountId` to another
	///
	/// # Params
	/// `from` - the `AccountId` from which info should be moved
	/// `to` - the `AccountId` to which info should be moved
	///
	/// # Return
	/// `Ok` - success
	/// `Err(NotFound)` - the account from which info should be moved not found in storage
	/// `Err(AlreadyInUse)` - the account to which info should be moved already store some other
	/// reward info
	pub(crate) fn move_reward_info(from: &T::AccountId, to: &T::AccountId) -> DispatchResult {
		ensure!(AccountRewardInfo::<T>::contains_key(from), Error::<T>::NotFound);
		ensure!(!AccountRewardInfo::<T>::contains_key(to), Error::<T>::AlreadyInUse);

		let reward_info = AccountRewardInfo::<T>::take(from);
		AccountRewardInfo::<T>::insert(to, reward_info);

		Ok(())
	}

	pub fn total_rewarded() -> T::Balance {
		let signup_reward = SignupRewardTotalAllocated::<T>::get();
		let referral_reward = ReferralRewardTotalAllocated::<T>::get();
		let fee_subsidies = TxFeeSubsidiesTotalAllocated::<T>::get();
		let karma_reward = KarmaRewardTotalAllocated::<T>::get();

		signup_reward + referral_reward + fee_subsidies + karma_reward
	}

	/// Randomly choose a number from 0 to `total`.
	fn choose_number(max: u32) -> u32 {
		let mut random_number = Self::generate_random_number(0);

		// Best effort attempt to remove bias from modulus operator.
		for i in 1..T::MaxGenerateRandom::get() {
			if random_number < u32::MAX - u32::MAX % max {
				break
			}

			random_number = Self::generate_random_number(i);
		}

		random_number % max
	}

	/// Generate a random number from a given seed.
	/// Note that there is potential bias introduced by using modulus operator.
	/// You should call this function with different seed values until the random
	/// number lies within `u32::MAX - u32::MAX % n`.
	/// TODO: deal with randomness freshness
	/// https://github.com/paritytech/substrate/issues/8311
	fn generate_random_number(seed: u32) -> u32 {
		let (random_seed, _) = T::Randomness::random(&(T::PalletId::get(), seed).encode());
		let random_number = <u32>::decode(&mut random_seed.as_ref())
			.expect("secure hashes should always be bigger than u32; qed");
		random_number
	}

	fn distribute_karma_rewards() -> Result<(), &'static str> {
		let participates_number = KarmaRewardUsersParticipates::<T>::get();
		let winners_number = KarmaRewardUsersWin::<T>::get();

		let mut accounts = AccountRewardInfo::<T>::iter()
			.filter(|(_, info)| !info.karma_reward)
			.map(|(account_id, _)| (T::ScoreProvider::score_of(&account_id), account_id))
			.collect::<Vec<_>>();

		accounts.sort_by(|(score_a, _), (score_b, _)| score_b.cmp(score_a));

		let mut participate_accounts = accounts
			.into_iter()
			.map(|(_, account_id)| account_id)
			.take(participates_number as usize)
			.collect::<Vec<_>>();

		// Winners can't be more than participates
		let winner_accounts = if participate_accounts.len() <= winners_number as usize {
			participate_accounts
		} else {
			(0..winners_number)
				.map(|_| {
					let index = Self::choose_number(participate_accounts.len() as u32);
					participate_accounts.remove(index as usize)
				})
				.collect::<Vec<_>>()
		};

		let reward = KarmaRewardAmount::<T>::get();
		winner_accounts.iter().for_each(|account_id| {
			let total_allocated = KarmaRewardTotalAllocated::<T>::get();

			if total_allocated < MaxKarmaRewardAlloc::<T>::get() {
				let _result = Self::issue_karma_reward(account_id, reward);
			}
		});

		Ok(())
	}
}

impl<T: Config> KarmaHooks<T::AccountId, T::Balance, T::Username, T::PhoneNumber> for Pallet<T> {
	fn on_new_user(
		_verifier: T::AccountId,
		who: T::AccountId,
		_name: T::Username,
		_phone_number: T::PhoneNumber,
	) -> DispatchResult {
		let total_allocated = SignupRewardTotalAllocated::<T>::get();

		let reward = if total_allocated < SignupRewardPhase1Alloc::<T>::get() {
			SignupRewardPhase1Amount::<T>::get()
		} else if total_allocated <
			(SignupRewardPhase2Alloc::<T>::get() + SignupRewardPhase1Alloc::<T>::get())
		{
			SignupRewardPhase2Amount::<T>::get()
		} else {
			SignupRewardPhase3Amount::<T>::get()
		};

		Self::issue_signup_reward(&who, reward)?;
		Ok(())
	}

	fn on_update_user(
		old_account_id: T::AccountId,
		new_account_id: T::AccountId,
	) -> DispatchResult {
		Self::move_reward_info(&old_account_id, &new_account_id)
	}
}
