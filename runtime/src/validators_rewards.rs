use crate::{
	EPOCH_DURATION_IN_SLOTS, ERA_DURATION_IN_EPOCH, INITIAL_AMOUNT, LAMBDA, LAMBDA_DELIMETER,
	MONTHS,
};

pub fn era_payout(era_index: u32) -> u128 {
	const ERAS_PER_MONTH: u32 = MONTHS / (ERA_DURATION_IN_EPOCH * EPOCH_DURATION_IN_SLOTS);
	let month_index = era_index / ERAS_PER_MONTH;

	let month_payout = month_payout(month_index);
	month_payout / ERAS_PER_MONTH as u128
}

pub fn month_payout(month_index: u32) -> u128 {
	let mut amount = INITIAL_AMOUNT;

	(0..month_index).for_each(|_| amount = amount * (LAMBDA_DELIMETER - LAMBDA) / LAMBDA_DELIMETER);

	amount
}

#[cfg(test)]
mod tests {
	use crate::validators_rewards::{era_payout, month_payout};

	macro_rules! assert_delta {
		($x:expr, $y:expr, $d:expr) => {
			assert!(
				$x.saturating_sub($y) < $d,
				"Difference {} more then expected {}",
				$x.saturating_sub($y),
				$d
			)
		};
	}

	#[test]
	fn test_era_payouts() {
		// First month
		assert_eq!(era_payout(0), 10_000_000_000_000 / 30 / 4);
		assert_eq!(era_payout(1), 10_000_000_000_000 / 30 / 4);
		assert_eq!(era_payout(2), 10_000_000_000_000 / 30 / 4);
		// ...
		assert_eq!(era_payout(117), 10_000_000_000_000 / 30 / 4);
		assert_eq!(era_payout(118), 10_000_000_000_000 / 30 / 4);
		assert_eq!(era_payout(119), 10_000_000_000_000 / 30 / 4);

		// Second month
		assert_eq!(era_payout(120), 9_799_640_000_000 / 30 / 4);
		assert_eq!(era_payout(121), 9_799_640_000_000 / 30 / 4);
		assert_eq!(era_payout(122), 9_799_640_000_000 / 30 / 4);
		// ...
		assert_eq!(era_payout(237), 9_799_640_000_000 / 30 / 4);
		assert_eq!(era_payout(238), 9_799_640_000_000 / 30 / 4);
		assert_eq!(era_payout(239), 9_799_640_000_000 / 30 / 4);
	}

	#[test]
	fn test_total_eras_payout() {
		const DELTA: u128 = 10;

		assert_delta!((0..120).map(era_payout).sum::<u128>(), 10_000_000_000_000, DELTA);
		assert_delta!((120..240).map(era_payout).sum::<u128>(), 9_799_640_000_000, DELTA);
		assert_delta!((240..360).map(era_payout).sum::<u128>(), 9_603_294_412_960, DELTA);
		assert_delta!((360..480).map(era_payout).sum::<u128>(), 9_410_882_806_101, DELTA);
		assert_delta!((480..600).map(era_payout).sum::<u128>(), 9_222_326_358_197, DELTA);
		assert_delta!((600..720).map(era_payout).sum::<u128>(), 9_037_547_827_284, DELTA);
		assert_delta!((720..840).map(era_payout).sum::<u128>(), 8_856_471_519_016, DELTA);
		assert_delta!((840..960).map(era_payout).sum::<u128>(), 8_679_023_255_660, DELTA);
		assert_delta!((960..1080).map(era_payout).sum::<u128>(), 8_505_130_345_709, DELTA);
		assert_delta!((1080..1200).map(era_payout).sum::<u128>(), 8_334_721_554_102, DELTA);
		assert_delta!((1200..1320).map(era_payout).sum::<u128>(), 8_167_727_073_044, DELTA);
		assert_delta!((1320..1440).map(era_payout).sum::<u128>(), 8_004_078_493_408, DELTA);
	}

	#[test]
	fn test_month_payouts() {
		assert_eq!(month_payout(0), 10_000_000_000_000);
		assert_eq!(month_payout(1), 9_799_640_000_000);
		assert_eq!(month_payout(2), 9_603_294_412_960);
		assert_eq!(month_payout(3), 9_410_882_806_101);
		assert_eq!(month_payout(4), 9_222_326_358_197);
		assert_eq!(month_payout(5), 9_037_547_827_284);
		assert_eq!(month_payout(6), 8_856_471_519_016);
		assert_eq!(month_payout(7), 8_679_023_255_660);
		assert_eq!(month_payout(8), 8_505_130_345_709);
		assert_eq!(month_payout(9), 8_334_721_554_102);
		assert_eq!(month_payout(10), 8_167_727_073_044);
		assert_eq!(month_payout(11), 8_004_078_493_408);
	}

	fn total_monthes_payout(months_number: u32) -> u128 {
		(0..months_number).map(month_payout).sum()
	}

	#[test]
	fn test_total_monthes_payout() {
		assert_eq!(total_monthes_payout(12), 107_620_843_645_481);
		assert_eq!(total_monthes_payout(24), 192_035_499_231_408);
		assert_eq!(total_monthes_payout(240), 495_223_524_341_127);
		assert_eq!(total_monthes_payout(260), 496_514_472_945_642);
	}
}
