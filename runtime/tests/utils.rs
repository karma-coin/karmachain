use frame_support::{assert_ok, BoundedVec};
use karmachain_node_runtime::*;
use pallet_appreciation::{Community, CommunityRole};
use sp_common::types::{CharTraitId, CommunityId};
use sp_core::{sr25519, Pair, Public};
use sp_runtime::traits::{IdentifyAccount, Verify};
use sp_common::identity::AccountIdentity;
use frame_support::traits::fungible::Mutate;

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
			"alice_phone_verifier"
				.as_bytes()
				.to_vec()
				.try_into()
				.expect("Invalid name length"),
			"1".as_bytes().to_vec().try_into().expect("Invalid phone number length"),
		));
	});

	ext
}

pub trait TestUtils {
	/// Call `new_user` tx. `AccountId` will be generated from `name`.
	fn with_user(&mut self, phone_verifier: &str, username: &str, phone_number: &str) -> &mut Self;

	/// Create community entity in storage
	fn with_community(&mut self, community_id: CommunityId, name: &str, closed: bool) -> &mut Self;

	/// Add user to community
	fn with_community_member(
		&mut self,
		community_id: CommunityId,
		phone_number: &str,
		role: CommunityRole,
	) -> &mut Self;

	/// Perform `appreciation` tx
	fn with_appreciation(
		&mut self,
		who: &str,
		to: &str,
		amount: Balance,
		char_trait: Option<CharTraitId>,
		community_id: Option<CommunityId>,
	) -> &mut Self;

	/// Set account balance
	fn with_balance(
		&mut self,
		who: &str,
		amount: Balance,
	) -> &mut Self;

	/// Perform `set_admin` tx
	fn with_set_admin(
		&mut self,
		community_id: CommunityId,
		who: &str,
		to: &str,
	) -> &mut Self;
}

impl TestUtils for sp_io::TestExternalities {
	fn with_user(&mut self, phone_verifier: &str, username: &str, phone_number: &str) -> &mut Self {
		self.execute_with(|| {
			// Alice is PhoneVerifier
			let alice = get_account_id_from_seed::<sr25519::Public>(phone_verifier);

			let account_id = get_account_id_from_seed::<sr25519::Public>(&username);
			let username: BoundedVec<_, _> =
				username.as_bytes().to_vec().try_into().expect("Invalid name length");
			let phone_number: BoundedVec<_, _> = phone_number
				.as_bytes()
				.to_vec()
				.try_into()
				.expect("Invalid phone number length");

			assert_ok!(Identity::new_user(
				RuntimeOrigin::signed(alice.clone()),
				account_id,
				username,
				phone_number,
			));
		});

		self
	}

	fn with_community(&mut self, community_id: CommunityId, name: &str, closed: bool) -> &mut Self {
		self.execute_with(|| {
			let no_community_id = pallet_appreciation::NoCommunityId::<Runtime>::get().unwrap();
			assert_ne!(
				no_community_id, community_id,
				"Cannot create community. This community id is taken by `NoCommunityId`"
			);

			let community = Community {
				id: community_id,
				name: name.as_bytes().to_vec().try_into().unwrap(),
				desc: Default::default(),
				emoji: Default::default(),
				website_url: Default::default(),
				twitter_url: Default::default(),
				insta_url: Default::default(),
				face_url: Default::default(),
				discord_url: Default::default(),
				char_traits: Default::default(),
				closed,
			};

			let mut values = pallet_appreciation::Communities::<Runtime>::get().into_inner();
			assert!(
				values.iter().find(|community| community.id == community_id).is_none(),
				"Such community already exists"
			);
			values.push(community);
			let values: BoundedVec<_, _> = values.try_into().unwrap();
			pallet_appreciation::Communities::<Runtime>::put(values);
		});

		self
	}

	fn with_community_member(
		&mut self,
		community_id: CommunityId,
		phone_number: &str,
		role: CommunityRole,
	) -> &mut Self {
		self.execute_with(|| {
			// TODO: check community exists

			let phone_number: BoundedVec<_, PhoneNumberLimit> = phone_number
				.as_bytes()
				.to_vec()
				.try_into()
				.expect("Invalid phone number length");

			pallet_appreciation::CommunityMembership::<Runtime>::insert(
				phone_number,
				community_id,
				role,
			);
		});

		self
	}

	fn with_appreciation(
		&mut self,
		who: &str,
		to: &str,
		amount: Balance,
		char_trait: Option<CharTraitId>,
		community_id: Option<CommunityId>,
	) -> &mut Self {
		self.execute_with(|| {
			let who = get_account_id_from_seed::<sr25519::Public>(who);
			let to = get_account_id_from_seed::<sr25519::Public>(to);

			assert_ok!(Appreciation::appreciation(
				RuntimeOrigin::signed(who),
				AccountIdentity::AccountId(to),
				amount,
				char_trait,
				community_id
			));
		});

		self
	}

	fn with_balance(
		&mut self,
		who: &str,
		amount: Balance,
	) -> &mut Self {
		self.execute_with(|| {
			let who = get_account_id_from_seed::<sr25519::Public>(who);

			assert_ok!(Balances::mint_into(&who, amount));
		});

		self
	}

	fn with_set_admin(
		&mut self,
		community_id: CommunityId,
		who: &str,
		to: &str,
	) -> &mut Self {
		self.execute_with(|| {
			let who = get_account_id_from_seed::<sr25519::Public>(who);
			let to = get_account_id_from_seed::<sr25519::Public>(to);

			assert_ok!(Appreciation::set_admin(
				RuntimeOrigin::signed(who),
				community_id,
				AccountIdentity::AccountId(to),
			));
		});

		self
	}
}
