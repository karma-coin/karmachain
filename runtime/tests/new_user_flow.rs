mod utils;

use utils::*;
use frame_support::{assert_noop, assert_ok, BoundedVec};
use karmachain_node_runtime::*;
use runtime_api::identity::runtime_decl_for_IdentityApi::IdentityApiV1;
use sp_core::sr25519;

#[test]
fn new_user_happy_flow() {
	let alice = get_account_id_from_seed::<sr25519::Public>("Alice");
	let mut ext: sp_io::TestExternalities = new_test_ext();

	ext.execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);

		let account_id = get_account_id_from_seed::<sr25519::Public>("Bob");
		let name: BoundedVec<_, NameLimit> =
			"user1234567890".as_bytes().to_vec().try_into().expect("Invalid name length");
		let number: BoundedVec<_, PhoneNumberLimit> = "0123456789"
			.as_bytes()
			.to_vec()
			.try_into()
			.expect("Invalid phone number length");

		assert_ok!(Identity::new_user(
			RuntimeOrigin::signed(alice),
			account_id.clone(),
			name.clone(),
			number.clone(),
		));

		let user_info = Runtime::get_user_info_by_account(account_id).expect("Missing user info");
		assert_eq!(user_info.user_name, name.clone().into_inner());
		assert_eq!(user_info.mobile_number, number.clone().into_inner());
		assert_eq!(user_info.nonce, 0);
		// TODO:
		// assert_eq!(user_info.balance, SIGN_UP_REWARD, "expected signup rewards balance");
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
			Runtime::get_user_info_by_name(name.clone().into()).expect("Missing user info");
		assert_eq!(user_info.user_name, name.clone().into_inner());
		assert_eq!(user_info.mobile_number, number.clone().into_inner());
		assert_eq!(user_info.nonce, 0);
		// TODO:
		// assert_eq!(user_info.balance, SIGN_UP_REWARD, "expected signup rewards balance");
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
			Runtime::get_user_info_by_number(number.clone().into()).expect("Missing user info");
		assert_eq!(user_info.user_name, name.clone().into_inner());
		assert_eq!(user_info.mobile_number, number.clone().into_inner());
		assert_eq!(user_info.nonce, 0);
		// TODO:
		// assert_eq!(user_info.balance, SIGN_UP_REWARD, "expected signup rewards balance");
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
		// TODO: check block events
	});
}

#[test]
fn new_user_existing_user_name() {
	let alice = get_account_id_from_seed::<sr25519::Public>("Alice");
	let mut ext: sp_io::TestExternalities = new_test_ext();

	ext.execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);

		let account_id_1 = get_account_id_from_seed::<sr25519::Public>("Bob");
		let account_id_2 = get_account_id_from_seed::<sr25519::Public>("Charlie");
		let name: BoundedVec<_, NameLimit> =
			"user1234567890".as_bytes().to_vec().try_into().expect("Invalid name length");
		let number_1: BoundedVec<_, PhoneNumberLimit> = "0123456789"
			.as_bytes()
			.to_vec()
			.try_into()
			.expect("Invalid phone number length");
		let number_2: BoundedVec<_, PhoneNumberLimit> = "9876543210"
			.as_bytes()
			.to_vec()
			.try_into()
			.expect("Invalid phone number length");

		assert_ok!(Identity::new_user(
			RuntimeOrigin::signed(alice.clone()),
			account_id_1.clone(),
			name.clone(),
			number_1.clone(),
		));

		assert_noop!(
			Identity::new_user(
				RuntimeOrigin::signed(alice),
				account_id_2.clone(),
				name.clone(),
				number_2.clone(),
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
		let name: BoundedVec<_, NameLimit> =
			"user1234567890".as_bytes().to_vec().try_into().expect("Invalid name length");
		let number: BoundedVec<_, PhoneNumberLimit> = "0123456789"
			.as_bytes()
			.to_vec()
			.try_into()
			.expect("Invalid phone number length");

		assert_ok!(Identity::new_user(
			RuntimeOrigin::signed(alice.clone()),
			bob_account_id.clone(),
			name.clone(),
			number.clone(),
		));

		assert_ok!(Identity::new_user(
			RuntimeOrigin::signed(alice),
			charlie_account_id.clone(),
			name.clone(),
			number.clone(),
		));

		assert!(Runtime::get_user_info_by_account(bob_account_id).is_none());
		let user_info =
			Runtime::get_user_info_by_account(charlie_account_id).expect("Missing user info");
		assert_eq!(user_info.user_name, name.clone().into_inner());
		assert_eq!(user_info.mobile_number, number.clone().into_inner());
		assert_eq!(user_info.nonce, 0);
		// TODO:
		// assert_eq!(user_info.balance, SIGN_UP_REWARD, "expected signup rewards balance");
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
	});
}
