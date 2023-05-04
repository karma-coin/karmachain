//! This file contains tests which not complex enough
//! to move to separate file. So keep this tests here
//! in separate modules.

mod utils;

/// Contains tests for `EXISTENTIAL_DEPOSIT` (minimum amount of tokens needed for active account).
mod existential_deposit {
    use frame_support::assert_err;
    use sp_core::sr25519;
    use karmachain_node_runtime::{EXISTENTIAL_DEPOSIT, Runtime, Appreciation, RuntimeOrigin};
    use crate::utils::*;
    use sp_common::identity::AccountIdentity;

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
                assert_err!(Appreciation::appreciation(
                    RuntimeOrigin::signed(who),
                    AccountIdentity::AccountId(to),
                    1, // Transfer 1 Kcent
                    None,
                    None
                ), pallet_balances::Error::<Runtime>::ExistentialDeposit);
            });
    }
}