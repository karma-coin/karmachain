#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_balances::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
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

		pub tx_fee_subsidy_max_per_user: u64,
		pub tx_fee_subsidies_alloc: T::Balance,
		pub tx_fee_subsidy_max_amount: T::Balance,

		pub karma_reward_amount: T::Balance,
		pub karma_reward_alloc: T::Balance,
		pub karma_reward_top_n_users: u64,
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
			TxFeeSibsidiesAlloc::<T>::put(self.tx_fee_subsidies_alloc);

			todo!()
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
	pub type SignupRewardPhase1Amount<T: Config> = StorageValue<_, T::Balance, ValueQuery>;
	#[pallet::storage]
	pub type SignupRewardPhase1Amount<T: Config> = StorageValue<_, T::Balance, ValueQuery>;

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
	pub type TxFeeSibsidiesAlloc<T: Config> = StorageValue<_, T::Balance, ValueQuery>;

	#[pallet::event]
	pub enum Event<T: Config> {}

	#[pallet::error]
	pub enum Error<T> {}
}
