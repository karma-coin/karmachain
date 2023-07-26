//! This file contains tests which not complex enough
//! to move to separate file. So keep this tests here
//! in separate modules.

mod utils;

/// Contains tests for `EXISTENTIAL_DEPOSIT` (minimum amount of tokens needed for active account).
mod existential_deposit {
	use crate::utils::*;
	use frame_support::assert_err;
	use karmachain_node_runtime::{Appreciation, RuntimeOrigin, EXISTENTIAL_DEPOSIT};
	use sp_common::identity::AccountIdentity;
	use sp_core::sr25519;

	/// Any tokens transfer that lead sender's balance falls below `EXISTENTIAL_DEPOSIT`
	/// should fail.
	///
	/// This test creates account with minimal possible balance and tries to transfer
	/// `1` token, expecting this lead to `ExistentialDeposit` error.
	#[test]
	fn transfer_tokens_below_existential_deposit_should_fail() {
		new_test_ext()
			.with_user("Bob", "1111")
			.with_balance("Bob", EXISTENTIAL_DEPOSIT)
			.execute_with(|| {
				let who = get_account_id_from_seed::<sr25519::Public>("Bob");
				let to = get_account_id_from_seed::<sr25519::Public>("Alice");

				// Use appreciation tx to perform transfer
				assert_err!(
					Appreciation::appreciation(
						RuntimeOrigin::signed(who),
						AccountIdentity::AccountId(to),
						1,    // Transfer 1 Kcent
						None, // No `char_trait`, means appreciation work like transfer tx
						None
					),
					sp_runtime::TokenError::NotExpendable
				);
			});
	}
}

mod signup_rewards {
	use crate::utils::*;
	use frame_support::traits::GenesisBuild;
	use karmachain_node_runtime::Runtime;
	use runtime_api::identity::runtime_decl_for_identity_api::IdentityApiV1;
	use sp_common::identity::AccountIdentity;
	use sp_core::sr25519;

	/// In order to avoid `ExistentialDeposit` error when registering user,
	/// user receives signup reward. Checks that this reward has been credited.
	#[test]
	fn signup_reward_works() {
		let mut test_executor = new_test_ext();

		test_executor.execute_with(|| {
			// Rewards values are being configured throw `chainSpec` file
			// so here we use default values instead of specific `chainSpec` file.
			pallet_reward::GenesisConfig::<Runtime>::default().build()
		});

		test_executor.with_user("Bob", "1111").execute_with(|| {
			let account_id = get_account_id_from_seed::<sr25519::Public>("Bob");
			let info = Runtime::get_user_info(AccountIdentity::AccountId(account_id))
				.expect("Fail to get info");

			// Default signup reward to  10_000_000 KCents
			assert_eq!(info.balance, 10_000_000)
		});
	}

	/// Signup rewards contains 3 phases. Each phase has total amount of tokens that can be
	/// rewarded. Each phase has own reward amount.
	#[test]
	fn signup_reward_phase_works() {
		let mut test_executor = new_test_ext();

		test_executor.execute_with(|| {
			// Rewards values are being configured throw `chainSpec` file
			// so here we use default values instead of specific `chainSpec` file.
			let mut genesis_config = pallet_reward::GenesisConfig::<Runtime>::default();

			// In order to speed up the test make phases total amounts lower
			genesis_config.signup_reward_phase1_alloc /= 1_000_000;
			genesis_config.signup_reward_phase2_alloc /= 1_000_000;

			genesis_config.build();
		});

		// The first 10 users get 10 KCs on signup
		for number in 0..10_u64 {
			let id = format!("user_{number}");
			test_executor.with_user(&id, &id).execute_with(|| {
				let account_id = get_account_id_from_seed::<sr25519::Public>(&id);
				let info = Runtime::get_user_info(AccountIdentity::AccountId(account_id))
					.expect("Fail to get info");

				// 10 KCoins
				assert_eq!(info.balance, 10_000_000)
			});
		}

		// The next 200 user get 1 KC on signup
		(10..210_u64).for_each(|number| {
			let id = format!("user_{number}");
			test_executor.with_user(&id, &id).execute_with(|| {
				let account_id = get_account_id_from_seed::<sr25519::Public>(&id);
				let info = Runtime::get_user_info(AccountIdentity::AccountId(account_id))
					.expect("Fail to get info");

				// 1 KCoins
				assert_eq!(info.balance, 1_000_000)
			})
		});

		// The users get 1_000 KCents on signup
		(210..250_u64).for_each(|number| {
			let id = format!("user_{number}");
			test_executor.with_user(&id, &id).execute_with(|| {
				let account_id = get_account_id_from_seed::<sr25519::Public>(&id);
				let info = Runtime::get_user_info(AccountIdentity::AccountId(account_id))
					.expect("Fail to get info");

				// 1 KCent
				assert_eq!(info.balance, 1_000)
			})
		});
	}
}
