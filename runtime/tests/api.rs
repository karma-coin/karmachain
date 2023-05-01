//! Testing `Runtime` API

mod utils;

/// Tests API that provides identity information about user such as
/// `get_identity_by_account`, `get_identity_by_name`, `get_identity_by_number`
mod identity {
	use crate::utils::{get_account_id_from_seed, new_test_ext, TestUtils};
	use karmachain_node_runtime::{NameLimit, PhoneNumberLimit, Runtime};
	use runtime_api::identity::runtime_decl_for_IdentityApi::IdentityApiV1;
	use sp_core::{bounded::BoundedVec, sr25519};

	#[test]
	fn get_identity_by_account_user_not_exists() {
		new_test_ext().execute_with(|| {
			let bob = "Bob";
			let bob_account_id = get_account_id_from_seed::<sr25519::Public>(bob);
			assert!(Runtime::get_user_info_by_account(bob_account_id).is_none())
		});
	}

	#[test]
	fn get_identity_by_name_user_not_exists() {
		new_test_ext().execute_with(|| {
			let bob = "Bob";
			let bob_username = bob.as_bytes().to_vec().try_into().unwrap();
			assert!(Runtime::get_user_info_by_name(bob_username).is_none())
		});
	}

	#[test]
	fn get_identity_by_number_user_not_exists() {
		new_test_ext().execute_with(|| {
			let bob = "Bob";
			let bob_phone_number = bob.as_bytes().to_vec().try_into().unwrap();
			assert!(Runtime::get_user_info_by_number(bob_phone_number).is_none())
		});
	}

	#[test]
	fn get_identity_by_account_user_happy_flow() {
		new_test_ext().with_user("Alice", "Bob", "11111111111").execute_with(|| {
			let bob_account_id = get_account_id_from_seed::<sr25519::Public>("Bob");
			let bob_username: BoundedVec<_, NameLimit> =
				"Bob".as_bytes().to_vec().try_into().expect("Invalid name length");
			let bob_phone_number: BoundedVec<_, PhoneNumberLimit> = "11111111111"
				.as_bytes()
				.to_vec()
				.try_into()
				.expect("Invalid phone number length");

			let info = Runtime::get_user_info_by_account(bob_account_id.clone())
				.expect("Fail to get info");

			assert_eq!(info.account_id, bob_account_id);
			assert_eq!(info.user_name, bob_username.into_inner());
			assert_eq!(info.mobile_number, bob_phone_number.into_inner());
		});
	}

	#[test]
	fn get_identity_by_name_user_happy_flow() {
		new_test_ext().with_user("Alice", "Bob", "11111111111").execute_with(|| {
			let bob_account_id = get_account_id_from_seed::<sr25519::Public>("Bob");
			let bob_username: BoundedVec<_, NameLimit> =
				"Bob".as_bytes().to_vec().try_into().expect("Invalid name length");
			let bob_phone_number: BoundedVec<_, PhoneNumberLimit> = "11111111111"
				.as_bytes()
				.to_vec()
				.try_into()
				.expect("Invalid phone number length");

			let info =
				Runtime::get_user_info_by_name(bob_username.clone()).expect("Fail to get info");

			assert_eq!(info.account_id, bob_account_id);
			assert_eq!(info.user_name, bob_username.into_inner());
			assert_eq!(info.mobile_number, bob_phone_number.into_inner());
		});
	}

	#[test]
	fn get_identity_by_number_user_happy_flow() {
		new_test_ext().with_user("Alice", "Bob", "11111111111").execute_with(|| {
			let bob_account_id = get_account_id_from_seed::<sr25519::Public>("Bob");
			let bob_username: BoundedVec<_, NameLimit> =
				"Bob".as_bytes().to_vec().try_into().expect("Invalid name length");
			let bob_phone_number: BoundedVec<_, PhoneNumberLimit> = "11111111111"
				.as_bytes()
				.to_vec()
				.try_into()
				.expect("Invalid phone number length");

			let info = Runtime::get_user_info_by_number(bob_phone_number.clone())
				.expect("Fail to get info");

			assert_eq!(info.account_id, bob_account_id);
			assert_eq!(info.user_name, bob_username.into_inner());
			assert_eq!(info.mobile_number, bob_phone_number.into_inner());
		});
	}
}

