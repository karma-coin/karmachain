#![cfg_attr(not(feature = "std"), no_std)]

pub mod crypto;
mod types;

pub use pallet::*;
pub use types::*;

use frame_support::{
	pallet_prelude::*,
	traits::{Currency, Randomness},
};
use frame_system::offchain::{SendSignedTransaction, Signer};
use sp_common::{hooks::Hooks as KarmaHooks, traits::ScoreProvider};
use sp_runtime::traits::Zero;
use sp_std::{default::Default, vec::Vec};

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::{log, traits::Randomness, PalletId};
	use frame_system::{
		offchain::{AppCrypto, CreateSignedTransaction, SigningTypes},
		pallet_prelude::*,
	};

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config:
		frame_system::Config
		+ pallet_balances::Config
		+ pallet_identity::Config
		+ CreateSignedTransaction<Call<Self>>
	{
		/// The Reward's pallet id
		#[pallet::constant]
		type PalletId: Get<PalletId>;
		/// The overarching event type.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		/// Something that provides trait score in the runtime
		type ScoreProvider: ScoreProvider<Self::AccountId>;
		/// Number of time we should try to generate a random number that has no modulo bias.
		/// The larger this number, the more potential computation is used for picking the winner,
		/// but also the more likely that the chosen winner is done fairly.
		#[pallet::constant]
		type MaxGenerateRandom: Get<u32>;
		/// Something that provides randomness in the runtime.
		type Randomness: Randomness<Self::Hash, Self::BlockNumber>;
		/// Maximum number of winners in karma rewards per one round
		#[pallet::constant]
		type MaxWinners: Get<u32>;
		/// Maximum number of offchain account that can sign `submit_karma_rewards` tx
		#[pallet::constant]
		type MaxOffchainAccounts: Get<u32>;
		/// The identifier type for an offchain worker.
		type AuthorityId: AppCrypto<Self::Public, <Self as SigningTypes>::Signature>;
	}

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config> {
		pub offchain_accounts: Vec<T::AccountId>,

		pub signup_reward_phase1_alloc: T::Balance,
		pub signup_reward_phase2_alloc: T::Balance,

		pub signup_reward_phase1_amount: T::Balance,
		pub signup_reward_phase2_amount: T::Balance,
		pub signup_reward_phase3_amount: T::Balance,

		pub referral_reward_phase1_alloc: T::Balance,
		pub referral_reward_phase2_alloc: T::Balance,

		pub referral_reward_phase1_amount: T::Balance,
		pub referral_reward_phase2_amount: T::Balance,

		pub tx_fee_subsidy_max_per_user: u8,
		pub tx_fee_subsidies_alloc: T::Balance,
		pub tx_fee_subsidy_max_amount: T::Balance,

		pub karma_reward_frequency: T::BlockNumber,
		pub karma_reward_amount: T::Balance,
		pub karma_reward_alloc: T::Balance,
		pub karma_reward_users_participates: u32,
	}

	#[cfg(feature = "std")]
	impl<T: Config> Default for GenesisConfig<T> {
		fn default() -> Self {
			Self {
				offchain_accounts: vec![],

				signup_reward_phase1_alloc: 100_000_000_000_000_u128.try_into().ok().unwrap(),
				signup_reward_phase2_alloc: 200_000_000_000_000_u128.try_into().ok().unwrap(),

				signup_reward_phase1_amount: 10_000_000_u128.try_into().ok().unwrap(),
				signup_reward_phase2_amount: 1_000_000_u128.try_into().ok().unwrap(),
				signup_reward_phase3_amount: 1_000_u128.try_into().ok().unwrap(),
				referral_reward_phase1_alloc: 10_000_000_000_000_u128.try_into().ok().unwrap(),
				referral_reward_phase2_alloc: 200_000_000_000_000_u128.try_into().ok().unwrap(),

				referral_reward_phase1_amount: 10_000_000_u128.try_into().ok().unwrap(),
				referral_reward_phase2_amount: 1_000_000_u128.try_into().ok().unwrap(),

				tx_fee_subsidy_max_per_user: 10,
				tx_fee_subsidies_alloc: 250_000_000_000_000_u128.try_into().ok().unwrap(),
				tx_fee_subsidy_max_amount: 1_000_u128.try_into().ok().unwrap(),

				karma_reward_frequency: 5_u32.into(),
				karma_reward_amount: 10_000_000_u128.try_into().ok().unwrap(),
				karma_reward_alloc: 300_000_000_000_000_u128.try_into().ok().unwrap(),
				karma_reward_users_participates: 1000,
			}
		}
	}

	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
		fn build(&self) {
			OffchainAccounts::<T>::put::<BoundedVec<_, T::MaxOffchainAccounts>>(
				self.offchain_accounts.clone().try_into().expect("Too many offchain accounts"),
			);

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

			KarmaRewardFrequency::<T>::put(self.karma_reward_frequency);
			KarmaRewardAmount::<T>::put(self.karma_reward_amount);
			MaxKarmaRewardAlloc::<T>::put(self.karma_reward_alloc);
			KarmaRewardUsersParticipates::<T>::put(self.karma_reward_users_participates);
		}
	}

	#[pallet::storage]
	pub type OffchainAccounts<T: Config> =
		StorageValue<_, BoundedVec<T::AccountId, T::MaxOffchainAccounts>, ValueQuery>;

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
	pub type TxFeeSubsidyMaxPerUser<T: Config> = StorageValue<_, u8, ValueQuery>;
	#[pallet::storage]
	pub type TxFeeSubsidyMaxAmount<T: Config> = StorageValue<_, T::Balance, ValueQuery>;

	#[pallet::storage]
	pub type TxFeeSubsidiesTotalAllocated<T: Config> = StorageValue<_, T::Balance, ValueQuery>;
	#[pallet::storage]
	pub type TxFeeSubsidiesAlloc<T: Config> = StorageValue<_, T::Balance, ValueQuery>;

	#[pallet::storage]
	pub type KarmaRewardFrequency<T: Config> = StorageValue<_, T::BlockNumber, ValueQuery>;
	#[pallet::storage]
	pub type KarmaRewardTotalAllocated<T: Config> = StorageValue<_, T::Balance, ValueQuery>;
	#[pallet::storage]
	pub type MaxKarmaRewardAlloc<T: Config> = StorageValue<_, T::Balance, ValueQuery>;
	#[pallet::storage]
	pub type KarmaRewardAmount<T: Config> = StorageValue<_, T::Balance, ValueQuery>;
	#[pallet::storage]
	pub type KarmaRewardUsersParticipates<T: Config> = StorageValue<_, u32, ValueQuery>;

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
		/// Not enough karma reward allocated to pay to all winners
		TooManyWinners,
		/// Account try to submit transaction without having privilege to do this
		///
		/// This may happen when someone calls `submit_karma_rewards` or if keys for offchain
		/// account different from those which passed throw `GenesisConfig`
		NotAllowed,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn offchain_worker(n: BlockNumberFor<T>) {
			let winners = Self::distribute_karma_rewards(n);

			if winners.is_empty() {
				return
			}

			match Self::sign_and_send_submit_karma_rewards(winners) {
				Ok(()) => log::info!("Submit karma reward tx successfully sent!"),
				Err(e) => log::error!("Fail to submit karma reward tx: {}", e),
			}
		}
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(3,1).ref_time())]
		pub fn submit_karma_rewards(
			origin: OriginFor<T>,
			winners: BoundedVec<T::AccountId, T::MaxWinners>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			ensure!(OffchainAccounts::<T>::get().contains(&who), Error::<T>::NotAllowed);

			let reward_total_allocated = KarmaRewardTotalAllocated::<T>::get();
			let reward_allocated = MaxKarmaRewardAlloc::<T>::get();
			let reward = KarmaRewardAmount::<T>::get();

			// How many accounts can be rewarded due to remained reward amount
			let can_reward_n_accounts = (reward_total_allocated - reward_allocated) / reward;
			let winners_number: T::Balance = (winners.len() as u32).into();
			ensure!(can_reward_n_accounts >= winners_number, Error::<T>::TooManyWinners);

			for winner in winners {
				Self::issue_karma_reward(&winner, reward)?;
			}

			Ok(())
		}
	}
}

