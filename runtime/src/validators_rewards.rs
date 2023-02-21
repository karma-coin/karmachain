use crate::KCOINS;

pub fn month_payout(month_index: u32) -> u128 {
	const LAMBDA: u128 = 20_036;
	const LAMBDA_DELIMETER: u128 = 1_000_000;
	const INITIAL_AMOUNT: u128 = 10_000_000 * KCOINS;

	let mut amount = INITIAL_AMOUNT;

	(0..month_index).for_each(|_| amount = amount * (LAMBDA_DELIMETER - LAMBDA) / LAMBDA_DELIMETER);

	amount
}

#[cfg(test)]
mod tests {
	use crate::validators_rewards::month_payout;

	#[test]
	fn test_month_payout() {
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

	fn total_payout(months_number: u32) -> u128 {
		(0..months_number).map(month_payout).sum()
	}

	#[test]
	fn test_total_paypout() {
		assert_eq!(total_payout(12), 107_620_843_645_481);
		assert_eq!(total_payout(24), 192_035_499_231_408);
		assert_eq!(total_payout(240), 495_223_524_341_127);
		assert_eq!(total_payout(260), 496_514_472_945_642);
	}
}
