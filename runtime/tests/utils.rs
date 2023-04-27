use frame_support::{assert_noop, assert_ok, BoundedVec};
use karmachain_node_runtime::*;
use runtime_api::identity::runtime_decl_for_IdentityApi::IdentityApiV1;
use sp_core::{sr25519, Pair, Public};
use sp_runtime::traits::{IdentifyAccount, Verify};

/// Generate a crypto pair from seed.
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

pub type AccountPublic = <Signature as Verify>::Signer;

/// Generate an account ID from seed.
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
	AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Construct testing environment and set Alice as PhoneVerifier
pub fn new_test_ext() -> sp_io::TestExternalities {
	// Constructing testing environment
	let mut ext: sp_io::TestExternalities = frame_system::GenesisConfig::default()
		.build_storage::<Runtime>()
		.unwrap()
		.into();

	ext.execute_with(|| {
		// Creating Alice's AccountId
		let alice = get_account_id_from_seed::<sr25519::Public>("Alice");
		let bounded_vec: BoundedVec<_, MaxPhoneVerifiers> = vec![alice.clone()].try_into().unwrap();
		// Set Alice as phone verifier
		pallet_identity::PhoneVerifiers::<Runtime>::put(bounded_vec);
		// Set default id for NoCharTrait
		pallet_appreciation::NoCharTraitId::<Runtime>::put(0);
		// Set default id for SignupTrait
		pallet_appreciation::SignupCharTraitId::<Runtime>::put(1);
		// Set default id for NoCommunity
		pallet_appreciation::NoCommunityId::<Runtime>::put(0);

		// Because of Alice is PhoneVerifier we must register her identity
		assert_ok!(Identity::new_user(
			RuntimeOrigin::signed(alice.clone()),
			alice.clone(),
			"alice_phone_verifier".as_bytes().to_vec().try_into().expect("Invalid name length"),
			"1".as_bytes().to_vec().try_into().expect("Invalid phone number length"),
		));
	});

	ext
}