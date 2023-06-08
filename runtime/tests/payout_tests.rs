use karmachain_node_runtime::*;
use pallet_staking::{ActiveEraInfo, EraPayout as EraPayoutT};
use sp_staking::EraIndex;

#[allow(dead_code)]
pub struct TestActiveEraInfo {
	index: EraIndex,
	start: Option<u64>,
}

impl From<TestActiveEraInfo> for ActiveEraInfo {
	fn from(val: TestActiveEraInfo) -> ActiveEraInfo {
		unsafe { std::mem::transmute::<TestActiveEraInfo, ActiveEraInfo>(val) }
	}
}

fn set_era(index: EraIndex) {
	let era_info = TestActiveEraInfo { index, start: None };
	pallet_staking::ActiveEra::<Runtime>::put(ActiveEraInfo::from(era_info));
}

// Note: this functions assumes next:
// during one month all era rewards are the same,
// each month contains 30 eras
fn check_month_payouts(month_index: u32, total_month_reward: Balance) {
	let start_era_index = month_index * 120;
	let end_era_index = (month_index + 1) * 120;

	for era_index in start_era_index..end_era_index {
		set_era(era_index);
		let (payout, rest) = EraPayout::<Staking>::era_payout(0, 0, 0);
		assert_eq!(payout, total_month_reward / 30 / 4);
		assert_eq!(rest, 0);
	}
}

#[test]
fn test_era_payout() {
	let mut ext: sp_io::TestExternalities = frame_system::GenesisConfig::default()
		.build_storage::<Runtime>()
		.unwrap()
		.into();

	ext.execute_with(|| {
		check_month_payouts(0, 10_000_000_000_000);
		check_month_payouts(1, 9_799_640_000_000);
		check_month_payouts(2, 9_603_294_412_960);
		check_month_payouts(3, 9_410_882_806_101);
		check_month_payouts(4, 9_222_326_358_197);
		check_month_payouts(5, 9_037_547_827_284);
		check_month_payouts(6, 8_856_471_519_016);
		check_month_payouts(7, 8_679_023_255_660);
		check_month_payouts(8, 8_505_130_345_709);
		check_month_payouts(9, 8_334_721_554_102);
		check_month_payouts(10, 8_167_727_073_044);
		check_month_payouts(11, 8_004_078_493_408);
	});
}
