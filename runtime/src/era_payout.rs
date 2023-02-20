use crate::UNITS;

pub fn era_payout(month_index: u32) -> u128 {
	const LAMBDA: u128 = 20_036;
	const LAMBDA_DELIMETER: u128 = 1_000_000;
	const INITIAL_AMOUNT: u128 = 10_000_000 * UNITS;

	let mut amount = INITIAL_AMOUNT;

	(0..month_index).for_each(|_| amount = amount * (LAMBDA_DELIMETER - LAMBDA) / LAMBDA_DELIMETER);

	amount
}