/// Tests API that fetch list of users by params
/// `get_identity_by_account`, `get_identity_by_name`, `get_identity_by_number`
mod community {
	use crate::utils::{new_test_ext, TestUtils};
	use karmachain_node_runtime::Runtime;
	use pallet_appreciation::CommunityRole;
	use runtime_api::identity::runtime_decl_for_IdentityApi::IdentityApiV1;

	#[test]
	fn get_all_users_community_not_exists() {
		new_test_ext().execute_with(|| {
			let users = Runtime::get_all_users(1);
			assert!(users.is_empty());
		});
	}

	#[test]
	fn get_all_users_empty_community() {
		const COMMUNITY_ID: u32 = 1;

		new_test_ext().with_community(COMMUNITY_ID, "test", true).execute_with(|| {
			let users = Runtime::get_all_users(COMMUNITY_ID);
			assert!(users.is_empty());
		});
	}

	#[test]
	fn get_all_users_happy_flow() {
		const COMMUNITY_ID: u32 = 1;

		new_test_ext()
			.with_community(COMMUNITY_ID, "test", true)
			.with_user("Alice", "Bob", "1111")
			.with_community_member(COMMUNITY_ID, "1111", CommunityRole::Member)
			.with_user("Alice", "Charlie", "2222")
			.with_community_member(COMMUNITY_ID, "2222", CommunityRole::Member)
			.execute_with(|| {
				let users = Runtime::get_all_users(COMMUNITY_ID);
				assert_eq!(users.len(), 2);
			});
	}

	#[test]
	fn get_contacts_no_users_exists() {
		new_test_ext().execute_with(|| {
			let prefix = "Bob".as_bytes().to_vec().try_into().unwrap();
			let users = Runtime::get_contacts(prefix, None);
			assert!(users.is_empty());
		})
	}

	#[test]
	fn get_contacts_case_matters() {
		new_test_ext().with_user("Alice", "Bob", "1111").execute_with(|| {
			let prefix = "bob".as_bytes().to_vec().try_into().unwrap();
			let users = Runtime::get_contacts(prefix, None);
			assert!(users.is_empty());
		})
	}

	#[test]
	fn get_contacts_search_only_prefix() {
		new_test_ext().with_user("Alice", "Bob", "1111").execute_with(|| {
			let prefix = "ob".as_bytes().to_vec().try_into().unwrap();
			let users = Runtime::get_contacts(prefix, None);
			assert!(users.is_empty());
		})
	}

	#[test]
	fn get_contacts_happy_flow() {
		new_test_ext()
			.with_user("Alice", "Bob", "1111")
			.with_user("Alice", "Bogdan", "2222")
			.execute_with(|| {
				let prefix = "Bo".as_bytes().to_vec().try_into().unwrap();
				let users = Runtime::get_contacts(prefix, None);
				assert_eq!(users.len(), 2);
			})
	}
}

/// Tests API that provides transactions by hash or by account
/// `get_transactions_by_account`, `get_transaction`
mod transactions {
	use crate::utils::{get_account_id_from_seed, new_test_ext, TestUtils};
	use karmachain_node_runtime::{Hash, Runtime, System};
	use pallet_appreciation::CommunityRole;
	use runtime_api::transactions::runtime_decl_for_TransactionIndexer::TransactionIndexerV1;
	use sp_core::sr25519;
	use sp_runtime::traits::{BlakeTwo256, Hash as HashT};

