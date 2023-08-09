//! Testing `Runtime` API

mod utils;

/// Tests API that provides identity information about user such as
/// `get_identity_by_account`, `get_identity_by_name`, `get_identity_by_number`
mod identity {
	use crate::utils::{get_account_id_from_seed, new_test_ext, TestUtils};
	use karmachain_node_runtime::{NameLimit, PhoneNumber, PhoneNumberHash, Runtime};
	use runtime_api::identity::runtime_decl_for_identity_api::IdentityApiV1;
	use sp_common::{identity::AccountIdentity, traits::MaybeLowercase, BoundedString};
	use sp_core::{hashing::blake2_512, sr25519};

	#[test]
	fn get_identity_by_account_user_not_exists() {
		new_test_ext().execute_with(|| {
			let bob = "Bob";
			let bob_account_id = get_account_id_from_seed::<sr25519::Public>(bob);
			assert!(Runtime::get_user_info(AccountIdentity::AccountId(bob_account_id)).is_none())
		});
	}

	#[test]
	fn get_identity_by_name_user_not_exists() {
		new_test_ext().execute_with(|| {
			let bob = "Bob";
			let bob_username = bob.try_into().unwrap();
			assert!(Runtime::get_user_info(AccountIdentity::Username(bob_username)).is_none())
		});
	}

	#[test]
	fn get_identity_by_number_user_not_exists() {
		new_test_ext().execute_with(|| {
			let bob = "Bob";
			let bob_phone_number: PhoneNumber = bob.try_into().unwrap();
			let bob_phone_number_hash =
				PhoneNumberHash::from(blake2_512(Vec::from(bob_phone_number).as_slice()));
			assert!(Runtime::get_user_info(AccountIdentity::PhoneNumberHash(bob_phone_number_hash))
				.is_none())
		});
	}

	#[test]
	fn get_identity_by_account_user_happy_flow() {
		new_test_ext().with_user("Bob", "11111111111").execute_with(|| {
			let bob_account_id = get_account_id_from_seed::<sr25519::Public>("Bob");
			let bob_username: BoundedString<NameLimit> =
				"Bob".try_into().expect("Invalid name length");
			let bob_phone_number: PhoneNumber =
				"11111111111".try_into().expect("Invalid phone number length");
			let bob_phone_number_hash =
				PhoneNumberHash::from(blake2_512(Vec::from(bob_phone_number).as_slice()));

			let info = Runtime::get_user_info(AccountIdentity::AccountId(bob_account_id.clone()))
				.expect("Fail to get info");

			assert_eq!(info.account_id, bob_account_id);
			assert_eq!(info.user_name, bob_username.to_lowercase());
			assert_eq!(info.phone_number_hash, bob_phone_number_hash);
		});
	}

	#[test]
	fn get_identity_by_name_user_happy_flow() {
		new_test_ext().with_user("Bob", "11111111111").execute_with(|| {
			let bob_account_id = get_account_id_from_seed::<sr25519::Public>("Bob");
			let bob_username: BoundedString<NameLimit> =
				"Bob".try_into().expect("Invalid name length");
			let bob_phone_number: PhoneNumber =
				"11111111111".try_into().expect("Invalid phone number length");
			let bob_phone_number_hash =
				PhoneNumberHash::from(blake2_512(Vec::from(bob_phone_number).as_slice()));

			let info = Runtime::get_user_info(AccountIdentity::Username(bob_username.clone()))
				.expect("Fail to get info");

			assert_eq!(info.account_id, bob_account_id);
			assert_eq!(info.user_name, bob_username.to_lowercase());
			assert_eq!(info.phone_number_hash, bob_phone_number_hash);
		});
	}

	#[test]
	fn get_identity_by_number_user_happy_flow() {
		new_test_ext().with_user("Bob", "11111111111").execute_with(|| {
			let bob_account_id = get_account_id_from_seed::<sr25519::Public>("Bob");
			let bob_username: BoundedString<NameLimit> =
				"Bob".try_into().expect("Invalid name length");
			let bob_phone_number: PhoneNumber =
				"11111111111".try_into().expect("Invalid phone number length");
			let bob_phone_number_hash =
				PhoneNumberHash::from(blake2_512(Vec::from(bob_phone_number).as_slice()));

			let info =
				Runtime::get_user_info(AccountIdentity::PhoneNumberHash(bob_phone_number_hash))
					.expect("Fail to get info");

			assert_eq!(info.account_id, bob_account_id);
			assert_eq!(info.user_name, bob_username.to_lowercase());
			assert_eq!(info.phone_number_hash, bob_phone_number_hash);
		});
	}

