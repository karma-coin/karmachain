mod utils;

use frame_support::{assert_noop, assert_ok};
use karmachain_node_runtime::*;
use sp_core::sr25519;
use utils::*;

#[test]
fn add_char_trait() {
	let mut test_executor = new_test_ext();

	test_executor.execute_with(|| {
		let sudo = get_account_id_from_seed::<sr25519::Public>("Alice");
		// Set block number to 1 because events are not emitted on block 0.
		System::set_block_number(1);

		// Adding system reserver char trait it (like `NoCharTraitId`) should failed
		let call = Box::new(RuntimeCall::Appreciation(
			pallet_appreciation::Call::<Runtime>::add_char_trait {
				id: 0,
				name: "name".try_into().unwrap(),
				emoji: "emoji".try_into().unwrap(),
			},
		));
		assert_ok!(Sudo::sudo(RuntimeOrigin::signed(sudo.clone()), call));
		// Should emit event to indicate error when called with the root `key` and `call` is `Err`.
		System::assert_has_event(RuntimeEvent::Sudo(pallet_sudo::Event::Sudid {
			sudo_result: Err(pallet_appreciation::Error::<Runtime>::CharTraitAlreadyExists.into()),
		}));

		// Change block number to separete events for each call
		System::set_block_number(2);
		let call = Box::new(RuntimeCall::Appreciation(
			pallet_appreciation::Call::<Runtime>::add_char_trait {
				id: 1,
				name: "name".try_into().unwrap(),
				emoji: "emoji".try_into().unwrap(),
			},
		));
		assert_ok!(Sudo::sudo(RuntimeOrigin::signed(sudo.clone()), call));
		// Should emit event to indicate success when called with the root `key` and `call` is `Ok`.
		System::assert_has_event(RuntimeEvent::Sudo(pallet_sudo::Event::Sudid {
			sudo_result: Ok(()),
		}));

		// Change block number to separete events for each call
		System::set_block_number(3);
		// Adding char trait with same id should fail
		let call = Box::new(RuntimeCall::Appreciation(
			pallet_appreciation::Call::<Runtime>::add_char_trait {
				id: 1,
				name: "other name".try_into().unwrap(),
				emoji: "other emoji".try_into().unwrap(),
			},
		));
		assert_ok!(Sudo::sudo(RuntimeOrigin::signed(sudo.clone()), call),);
		// Should emit event to indicate error when called with the root `key` and `call` is `Err`.
		System::assert_has_event(RuntimeEvent::Sudo(pallet_sudo::Event::Sudid {
			sudo_result: Err(pallet_appreciation::Error::<Runtime>::CharTraitAlreadyExists.into()),
		}));

		// Change block number to separete events for each call
		System::set_block_number(4);
		// Adding char trait with same name should fail
		let call = Box::new(RuntimeCall::Appreciation(
			pallet_appreciation::Call::<Runtime>::add_char_trait {
				id: 2,
				name: "name".try_into().unwrap(),
				emoji: "other emoji".try_into().unwrap(),
			},
		));
		assert_ok!(Sudo::sudo(RuntimeOrigin::signed(sudo.clone()), call),);
		// Should emit event to indicate error when called with the root `key` and `call` is `Err`.
		System::assert_has_event(RuntimeEvent::Sudo(pallet_sudo::Event::Sudid {
			sudo_result: Err(pallet_appreciation::Error::<Runtime>::CharTraitAlreadyExists.into()),
		}));

		// Change block number to separete events for each call
		System::set_block_number(5);
		// Adding char trait with same emoji should fail
		let call = Box::new(RuntimeCall::Appreciation(
			pallet_appreciation::Call::<Runtime>::add_char_trait {
				id: 2,
				name: "other name".try_into().unwrap(),
				emoji: "emoji".try_into().unwrap(),
			},
		));
		assert_ok!(Sudo::sudo(RuntimeOrigin::signed(sudo.clone()), call),);
		// Should emit event to indicate error when called with the root `key` and `call` is `Err`.
		System::assert_has_event(RuntimeEvent::Sudo(pallet_sudo::Event::Sudid {
			sudo_result: Err(pallet_appreciation::Error::<Runtime>::CharTraitAlreadyExists.into()),
		}));
	});
}