	#[test]
	fn get_transactions_by_account_no_transactions() {
		new_test_ext().execute_with(|| {
			let account_id = get_account_id_from_seed::<sr25519::Public>("Bob");
			let transactions = Runtime::get_transactions_by_account(account_id);
			assert!(transactions.is_empty());
		});
	}

	#[test]
	fn get_transactions_by_account_new_user_tx() {
		new_test_ext().with_user("Alice", "Bob", "1111").execute_with(|| {
			let alice_account_id = get_account_id_from_seed::<sr25519::Public>("Alice");
			let bob_account_id = get_account_id_from_seed::<sr25519::Public>("Bob");

			let alice_transactions = Runtime::get_transactions_by_account(alice_account_id);
			let bob_transactions = Runtime::get_transactions_by_account(bob_account_id);

			// Alice has +2 transaction because of registration transactions inside `new_test_ext`
			assert_eq!(alice_transactions.len(), 3);
			assert_eq!(bob_transactions.len(), 1);
		});
	}

	// TODO: for now this feature is not implemented, so no way to test
	#[test]
	#[ignore]
	fn get_transactions_by_account_update_user_tx() {
		todo!()
	}

	#[test]
	fn get_transactions_by_account_appreciation_tx() {
		new_test_ext()
			.with_user("Alice", "Bob", "1111")
			.with_balance("Alice", 1_000_000_000)
			.with_appreciation("Alice", "Bob", 1_000_000, None, None)
			.execute_with(|| {
				let alice_account_id = get_account_id_from_seed::<sr25519::Public>("Alice");
				let bob_account_id = get_account_id_from_seed::<sr25519::Public>("Bob");

				let alice_transactions = Runtime::get_transactions_by_account(alice_account_id);
				let bob_transactions = Runtime::get_transactions_by_account(bob_account_id);

				// Alice has
				// 	+2 transaction because of registration transactions inside `new_test_ext`
				// 	+1 for Bob registration as she is PhoneVerifier
				//  +1 for transfer
				assert_eq!(alice_transactions.len(), 4);
				// Bob has
				// 	+1 for registration
				//  +1 for transfer
				assert_eq!(bob_transactions.len(), 2);
			});
	}

	#[test]
	fn get_transactions_by_account_set_admin_tx() {
		const COMMUNITY_ID: u32 = 1;

		new_test_ext()
			.with_user("Alice", "Bob", "1111")
			.with_community(COMMUNITY_ID, "test", false)
			.with_community_member(COMMUNITY_ID, "1111", CommunityRole::Admin)
			.with_set_admin(COMMUNITY_ID, "Bob", "Alice")
			.execute_with(|| {
				let alice_account_id = get_account_id_from_seed::<sr25519::Public>("Alice");
				let bob_account_id = get_account_id_from_seed::<sr25519::Public>("Bob");

				let alice_transactions = Runtime::get_transactions_by_account(alice_account_id);
				let bob_transactions = Runtime::get_transactions_by_account(bob_account_id);

				// Alice has
				// 	+2 transaction because of registration transactions inside `new_test_ext`
				// 	+1 for Bob registration as she is PhoneVerifier
				//  +1 for set_admin
				assert_eq!(alice_transactions.len(), 4);
				// Bob has
				// 	+1 for registration
				//  +1 for set_admin
				assert_eq!(bob_transactions.len(), 2);
			});
	}

	#[test]
	fn get_transaction_transaction_not_exists() {
		new_test_ext().execute_with(|| {
			let hash = Hash::random();
			let transaction = Runtime::get_transaction(hash);
			assert!(transaction.is_none());
		});
	}

	#[test]
	fn get_transaction_happy_flow() {
		new_test_ext().with_user("Alice", "Bob", "1111").execute_with(|| {
			let extrinsic_data = System::extrinsic_data(0);
			let hash = BlakeTwo256::hash(&extrinsic_data);

			let transaction = Runtime::get_transaction(hash);
			assert!(transaction.is_some());
		});
	}
}
