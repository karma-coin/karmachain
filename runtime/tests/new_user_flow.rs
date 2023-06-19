mod utils;

use frame_support::{assert_noop, assert_ok};
use karmachain_node_runtime::*;
use runtime_api::identity::runtime_decl_for_IdentityApi::IdentityApiV1;
use sp_core::{sr25519, hashing::blake2_512};
use utils::*;

#[test]
fn new_user_happy_flow() {
	let mut ext: sp_io::TestExternalities = new_test_ext();

	ext.execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);

		let account_id = get_account_id_from_seed::<sr25519::Public>("Bob");
		let username: Username = "Bob".try_into().unwrap();
		let phone_number: PhoneNumber = "+0123456789".try_into().unwrap();
		let phone_number_hash = PhoneNumberHash::from(blake2_512(Vec::from(phone_number).as_slice()));


		// let (public_key, signature) =
		// 	get_verification_evidence(account_id.clone(), username.clone(), phone_number_hash.clone());

		assert_ok!(Identity::new_user(
			RuntimeOrigin::signed(account_id.clone()),
			// public_key,
			// signature,
			account_id.clone(),
			username.clone(),
			phone_number_hash,
		));

		let user_info = Runtime::get_user_info_by_account(account_id).expect("Missing user info");
		assert_eq!(user_info.user_name, username);
		assert_eq!(user_info.phone_number_hash, phone_number_hash);
		assert_eq!(user_info.nonce, 0);
		assert_eq!(user_info.karma_score, 1);
		assert_eq!(user_info.trait_scores.len(), 1, "expected signup trait score");
		assert_eq!(
			user_info
				.trait_scores
				.iter()
				.find(|v| v.trait_id == 1 && v.community_id == 0)
				.map(|v| v.karma_score),
			Some(1)
		);

		let user_info =
			Runtime::get_user_info_by_name(username.clone()).expect("Missing user info");
		assert_eq!(user_info.user_name, username);
		assert_eq!(user_info.phone_number_hash, phone_number_hash);
		assert_eq!(user_info.nonce, 0);
		assert_eq!(user_info.karma_score, 1);
		assert_eq!(user_info.trait_scores.len(), 1, "expected signup trait score");
		assert_eq!(
			user_info
				.trait_scores
				.iter()
				.find(|v| v.trait_id == 1 && v.community_id == 0)
				.map(|v| v.karma_score),
			Some(1)
		);

		let user_info =
			Runtime::get_user_info_by_number(phone_number_hash).expect("Missing user info");
		assert_eq!(user_info.user_name, username);
		assert_eq!(user_info.phone_number_hash, phone_number_hash);
		assert_eq!(user_info.nonce, 0);
		assert_eq!(user_info.karma_score, 1);
		assert_eq!(user_info.trait_scores.len(), 1, "expected signup trait score");
		assert_eq!(
			user_info
				.trait_scores
				.iter()
				.find(|v| v.trait_id == 1 && v.community_id == 0)
				.map(|v| v.karma_score),
			Some(1)
		);

		// TODO: check transactions on chain
		// TODO: check chain stats
		// Check that signup char trait score increasing emits event

		let alice = get_account_id_from_seed::<sr25519::Public>("Alice");
		System::assert_has_event(
			pallet_appreciation::Event::<Runtime>::CharTraitScoreIncreased {
				who: user_info.account_id.clone(),
				community_id: 0,
				char_trait_id: 1,
			}
			.into(),
		);
		System::assert_has_event(
			pallet_identity::Event::<Runtime>::NewUser {
				phone_verifier: alice,
				account_id: user_info.account_id,
				username,
				phone_number_hash,
			}
			.into(),
		)
	});
}

