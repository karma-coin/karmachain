#![cfg_attr(not(feature = "std"), no_std)]

mod types;

pub use pallet::*;
pub use types::*;

use frame_support::{pallet_prelude::*, traits::Currency};
use sp_common::{hooks::Hooks as KarmaHooks, traits::IdentityProvider};

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_system::pallet_prelude::*;
	use sp_common::traits::IdentityProvider;

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config:
		frame_system::Config + pallet_balances::Config + pallet_identity::Config
	{
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		type IdentityProvider: IdentityProvider<Self::AccountId, Self::Username, Self::PhoneNumber>;
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
		pub karma_reward_top_n_users: u32,
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
				karma_reward_top_n_users: 1000,
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
			KarmaRewardTopNUsers::<T>::put(self.karma_reward_top_n_users);
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
	pub type KarmaRewardTopNUsers<T: Config> = StorageValue<_, u32, ValueQuery>;

	#[pallet::storage]
	pub type AccountRewardInfo<T: Config> =
		StorageMap<_, Blake2_128Concat, T::PhoneNumber, AccountRewardsData, ValueQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		RewardIssued { who: T::AccountId, amount: T::Balance, reward_type: RewardType },
	}

	#[pallet::error]
	pub enum Error<T> {
		AlreadyRewarded,
		///
		NotFound,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn offchain_worker(_n: BlockNumberFor<T>) {
			// all_users
			//	.iter()
			// 	.filter(|who| !already_get_reward(who))
			//  .sort_by_score()
			// 	.take(KarmaRewardTopNUsers::<T>::get())
			//  .for_each(|who| Self::issue_karma_reward(who, reward))
			// TODO:
		}
	}
}

impl<T: Config> Pallet<T> {
	pub fn issue_signup_reward(who: &T::AccountId, amount: T::Balance) -> DispatchResult {
		let phone_number =
			T::IdentityProvider::identity_by_id(who).ok_or(Error::<T>::NotFound)?.number;

		// Check that user do not get the reward earlier
		let mut account_reward_info = AccountRewardInfo::<T>::get(&phone_number);
		ensure!(!account_reward_info.signup_reward, Error::<T>::AlreadyRewarded);

		// Mark that user get the reward
		account_reward_info.signup_reward = true;
		AccountRewardInfo::<T>::set(&phone_number, account_reward_info);

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

	pub fn issue_referral_reward(who: &T::AccountId, amount: T::Balance) -> DispatchResult {
		let phone_number =
			T::IdentityProvider::identity_by_id(who).ok_or(Error::<T>::NotFound)?.number;

		// Check that user do not get the reward earlier
		let mut account_reward_info = AccountRewardInfo::<T>::get(&phone_number);
		ensure!(!account_reward_info.referral_reward, Error::<T>::AlreadyRewarded);

		// Mark that user get the reward
		account_reward_info.referral_reward = true;
		AccountRewardInfo::<T>::set(&phone_number, account_reward_info);

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

	pub fn issue_karma_reward(who: &T::AccountId, amount: T::Balance) -> DispatchResult {
		let phone_number =
			T::IdentityProvider::identity_by_id(who).ok_or(Error::<T>::NotFound)?.number;

		// Check that user do not get the reward earlier
		let mut account_reward_info = AccountRewardInfo::<T>::get(&phone_number);
		ensure!(!account_reward_info.karma_reward, Error::<T>::AlreadyRewarded);

		// Mark that user get the reward
		account_reward_info.karma_reward = true;
		AccountRewardInfo::<T>::set(&phone_number, account_reward_info);

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

	pub fn total_rewarded() -> T::Balance {
		let signup_reward = SignupRewardTotalAllocated::<T>::get();
		let referral_reward = ReferralRewardTotalAllocated::<T>::get();
		let fee_subsidies = TxFeeSubsidiesTotalAllocated::<T>::get();
		let karma_reward = KarmaRewardTotalAllocated::<T>::get();

		signup_reward + referral_reward + fee_subsidies + karma_reward
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
}
