//! This file contains tests which not complex enough
//! to move to separate file. So keep this tests here
//! in separate modules.

mod utils;

/// Contains tests for `EXISTENTIAL_DEPOSIT` (minimum amount of tokens needed for active account).
mod existential_deposit {
	use crate::utils::*;
	use frame_support::{assert_err, traits::GenesisBuild};
	use karmachain_node_runtime::{Appreciation, Runtime, RuntimeOrigin, EXISTENTIAL_DEPOSIT};
	use runtime_api::identity::runtime_decl_for_IdentityApi::IdentityApiV1;
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
			.with_user("Alice", "Bob", "1111")
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
					pallet_balances::Error::<Runtime>::ExistentialDeposit
				);
			});
	}

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

		test_executor.with_user("Alice", "Bob", "1111").execute_with(|| {
			let account_id = get_account_id_from_seed::<sr25519::Public>("Bob");
			let info = Runtime::get_user_info_by_account(account_id).expect("Fail to get info");

			// Default signup reward to  10_000_000 KCents
			assert_eq!(info.balance, 10_000_000)
		});
	}
}