#[test]
fn new_user_existing_user_name() {
	let mut ext: sp_io::TestExternalities = new_test_ext();

	ext.execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);

		let account_id_1 = get_account_id_from_seed::<sr25519::Public>("Bob");
		let account_id_2 = get_account_id_from_seed::<sr25519::Public>("Charlie");
		let name: Username = "user1234567890".try_into().expect("Invalid name length");
		let phone_number_1: PhoneNumber =
			"0123456789".try_into().expect("Invalid phone number length");
		let phone_number_2: PhoneNumber =
			"9876543210".try_into().expect("Invalid phone number length");

		let phone_number_1_hash = PhoneNumberHash::from(blake2_512(Vec::from(phone_number_1).as_slice()));
		let phone_number_2_hash = PhoneNumberHash::from(blake2_512(Vec::from(phone_number_2).as_slice()));


		// let (public_key_1, signature_1) =
		// 	get_verification_evidence(account_id_1.clone(), name.clone(), phone_number_1_hash.clone());
		// let (public_key_2, signature_2) =
		// 	get_verification_evidence(account_id_2.clone(), name.clone(), phone_number_2_hash.clone());

		assert_ok!(Identity::new_user(
			RuntimeOrigin::signed(account_id_1.clone()),
			// public_key_1,
			// signature_1,
			account_id_1,
			name.clone(),
			phone_number_1_hash,
		));

		assert_noop!(
			Identity::new_user(
				RuntimeOrigin::signed(account_id_2.clone()),
				// public_key_2,
				// signature_2,
				account_id_2,
				name,
				phone_number_2_hash,
			),
			pallet_identity::Error::<Runtime>::UserNameTaken
		);
	});
}

#[test]
fn new_user_migrate_account_flow() {
	let alice = get_account_id_from_seed::<sr25519::Public>("Alice");
	let mut ext: sp_io::TestExternalities = new_test_ext();

	ext.execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);

		let bob_account_id = get_account_id_from_seed::<sr25519::Public>("Bob");
		let charlie_account_id = get_account_id_from_seed::<sr25519::Public>("Charlie");
		let name: Username = "user1234567890".try_into().expect("Invalid name length");
		let phone_number: PhoneNumber =
			"0123456789".try_into().expect("Invalid phone number length");
		let phone_number_hash = PhoneNumberHash::from(blake2_512(Vec::from(phone_number).as_slice()));


		// let (public_key_1, signature_1) =
		// 	get_verification_evidence(bob_account_id.clone(), name.clone(), phone_number_hash.clone());
		// let (public_key_2, signature_2) =
		// 	get_verification_evidence(charlie_account_id.clone(), name.clone(), phone_number_hash.clone());

		assert_ok!(Identity::new_user(
			RuntimeOrigin::signed(bob_account_id.clone()),
			// public_key_1,
			// signature_1,
			bob_account_id.clone(),
			name.clone(),
			phone_number_hash,
		));

		assert_ok!(Identity::new_user(
			RuntimeOrigin::signed(charlie_account_id.clone()),
			// public_key_2,
			// signature_2,
			charlie_account_id.clone(),
			name.clone(),
			phone_number_hash,
		));

		assert!(Runtime::get_user_info_by_account(bob_account_id.clone()).is_none());
		let user_info = Runtime::get_user_info_by_account(charlie_account_id.clone())
			.expect("Missing user info");
		assert_eq!(user_info.user_name, name);
		assert_eq!(user_info.phone_number_hash, phone_number_hash);
		assert_eq!(user_info.nonce, 0);
		assert_eq!(user_info.karma_score, 1);
		assert_eq!(user_info.trait_scores.len(), 1, "expected signup trait score");
		assert_eq!(
			user_info
				.trait_scores
				.iter()
				.find(|v| v.trait_id == 1 && v.community_id == 0)
				.map(|v| v.karma_score),
			Some(1)
		);

		System::assert_has_event(
			pallet_identity::Event::<Runtime>::AccountMigrated {
				phone_verifier: alice,
				old_account_id: bob_account_id,
				new_account_id: charlie_account_id,
			}
			.into(),
		)
	});
}
