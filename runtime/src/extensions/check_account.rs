use crate::*;
use codec::{Decode, Encode};
use scale_info::TypeInfo;
use sp_runtime::{
	traits::{DispatchInfoOf, SignedExtension},
	transaction_validity::{
		InvalidTransaction, TransactionValidity, TransactionValidityError, ValidTransaction,
	},
};
use sp_std::{default::Default, marker::PhantomData, vec};

pub type AccountIdentityTag = AccountIdentity<
	<Runtime as frame_system::Config>::AccountId,
	<Runtime as pallet_identity::Config>::NameLimit,
	<Runtime as pallet_identity::Config>::PhoneNumberLimit,
>;

#[derive(Encode, Decode, Clone, Eq, PartialEq, TypeInfo)]
#[scale_info(skip_type_params(T))]
pub struct CheckAccount(PhantomData<Runtime>);

impl CheckAccount {
	pub fn new() -> Self {
		Self(Default::default())
	}
}

impl sp_std::fmt::Debug for CheckAccount {
	#[cfg(feature = "std")]
	fn fmt(&self, f: &mut sp_std::fmt::Formatter) -> sp_std::fmt::Result {
		write!(f, "CheckAccount")
	}

	#[cfg(not(feature = "std"))]
	fn fmt(&self, _: &mut sp_std::fmt::Formatter) -> sp_std::fmt::Result {
		Ok(())
	}
}

impl SignedExtension for CheckAccount {
	type AccountId = AccountId;
	type Call = RuntimeCall;
	type AdditionalSigned = ();
	type Pre = ();
	const IDENTIFIER: &'static str = "CheckAccount";

	fn additional_signed(&self) -> sp_std::result::Result<(), TransactionValidityError> {
		Ok(())
	}

	fn pre_dispatch(
		self,
		_who: &Self::AccountId,
		call: &Self::Call,
		_info: &DispatchInfoOf<Self::Call>,
		_len: usize,
	) -> Result<(), TransactionValidityError> {
		match call {
			RuntimeCall::Appreciation(pallet_appreciation::Call::appreciation { to, .. }) =>
				if <Runtime as pallet_appreciation::Config>::IdentityProvider::exist_by_identity(to)
				{
					Ok(())
				} else {
					Err(InvalidTransaction::Custom(u8::MAX).into())
				},
			_ => Ok(()),
		}
	}

	fn validate(
		&self,
		who: &Self::AccountId,
		call: &Self::Call,
		_info: &DispatchInfoOf<Self::Call>,
		_len: usize,
	) -> TransactionValidity {
		match call {
			// In case this is `appreciation` transaction
			RuntimeCall::Appreciation(pallet_appreciation::Call::appreciation { to, .. }) =>
			// Check if the user is registered
				if <Runtime as pallet_appreciation::Config>::IdentityProvider::exist_by_identity(to)
				{
					// User already is registered, can execute transaction
					Ok(ValidTransaction::default())
				} else {
					pallet_appreciation::Referrals::<Runtime>::insert(who, to, ());
					// User is not registered need to provide tag to wait,
					// until `new_user` transaction provide this tag
					let requires = vec![Encode::encode(&(to))];
					Ok(ValidTransaction { requires, ..Default::default() })
				},
			// In case this is `new_user` transaction
			RuntimeCall::Identity(pallet_identity::Call::new_user {
				account_id,
				number,
				name,
				..
			}) => {
				let account_id_tag: AccountIdentityTag =
					AccountIdentity::AccountId(account_id.clone());
				let number_tag: AccountIdentityTag = AccountIdentity::PhoneNumber(number.clone());
				let name_tag: AccountIdentityTag = AccountIdentity::Name(name.clone());

				// This transaction provides tag, that may unlock some `appreciation` transactions
				let provides = vec![
					Encode::encode(&account_id_tag),
					Encode::encode(&number_tag),
					Encode::encode(&name_tag),
				];
				Ok(ValidTransaction { provides, ..Default::default() })
			},
			_ => Ok(ValidTransaction::default()),
		}
	}
}
