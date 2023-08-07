mod utils;

use frame_support::assert_ok;
use karmachain_node_runtime::*;
use runtime_api::identity::runtime_decl_for_identity_api::IdentityApiV1;
use sp_common::identity::AccountIdentity;
use sp_core::{hashing::blake2_512, sr25519};
use utils::*;

#[test]
fn delete_user() {
	let username = "Alice";
	let phone_number = "11111111111";

	new_test_ext().with_user(username, phone_number).execute_with(|| {
		let account_id = get_account_id_from_seed::<sr25519::Public>(&username);
		let username: Username = username.try_into().unwrap();
		let phone_number: PhoneNumber = phone_number.try_into().unwrap();
		let phone_number_hash =
			PhoneNumberHash::from(blake2_512(Vec::from(phone_number).as_slice()));

		assert_ok!(Identity::delete_user(RuntimeOrigin::signed(account_id.clone()),));

		// All data from identity pallet removed
		assert!(!pallet_identity::IdentityOf::<Runtime>::contains_key(&account_id));
		assert!(!pallet_identity::UsernameFor::<Runtime>::contains_key(&username));
		assert!(!pallet_identity::PhoneNumberFor::<Runtime>::contains_key(&phone_number_hash));

		// All data from appreciation palelt removed
		assert!(pallet_appreciation::CommunityMembership::<Runtime>::iter_prefix(&account_id)
			.next()
			.is_none());
		pallet_appreciation::Communities::<Runtime>::get().iter().for_each(|community| {
			let community_id = community.id;
			let mut trait_score_records = pallet_appreciation::TraitScores::<Runtime>::iter_prefix(
				(&account_id, community_id),
			);
			assert!(trait_score_records.next().is_none())
		});

		// All data from transaction indexer pallet removed
		assert!(
			pallet_transaction_indexer::AccountTransactions::<Runtime>::get(&account_id).is_none()
		);
		assert!(pallet_transaction_indexer::PhoneNumberHashTransactions::<Runtime>::get(
			&phone_number_hash
		)
		.is_none());

		// Reward pallet keeps data to prevent abuse
		assert!(pallet_reward::AccountRewardInfo::<Runtime>::contains_key(&account_id));
		assert!(pallet_reward::DeletedAccounts::<Runtime>::contains_key(&phone_number_hash));
	});
}

#[test]
fn deleted_user_do_not_get_signup_reward() {
	let mut test_ext = new_test_ext();

	test_ext.with_user("Alice1", "1111").execute_with(|| {
		let account_id = get_account_id_from_seed::<sr25519::Public>(&"Alice1");
		assert_ok!(Identity::delete_user(RuntimeOrigin::signed(account_id.clone())));
	});

	test_ext.with_user("Alice2", "1111").execute_with(|| {
		let account_id = get_account_id_from_seed::<sr25519::Public>(&"Alice2");
		let info = Runtime::get_user_info(AccountIdentity::AccountId(account_id))
			.expect("Fail to get info");

		assert_eq!(info.balance, 0)
	});
}

#[test]
fn delete_user_balance_go_to_treasury() {
	let mut test_ext = new_test_ext();

	test_ext.with_user("Alice", "1111").execute_with(|| {
		let account_id = get_account_id_from_seed::<sr25519::Public>(&"Alice");
		let treasury_account_id = Treasury::account_id();
		let treasury_initial_balance = Balances::free_balance(&treasury_account_id);

		let info = Runtime::get_user_info(AccountIdentity::AccountId(account_id.clone()))
			.expect("Fail to get info");

		assert_ok!(Identity::delete_user(RuntimeOrigin::signed(account_id.clone())));

		assert_eq!(
			Balances::free_balance(&treasury_account_id),
			treasury_initial_balance + info.balance as u128
		);
	})
}
