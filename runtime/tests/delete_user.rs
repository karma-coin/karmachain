mod utils;

use karmachain_node_runtime::*;
use utils::*;
use frame_support::{assert_ok, storage::IterableStorageNMap};
use sp_core::{hashing::blake2_512, sr25519};

#[test]
fn delete_user() {
	let username = "Alice";
	let phone_number = "11111111111";

	new_test_ext()
		.with_user(username, phone_number)
		.execute_with(|| {
			let account_id = get_account_id_from_seed::<sr25519::Public>(&username);
			let username: Username = username.try_into().unwrap();
			let phone_number: PhoneNumber = phone_number.try_into().unwrap();
			let phone_number_hash = PhoneNumberHash::from(blake2_512(Vec::from(phone_number).as_slice()));

			assert_ok!(Identity::delete_user(
				RuntimeOrigin::signed(account_id.clone()),
			));

			// All data from identity pallet removed
			assert!(!pallet_identity::IdentityOf::<Runtime>::contains_key(&account_id));
			assert!(!pallet_identity::UsernameFor::<Runtime>::contains_key(&username));
			assert!(!pallet_identity::PhoneNumberFor::<Runtime>::contains_key(&phone_number_hash));

			// All data from appreciation palelt removed
			assert!(pallet_appreciation::CommunityMembership::<Runtime>::iter_prefix(&account_id).next().is_none());
			pallet_appreciation::Communities::<Runtime>::get()
				.iter()
				.for_each(|community| {
					let community_id = community.id;
					let mut trait_score_records = pallet_appreciation::TraitScores::<Runtime>::iter_prefix((&account_id, community_id));
					assert!(trait_score_records.next().is_none())
				});

			// All data from transaction indexer pallet removed
			assert!(pallet_transaction_indexer::AccountTransactions::<Runtime>::get(&account_id).is_none());
			assert!(pallet_transaction_indexer::PhoneNumberHashTransactions::<Runtime>::get(&phone_number_hash).is_none());

			// Reward pallet keeps data to prevent abuse
			assert!(pallet_reward::AccountRewardInfo::<Runtime>::contains_key(&account_id))
		});
}