impl<T: Config> Pallet<T> {
	pub(crate) fn issue_signup_reward(who: &T::AccountId, amount: T::Balance) -> DispatchResult {
		// Check that user do not get the reward earlier
		let mut account_reward_info = AccountRewardInfo::<T>::get(who);
		ensure!(!account_reward_info.signup_reward, Error::<T>::AlreadyRewarded);

		// Mark that user get the reward
		account_reward_info.signup_reward = true;
		AccountRewardInfo::<T>::set(who, account_reward_info);

		// Increase total allocated amount of the reward and deposit the reward to user
		SignupRewardTotalAllocated::<T>::mutate(|value| *value += amount);
		T::Currency::deposit_creating(who, amount);

		Self::deposit_event(Event::<T>::RewardIssued {
			who: who.clone(),
			amount,
			reward_type: RewardType::Signup,
		});

		Ok(())
	}

	pub(crate) fn issue_referral_reward(who: &T::AccountId, amount: T::Balance) -> DispatchResult {
		// Check that user do not get the reward earlier
		let mut account_reward_info = AccountRewardInfo::<T>::get(who);
		ensure!(!account_reward_info.referral_reward, Error::<T>::AlreadyRewarded);

		// Mark that user get the reward
		account_reward_info.referral_reward = true;
		AccountRewardInfo::<T>::set(who, account_reward_info);

		// Increase total allocated amount of the reward and deposit the reward to user
		ReferralRewardTotalAllocated::<T>::mutate(|value| *value += amount);
		T::Currency::deposit_creating(who, amount);

		Self::deposit_event(Event::<T>::RewardIssued {
			who: who.clone(),
			amount,
			reward_type: RewardType::Referral,
		});

		Ok(())
	}