	#[test]
	fn get_identity_by_name_user_case_insansative() {
		new_test_ext().with_user("Bob", "11111111111").execute_with(|| {
			let bob_account_id = get_account_id_from_seed::<sr25519::Public>("Bob");
			let bob_username: Username = "bob".try_into().expect("Invalid name length");
			let bob_phone_number: PhoneNumber =
				"11111111111".try_into().expect("Invalid phone number length");
			let bob_phone_number_hash =
				PhoneNumberHash::from(blake2_512(Vec::from(bob_phone_number).as_slice()));

			let info = Runtime::get_user_info(AccountIdentity::Username(bob_username.clone()))
				.expect("Fail to get info");

			assert_eq!(info.account_id, bob_account_id);
			assert_eq!(info.user_name, bob_username);
			assert_eq!(info.phone_number_hash, bob_phone_number_hash);
		});
	}

	#[test]
	fn get_identity_by_name_trim_start() {
		new_test_ext().with_user("Bob", "11111111111").execute_with(|| {
			let bob_account_id = get_account_id_from_seed::<sr25519::Public>("Bob");
			let bob_username: Username = " \n\tBob".try_into().expect("Invalid name length");
			let bob_phone_number: PhoneNumber =
				"11111111111".try_into().expect("Invalid phone number length");
			let bob_phone_number_hash =
				PhoneNumberHash::from(blake2_512(Vec::from(bob_phone_number).as_slice()));

			let info = Runtime::get_user_info(AccountIdentity::Username(bob_username.clone()))
				.expect("Fail to get info");

			assert_eq!(info.account_id, bob_account_id);
			assert_eq!(info.user_name, bob_username);
			assert_eq!(info.phone_number_hash, bob_phone_number_hash);
		});
	}

	#[test]
	fn get_identity_by_name_trim_end() {
		new_test_ext().with_user("Bob", "11111111111").execute_with(|| {
			let bob_account_id = get_account_id_from_seed::<sr25519::Public>("Bob");
			let bob_username: Username = "bob \n\t".try_into().expect("Invalid name length");
			let bob_phone_number: PhoneNumber =
				"11111111111".try_into().expect("Invalid phone number length");
			let bob_phone_number_hash =
				PhoneNumberHash::from(blake2_512(Vec::from(bob_phone_number).as_slice()));

			let info = Runtime::get_user_info(AccountIdentity::Username(bob_username.clone()))
				.expect("Fail to get info");

			assert_eq!(info.account_id, bob_account_id);
			assert_eq!(info.user_name, bob_username);
			assert_eq!(info.phone_number_hash, bob_phone_number_hash);
		});
	}

	#[test]
	fn get_identity_by_name_trim() {
		new_test_ext().with_user("Bob", "11111111111").execute_with(|| {
			let bob_account_id = get_account_id_from_seed::<sr25519::Public>("Bob");
			let bob_username: Username = " \n\tBob \n\t".try_into().expect("Invalid name length");
			let bob_phone_number: PhoneNumber =
				"11111111111".try_into().expect("Invalid phone number length");
			let bob_phone_number_hash =
				PhoneNumberHash::from(blake2_512(Vec::from(bob_phone_number).as_slice()));

			let info = Runtime::get_user_info(AccountIdentity::Username(bob_username.clone()))
				.expect("Fail to get info");

			assert_eq!(info.account_id, bob_account_id);
			assert_eq!(info.user_name, bob_username);
			assert_eq!(info.phone_number_hash, bob_phone_number_hash);
		});
	}
}

/// Tests API that fetch list of users by params
/// `get_identity_by_account`, `get_identity_by_name`, `get_identity_by_number`
mod community {
	use crate::utils::{new_test_ext, TestUtils};
	use karmachain_node_runtime::Runtime;
	use pallet_appreciation::CommunityRole;
	use runtime_api::identity::runtime_decl_for_identity_api::IdentityApiV1;

	#[test]
	fn get_all_users_community_not_exists() {
		new_test_ext().execute_with(|| {
			let users = Runtime::get_all_users(1, None, None);
			assert!(users.is_empty());
		});
	}

	#[test]
	fn get_all_users_empty_community() {
		const COMMUNITY_ID: u32 = 1;

		new_test_ext().with_community(COMMUNITY_ID, "test", true).execute_with(|| {
			let users = Runtime::get_all_users(COMMUNITY_ID, None, None);
			assert!(users.is_empty());
		});
	}

	#[test]
	fn get_all_users_happy_flow() {
		const COMMUNITY_ID: u32 = 1;

		new_test_ext()
			.with_community(COMMUNITY_ID, "test", true)
			.with_user("Bob", "1111")
			.with_community_member(COMMUNITY_ID, "Bob", CommunityRole::Member)
			.with_user("Charlie", "2222")
			.with_community_member(COMMUNITY_ID, "Charlie", CommunityRole::Member)
			.execute_with(|| {
				let users = Runtime::get_all_users(COMMUNITY_ID, None, None);
				assert_eq!(users.len(), 2);
			});
	}

