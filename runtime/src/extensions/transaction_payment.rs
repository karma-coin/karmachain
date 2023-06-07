use crate::*;
use codec::{Decode, Encode};
use pallet_transaction_payment::ChargeTransactionPayment;
use scale_info::TypeInfo;
use sp_runtime::{
	traits::{DispatchInfoOf, PostDispatchInfoOf, SignedExtension},
	transaction_validity::TransactionValidityError,
	DispatchResult,
};

// Wrapper upon `ChargeTransactionPayment` for current `Runtime`
// to enable fee subsidies
#[derive(Encode, Decode, Clone, Eq, PartialEq, TypeInfo)]
#[scale_info(skip_type_params(T))]
pub struct ChargeTransactionPaymentWithSubsidies(ChargeTransactionPayment<Runtime>);

impl sp_std::fmt::Debug for ChargeTransactionPaymentWithSubsidies {
	#[cfg(feature = "std")]
	fn fmt(&self, f: &mut sp_std::fmt::Formatter) -> sp_std::fmt::Result {
		write!(f, "ChargeTransactionPayment<{:?}>", self.0)
	}
	#[cfg(not(feature = "std"))]
	fn fmt(&self, _: &mut sp_std::fmt::Formatter) -> sp_std::fmt::Result {
		Ok(())
	}
}

impl ChargeTransactionPaymentWithSubsidies {
	/// utility constructor. Used only in client/factory code.
	pub fn from(fee: Balance) -> Self {
		Self(ChargeTransactionPayment::from(fee))
	}
}

impl SignedExtension for ChargeTransactionPaymentWithSubsidies {
	const IDENTIFIER: &'static str =
		<ChargeTransactionPayment<Runtime> as SignedExtension>::IDENTIFIER;
	type AccountId = <ChargeTransactionPayment<Runtime> as SignedExtension>::AccountId;
	type Call = <ChargeTransactionPayment<Runtime> as SignedExtension>::Call;
	type AdditionalSigned =
		<ChargeTransactionPayment<Runtime> as SignedExtension>::AdditionalSigned;
	type Pre = Option<<ChargeTransactionPayment<Runtime> as SignedExtension>::Pre>;

	fn additional_signed(&self) -> Result<Self::AdditionalSigned, TransactionValidityError> {
		self.0.additional_signed()
	}

	fn validate(
		&self,
		who: &Self::AccountId,
		call: &Self::Call,
		info: &DispatchInfoOf<Self::Call>,
		len: usize,
	) -> TransactionValidity {
		let fee = TransactionPayment::compute_fee(len as u32, info, self.0.tip());
		if !Reward::subsidies_tx_fee(who, fee.into()) {
			self.0.validate(who, call, info, len)
		} else {
			Ok(Default::default())
		}
	}

	fn pre_dispatch(
		self,
		who: &Self::AccountId,
		call: &Self::Call,
		info: &DispatchInfoOf<Self::Call>,
		len: usize,
	) -> Result<Self::Pre, TransactionValidityError> {
		let fee = TransactionPayment::compute_fee(len as u32, info, self.0.tip());
		if !Reward::subsidies_tx_fee(who, fee.into()) {
			self.0.pre_dispatch(who, call, info, len).map(Some)
		} else {
			Ok(None)
		}
	}

	fn post_dispatch(
		pre: Option<Self::Pre>,
		info: &DispatchInfoOf<Self::Call>,
		post_info: &PostDispatchInfoOf<Self::Call>,
		len: usize,
		result: &DispatchResult,
	) -> Result<(), TransactionValidityError> {
		match pre {
			Some(pre) => <ChargeTransactionPayment<Runtime> as SignedExtension>::post_dispatch(
				pre, info, post_info, len, result,
			),
			None => Ok(()),
		}
	}
}
