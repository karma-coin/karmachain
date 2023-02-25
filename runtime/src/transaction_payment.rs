use crate::*;

impl pallet_transaction_payment::Config for Runtime {
	/// The overarching event type.
	type RuntimeEvent = RuntimeEvent;
	/// Handler for withdrawing, refunding and depositing the transaction fee.
	/// Transaction fees are withdrawn before the transaction is executed.
	/// After the transaction was executed the transaction weight can be
	/// adjusted, depending on the used resources by the transaction. If the
	/// transaction weight is lower than expected, parts of the transaction fee
	/// might be refunded. In the end the fees can be deposited.
	type OnChargeTransaction = CurrencyAdapter<Balances, ()>;
	/// A fee mulitplier for `Operational` extrinsics to compute "virtual tip" to boost their
	/// `priority`
	///
	/// This value is multipled by the `final_fee` to obtain a "virtual tip" that is later
	/// added to a tip component in regular `priority` calculations.
	/// It means that a `Normal` transaction can front-run a similarly-sized `Operational`
	/// extrinsic (with no tip), by including a tip value greater than the virtual tip.
	///
	/// ```rust,ignore
	/// // For `Normal`
	/// let priority = priority_calc(tip);
	///
	/// // For `Operational`
	/// let virtual_tip = (inclusion_fee + tip) * OperationalFeeMultiplier;
	/// let priority = priority_calc(tip + virtual_tip);
	/// ```
	///
	/// Note that since we use `final_fee` the multiplier applies also to the regular `tip`
	/// sent with the transaction. So, not only does the transaction get a priority bump based
	/// on the `inclusion_fee`, but we also amplify the impact of tips applied to `Operational`
	/// transactions.
	type OperationalFeeMultiplier = ConstU8<5>;
	/// Convert a weight value into a deductible fee based on the currency type.
	type WeightToFee = IdentityFee<Balance>;
	/// Convert a length value into a deductible fee based on the currency type.
	type LengthToFee = IdentityFee<Balance>;
	/// Update the multiplier of the next block, based on the previous block's weight.
	type FeeMultiplierUpdate = ConstFeeMultiplier<FeeMultiplier>;
}
