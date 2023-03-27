use frame_support::{assert_noop, assert_ok, BoundedVec};
use karmachain_node_runtime::*;
use pallet_identity_rpc_runtime_api::runtime_decl_for_IdentityApi::IdentityApiV1;
use sp_core::{sr25519, Pair, Public};
use sp_runtime::traits::{IdentifyAccount, Verify};

/// Generate a crypto pair from seed.
fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

type AccountPublic = <Signature as Verify>::Signer;

/// Generate an account ID from seed.
fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
	AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Construct testing environment and set Alice as PhoneVerifier
fn new_test_ext() -> sp_io::TestExternalities {
	// Constructing testing environment
	let mut ext: sp_io::TestExternalities = frame_system::GenesisConfig::default()
		.build_storage::<Runtime>()
		.unwrap()
		.into();

	ext.execute_with(|| {
		// Creating Alice's AccountId
		let alice = get_account_id_from_seed::<sr25519::Public>("Alice");
		let bounded_vec: BoundedVec<_, MaxPhoneVerifiers> = vec![alice].try_into().unwrap();
		// Set Alice as phone verifier
		pallet_identity::PhoneVerifiers::<Runtime>::put(bounded_vec);
		// Set default id for NoCharTrait
		pallet_appreciation::NoCharTraitId::<Runtime>::put(0);
		// Set default id for SignupTrait
		pallet_appreciation::SignupCharTraitId::<Runtime>::put(1);
		// Set default id for NoCommunity
		pallet_appreciation::NoCommunityId::<Runtime>::put(0);
	});

	ext
}

#[test]
fn new_user_happy_flow() {
	let alice = get_account_id_from_seed::<sr25519::Public>("Alice");
	let mut ext: sp_io::TestExternalities = new_test_ext();

	ext.execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);

		let account_id = get_account_id_from_seed::<sr25519::Public>("Bob");
		let name: BoundedVec<_, NameLimit> =
			"user1234567890".as_bytes().to_vec().try_into().expect("Invelid name length");
		let number: BoundedVec<_, NumberLimit> = "0123456789"
			.as_bytes()
			.to_vec()
			.try_into()
			.expect("Invelid phone number length");

		assert_ok!(Identity::new_user(
			RuntimeOrigin::signed(alice),
			account_id.clone(),
			name.clone(),
			number.clone(),
		));

		let user_info = Runtime::get_user_info_by_account(account_id).expect("Missign user info");
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
			"user1234567890".as_bytes().to_vec().try_into().expect("Invelid name length");
		let number_1: BoundedVec<_, NumberLimit> = "0123456789"
			.as_bytes()
			.to_vec()
			.try_into()
			.expect("Invelid phone number length");
		let number_2: BoundedVec<_, NumberLimit> = "9876543210"
			.as_bytes()
			.to_vec()
			.try_into()
			.expect("Invelid phone number length");

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
fn new_user_existing_number() {
	let alice = get_account_id_from_seed::<sr25519::Public>("Alice");
	let mut ext: sp_io::TestExternalities = new_test_ext();

	ext.execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);

		let account_id_1 = get_account_id_from_seed::<sr25519::Public>("Bob");
		let account_id_2 = get_account_id_from_seed::<sr25519::Public>("Charlie");
		let name_1: BoundedVec<_, NameLimit> =
			"user1234567890".as_bytes().to_vec().try_into().expect("Invelid name length");
		let name_2: BoundedVec<_, NameLimit> =
			"user9876543210".as_bytes().to_vec().try_into().expect("Invelid name length");
		let number: BoundedVec<_, NumberLimit> = "0123456789"
			.as_bytes()
			.to_vec()
			.try_into()
			.expect("Invelid phone number length");

		assert_ok!(Identity::new_user(
			RuntimeOrigin::signed(alice.clone()),
			account_id_1.clone(),
			name_1.clone(),
			number.clone(),
		));

		assert_noop!(
			Identity::new_user(
				RuntimeOrigin::signed(alice),
				account_id_2.clone(),
				name_2.clone(),
				number.clone(),
			),
			pallet_identity::Error::<Runtime>::PhoneNumberTaken
		);
	});
}
