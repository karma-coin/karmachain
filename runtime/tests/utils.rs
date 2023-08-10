use codec::Encode;
use frame_support::{
	assert_ok,
	traits::{fungible::Mutate, GenesisBuild},
	BoundedVec,
};
use karmachain_node_runtime::*;
use pallet_appreciation::{Community, CommunityRole};
use sp_common::{
	identity::AccountIdentity,
	traits::MaybeNormalized,
	types::{CharTraitId, CommunityId},
	BoundedString,
};
use sp_core::{hashing::blake2_512, sr25519, Pair, Public};
use sp_rpc::types::VerificationEvidence;
use sp_runtime::traits::{IdentifyAccount, Verify};

/// Generate a crypto pair from seed.
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{seed}"), None)
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
	let mut storage = frame_system::GenesisConfig::default().build_storage::<Runtime>().unwrap();

	let sudo = get_account_id_from_seed::<sr25519::Public>("Alice");
	pallet_sudo::GenesisConfig::<Runtime> { key: Some(sudo) }
		.assimilate_storage(&mut storage)
		.unwrap();

	// Constructing testing environment
	let mut ext: sp_io::TestExternalities = storage.into();

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

pub trait TestUtils {
	/// Call `new_user` tx. `AccountId` will be generated from `name`.
	fn with_user(&mut self, username: &str, phone_number: &str) -> &mut Self;

	/// Create community entity in storage
	fn with_community(&mut self, community_id: CommunityId, name: &str, closed: bool) -> &mut Self;

	/// Add user to community
	fn with_community_member(
		&mut self,
		community_id: CommunityId,
		account_id: &str,
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
	fn with_balance(&mut self, who: &str, amount: Balance) -> &mut Self;

	/// Perform `set_admin` tx
	fn with_set_admin(&mut self, community_id: CommunityId, who: &str, to: &str) -> &mut Self;
}

impl TestUtils for sp_io::TestExternalities {
	fn with_user(&mut self, username: &str, phone_number: &str) -> &mut Self {
		self.execute_with(|| {
			let account_id = get_account_id_from_seed::<sr25519::Public>(username);
			let username = BoundedString::try_from(username).expect("Invalid name length");
			let phone_number: PhoneNumber =
				BoundedString::try_from(phone_number).expect("Invalid phone number length");

			let phone_number_hash =
				PhoneNumberHash::from(blake2_512(Vec::from(phone_number).as_slice()));

			let (public_key, signature) = get_verification_evidence(
				account_id.clone(),
				username.clone(),
				phone_number_hash.clone(),
			);

			assert_ok!(Identity::new_user(
				RuntimeOrigin::signed(account_id.clone()),
				public_key,
				signature,
				account_id,
				username,
				phone_number_hash,
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
				!values.iter().any(|community| community.id == community_id),
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
		account_id: &str,
		role: CommunityRole,
	) -> &mut Self {
		self.execute_with(|| {
			// TODO: check community exists

			let account_id = get_account_id_from_seed::<sr25519::Public>(account_id);

			pallet_appreciation::CommunityMembership::<Runtime>::insert(
				account_id,
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

	fn with_balance(&mut self, who: &str, amount: Balance) -> &mut Self {
		self.execute_with(|| {
			let who = get_account_id_from_seed::<sr25519::Public>(who);

			assert_ok!(Balances::mint_into(&who, amount));
		});

		self
	}

	fn with_set_admin(&mut self, community_id: CommunityId, who: &str, to: &str) -> &mut Self {
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

pub fn get_verification_evidence(
	account_id: AccountId,
	username: Username,
	phone_number_hash: PhoneNumberHash,
) -> (sp_core::sr25519::Public, sp_core::sr25519::Signature) {
	// Cast username to lowercase
	let username = username.normalize();

	let pair = sp_core::sr25519::Pair::from_string("//Alice", None).unwrap();
	let data = VerificationEvidence::<sp_core::sr25519::Public, _, _, _> {
		verifier_public_key: pair.public(),
		account_id,
		username,
		phone_number_hash,
	}
	.encode();
	let signature = pair.sign(&data);

	(pair.public(), signature)
}