	pub(crate) fn issue_karma_reward(who: &T::AccountId, amount: T::Balance) -> DispatchResult {
		// Check that user do not get the reward earlier
		let mut account_reward_info = AccountRewardInfo::<T>::get(who);
		ensure!(!account_reward_info.karma_reward, Error::<T>::AlreadyRewarded);

		// Mark that user get the reward
		account_reward_info.karma_reward = true;
		AccountRewardInfo::<T>::set(who, account_reward_info);

		// Increase total allocated amount of the reward and deposit the reward to user
		KarmaRewardTotalAllocated::<T>::mutate(|value| *value += amount);
		T::Currency::deposit_creating(who, amount);

		Self::deposit_event(Event::<T>::RewardIssued {
			who: who.clone(),
			amount,
			reward_type: RewardType::Karma,
		});

		Ok(())
	}

	pub fn subsidies_tx_fee(who: &T::AccountId, amount: T::Balance) -> bool {
		// Not more tokens left
		if TxFeeSubsidiesTotalAllocated::<T>::get() >= TxFeeSubsidiesAlloc::<T>::get() {
			return false
		}

		// Fee is too big
		if amount > TxFeeSubsidyMaxAmount::<T>::get() {
			return false
		}

		// No more tx fee subsidies allowed
		if AccountRewardInfo::<T>::get(who).transaction_subsidized >=
			TxFeeSubsidyMaxPerUser::<T>::get()
		{
			return false
		}

		AccountRewardInfo::<T>::mutate(who, |info| info.transaction_subsidized += 1);
		TxFeeSubsidiesTotalAllocated::<T>::mutate(|value| *value += amount);

		Self::deposit_event(Event::<T>::RewardIssued {
			who: who.clone(),
			amount,
			reward_type: RewardType::Subsidy,
		});

		true
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
	/// reward info
	pub(crate) fn move_reward_info(from: &T::AccountId, to: &T::AccountId) -> DispatchResult {
		ensure!(AccountRewardInfo::<T>::contains_key(from), Error::<T>::NotFound);

		let from_reward_info = AccountRewardInfo::<T>::take(from);
		let to_reward_info = AccountRewardInfo::<T>::take(to);

		ensure!(
			!to_reward_info.karma_reward &&
				!to_reward_info.signup_reward &&
				!to_reward_info.referral_reward,
			Error::<T>::AlreadyInUse
		);

		AccountRewardInfo::<T>::insert(
			from,
			AccountRewardsData {
				transaction_subsidized: from_reward_info.transaction_subsidized,
				..Default::default()
			},
		);
		AccountRewardInfo::<T>::insert(
			to,
			AccountRewardsData {
				transaction_subsidized: to_reward_info.transaction_subsidized,
				..from_reward_info
			},
		);

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

	fn distribute_karma_rewards(block_number: T::BlockNumber) -> Vec<T::AccountId> {
		if block_number % KarmaRewardFrequency::<T>::get() != T::BlockNumber::zero() {
			// Too early, skipping rewards for now
			return sp_std::vec![]
		}

		let participates_number = KarmaRewardUsersParticipates::<T>::get();
		let winners_number = T::MaxWinners::get();

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
		if participate_accounts.len() <= winners_number as usize {
			participate_accounts
		} else {
			(0..winners_number)
				.map(|_| {
					let index = Self::choose_number(participate_accounts.len() as u32);
					participate_accounts.remove(index as usize)
				})
				.collect::<Vec<_>>()
		}
	}

	fn sign_and_send_submit_karma_rewards(winners: Vec<T::AccountId>) -> Result<(), &'static str> {
		let winners: BoundedVec<T::AccountId, T::MaxWinners> =
			winners.try_into().map_err(|_| "Too many winner")?;

		let signer = Signer::<T, T::AuthorityId>::any_account();

		// Using `send_signed_transaction` associated type we create and submit a transaction
		// representing the call, we've just created.
		let result = signer.send_signed_transaction(|_account| Call::submit_karma_rewards {
			winners: winners.clone(),
		});

		match result {
			Some((_, Ok(_))) => Ok(()),
			Some((_, Err(_))) => Err("Failed to submit transaction"),
			None =>
				Err("No local accounts available. Consider adding one via `author_insertKey` RPC."),
		}
	}
}

impl<T: Config> KarmaHooks<T::AccountId, T::Balance, T::Username, T::PhoneNumberHash>
	for Pallet<T>
{
	fn on_new_user(
		_verifier: T::AccountId,
		who: T::AccountId,
		_name: T::Username,
		_phone_number_hash: T::PhoneNumberHash,
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

	fn on_referral(who: T::AccountId, _whom: T::AccountId) -> DispatchResult {
		let total_allocated = ReferralRewardTotalAllocated::<T>::get();

		let reward = if total_allocated < ReferralRewardPhase1Alloc::<T>::get() {
			ReferralRewardPhase1Amount::<T>::get()
		} else {
			ReferralRewardPhase2Amount::<T>::get()
		};

		Self::issue_referral_reward(&who, reward)?;

		Ok(())
	}

	fn on_update_user(
		old_account_id: T::AccountId,
		new_account_id: Option<T::AccountId>,
		_username: T::Username,
		_new_username: Option<T::Username>,
		_phone_number_hash: T::PhoneNumberHash,
		_new_phone_number_hash: Option<T::PhoneNumberHash>,
	) -> DispatchResult {
		if let Some(new_account_id) = new_account_id {
			Self::move_reward_info(&old_account_id, &new_account_id)
		} else {
			Ok(())
		}
	}
}