	#[test]
	fn get_all_users_pagination_works() {
		const COMMUNITY_ID: u32 = 1;

		new_test_ext()
			.with_community(COMMUNITY_ID, "test", true)
			.with_user("Bob", "1111")
			.with_community_member(COMMUNITY_ID, "Bob", CommunityRole::Member)
			.with_user("Charlie", "2222")
			.with_community_member(COMMUNITY_ID, "Charlie", CommunityRole::Member)
			.execute_with(|| {
				let users = Runtime::get_all_users(COMMUNITY_ID, None, Some(1));
				assert_eq!(users.len(), 1);
				let user = users.first().unwrap();
				assert_eq!(user.user_name, "bob");
			});

		new_test_ext()
			.with_community(COMMUNITY_ID, "test", true)
			.with_user("Bob", "1111")
			.with_community_member(COMMUNITY_ID, "Bob", CommunityRole::Member)
			.with_user("Charlie", "2222")
			.with_community_member(COMMUNITY_ID, "Charlie", CommunityRole::Member)
			.execute_with(|| {
				let users = Runtime::get_all_users(COMMUNITY_ID, Some(1), None);
				assert_eq!(users.len(), 1);
				let user = users.first().unwrap();
				assert_eq!(user.user_name, "charlie");
			});
	}

	#[test]
	fn get_contacts_no_users_exists() {
		new_test_ext().execute_with(|| {
			let prefix = "Bob".try_into().unwrap();
			let users = Runtime::get_contacts(prefix, None, None, None);
			assert!(users.is_empty());
		});
	}

	#[test]
	fn get_contacts_case_insensative() {
		new_test_ext().with_user("Bob", "1111").execute_with(|| {
			let prefix = "bob".try_into().unwrap();
			let users = Runtime::get_contacts(prefix, None, None, None);
			assert_eq!(users.len(), 1);
		});
	}

	#[test]
	fn get_contacts_search_only_prefix() {
		new_test_ext().with_user("Bob", "1111").execute_with(|| {
			let prefix = "ob".try_into().unwrap();
			let users = Runtime::get_contacts(prefix, None, None, None);
			assert!(users.is_empty());
		});
	}

	#[test]
	fn get_contacts_happy_flow() {
		new_test_ext()
			.with_user("Bob", "1111")
			.with_user("Bogdan", "2222")
			.execute_with(|| {
				let prefix = "Bo".try_into().unwrap();
				let users = Runtime::get_contacts(prefix, None, None, None);
				assert_eq!(users.len(), 2);
			});
	}

	#[test]
	fn get_contacts_pagination_works() {
		new_test_ext()
			.with_user("Alice", "0000")
			.with_user("Bob", "1111")
			.with_user("Bogdan", "2222")
			.execute_with(|| {
				let prefix = "Bo".try_into().unwrap();
				let users = Runtime::get_contacts(prefix, None, Some(1), None);
				assert_eq!(users.len(), 1);
				let user = users.first().unwrap();
				assert_eq!(user.user_name, "bogdan");
			});

		new_test_ext()
			.with_user("Alice", "0000")
			.with_user("Bob", "1111")
			.with_user("Bogdan", "2222")
			.execute_with(|| {
				let prefix = "Bo".try_into().unwrap();
				let users = Runtime::get_contacts(prefix, None, None, Some(1));
				assert_eq!(users.len(), 1);
				let user = users.first().unwrap();
				assert_eq!(user.user_name, "bob");
			});
	}
}

/// Tests API that provides transactions by hash or by account
/// `get_transactions_by_account`, `get_transaction`
mod transactions {
	use crate::utils::{get_account_id_from_seed, new_test_ext, TestUtils};
	use karmachain_node_runtime::{Hash, Runtime, System};
	use pallet_appreciation::CommunityRole;
	use runtime_api::transactions::runtime_decl_for_transaction_indexer::TransactionIndexerV1;
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
		new_test_ext().with_user("Bob", "1111").execute_with(|| {
			let bob_account_id = get_account_id_from_seed::<sr25519::Public>("Bob");

			let bob_transactions = Runtime::get_transactions_by_account(bob_account_id);

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
			.with_user("Alice", "1111")
			.with_user("Bob", "2222")
			.with_balance("Alice", 1_000_000_000)
			.with_appreciation("Alice", "Bob", 1_000_000, None, None)
			.execute_with(|| {
				let bob_account_id = get_account_id_from_seed::<sr25519::Public>("Bob");

				let bob_transactions = Runtime::get_transactions_by_account(bob_account_id);

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
			.with_user("Alice", "1111")
			.with_user("Bob", "2222")
			.with_community(COMMUNITY_ID, "test", false)
			.with_community_member(COMMUNITY_ID, "Bob", CommunityRole::Admin)
			.with_set_admin(COMMUNITY_ID, "Bob", "Alice")
			.execute_with(|| {
				let bob_account_id = get_account_id_from_seed::<sr25519::Public>("Bob");

				let bob_transactions = Runtime::get_transactions_by_account(bob_account_id);
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
		new_test_ext().with_user("Bob", "1111").execute_with(|| {
			let extrinsic_data = System::extrinsic_data(0);
			let hash = BlakeTwo256::hash(&extrinsic_data);

			let transaction = Runtime::get_transaction(hash);
			assert!(transaction.is_some());
		});
	}
}
