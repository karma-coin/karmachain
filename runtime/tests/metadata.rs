mod utils;

use frame_support::assert_ok;
use karmachain_node_runtime::*;
use sp_core::{sr25519};
use utils::*;

#[test]
fn set_metadata_works() {
	new_test_ext().execute_with(|| {
		let account_id = get_account_id_from_seed::<sr25519::Public>("Alice");
		let metadata_string = "metadata";

		assert_ok!(Identity::set_metadata(
			RuntimeOrigin::signed(account_id.clone()),
			metadata_string.as_bytes().to_vec().try_into().expect("Metadata exceeds [`Config::MaxMetadataLen`]")
		));

		let metadata = Identity::metadata(&account_id).expect("Missing metadata");
		assert_eq!(
			String::from_utf8(metadata.to_vec()).expect("Invalid UTF-8 string"),
			metadata_string
		);
	});
}

#[test]
fn set_metadata_override_with_no_error() {
	new_test_ext().execute_with(|| {
		let account_id = get_account_id_from_seed::<sr25519::Public>("Alice");
		let metadata_string = "metadata";

		assert_ok!(Identity::set_metadata(
			RuntimeOrigin::signed(account_id.clone()),
			metadata_string.as_bytes().to_vec().try_into().expect("Metadata exceeds [`Config::MaxMetadataLen`]")
		));

		let metadata = Identity::metadata(&account_id).expect("Missing metadata");
		assert_eq!(
			String::from_utf8(metadata.to_vec()).expect("Invalid UTF-8 string"),
			metadata_string
		);

		let new_metadata_string = "new metadata";

		assert_ok!(Identity::set_metadata(
			RuntimeOrigin::signed(account_id.clone()),
			new_metadata_string.as_bytes().to_vec().try_into().expect("Metadata exceeds [`Config::MaxMetadataLen`]")
		));

		let metadata = Identity::metadata(&account_id).expect("Missing metadata");
		assert_eq!(
			String::from_utf8(metadata.to_vec()).expect("Invalid UTF-8 string"),
			new_metadata_string
		);
	});
}

#[test]
#[should_panic]
fn set_metadata_check_metadata_length() {
	let account_id = get_account_id_from_seed::<sr25519::Public>("Alice");
		let metadata_string = "b".repeat(257);

		assert_ok!(Identity::set_metadata(
			RuntimeOrigin::signed(account_id.clone()),
			metadata_string.as_bytes().to_vec().try_into().expect("Metadata exceeds [`Config::MaxMetadataLen`]")
		));
}

#[test]
fn remove_metadata_works() {
	new_test_ext().execute_with(|| {
		let account_id = get_account_id_from_seed::<sr25519::Public>("Alice");
		let metadata_string = "metadata";

		assert_ok!(Identity::set_metadata(
			RuntimeOrigin::signed(account_id.clone()),
			metadata_string.as_bytes().to_vec().try_into().expect("Metadata exceeds [`Config::MaxMetadataLen`]")
		));

		let metadata = Identity::metadata(&account_id).expect("Missing metadata");
		assert_eq!(
			String::from_utf8(metadata.to_vec()).expect("Invalid UTF-8 string"),
			metadata_string
		);

		assert_ok!(Identity::remove_metadata(RuntimeOrigin::signed(account_id.clone())));
		assert_eq!(Identity::metadata(&account_id), None);
	});
}

#[test]
fn remove_metadata_without_metadata_no_error() {
	new_test_ext().execute_with(|| {
		let account_id = get_account_id_from_seed::<sr25519::Public>("Alice");
		assert_ok!(Identity::remove_metadata(RuntimeOrigin::signed(account_id.clone())));
	});
}
