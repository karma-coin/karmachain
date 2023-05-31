use crate::*;
use codec::{Decode, Encode};
use scale_info::TypeInfo;
use sp_runtime::{
	traits::{DispatchInfoOf, PostDispatchInfoOf, SignedExtension},
	transaction_validity::{
		InvalidTransaction, TransactionValidity, TransactionValidityError, ValidTransaction,
	},
	DispatchResult,
};
use sp_std::vec;

pub type AccountIdentityTag = AccountIdentity<
	<Runtime as frame_system::Config>::AccountId,
	<Runtime as pallet_identity::Config>::Username,
	<Runtime as pallet_identity::Config>::PhoneNumber,
>;

#[derive(Encode, Decode, Clone, Eq, PartialEq, TypeInfo)]
#[scale_info(skip_type_params(T))]
pub struct CheckAccount<T> {
	timestamp: u64,
	_marker: PhantomData<T>,
}

impl<T> CheckAccount<T> {
	pub fn new() -> Self {
		Self { timestamp: 0, _marker: Default::default() }
	}
}

impl<T> sp_std::fmt::Debug for CheckAccount<T> {
	#[cfg(feature = "std")]
	fn fmt(&self, f: &mut sp_std::fmt::Formatter) -> sp_std::fmt::Result {
		write!(f, "CheckAccount")
	}

	#[cfg(not(feature = "std"))]
	fn fmt(&self, _: &mut sp_std::fmt::Formatter) -> sp_std::fmt::Result {
		Ok(())
	}
}

impl<T> SignedExtension for CheckAccount<T>
where
	T: Send + Sync,
	T: pallet_appreciation::Config,
	T::IdentityProvider: IdentityProvider<AccountId, Username, PhoneNumber>,
	T: pallet_timestamp::Config<Moment = u64>,
{
	const IDENTIFIER: &'static str = "CheckAccount";
	type AccountId = AccountId;
	type Call = RuntimeCall;
	type AdditionalSigned = ();
	type Pre = (u64, RuntimeCall);

	fn additional_signed(&self) -> sp_std::result::Result<(), TransactionValidityError> {
		Ok(())
	}

	fn validate(
		&self,
		_who: &Self::AccountId,
		call: &Self::Call,
		_info: &DispatchInfoOf<Self::Call>,
		_len: usize,
	) -> TransactionValidity {
		// In case this is `appreciation` transaction
		if let Some(to) = call.map_appreciation() {
			// Check if the user is registered
			return if T::IdentityProvider::exist_by_identity(&to) {
				// User already is registered, can execute transaction
				Ok(ValidTransaction::default())
			} else {
				// User is not registered need to provide tag to wait,
				// until `new_user` transaction provide this tag
				let requires = vec![Encode::encode(&(to))];

				// These transactions should be stored in the pool for a period of 14 days
				// `longevity` time sets in blocks
				let longevity = 14 * DAYS;

				Ok(ValidTransaction { requires, longevity: longevity.into(), ..Default::default() })
			}
		}

		// In case this is `new_user` transaction
		if let Some((account_id, username, phone_number)) = call.map_new_user() {
			let account_id_tag: AccountIdentityTag = AccountIdentity::AccountId(account_id.clone());
			let number_tag: AccountIdentityTag = AccountIdentity::PhoneNumber(phone_number.clone());
			let name_tag: AccountIdentityTag = AccountIdentity::Name(username.clone());

			// This transaction provides tag, that may unlock some `appreciation` transactions
			let provides = vec![
				Encode::encode(&account_id_tag),
				Encode::encode(&number_tag),
				Encode::encode(&name_tag),
			];

			return Ok(ValidTransaction { provides, ..Default::default() })
		}

		Ok(ValidTransaction::default())
	}

	fn pre_dispatch(
		self,
		who: &Self::AccountId,
		call: &Self::Call,
		_info: &DispatchInfoOf<Self::Call>,
		_len: usize,
	) -> Result<Self::Pre, TransactionValidityError> {
		let now = self.timestamp;

		// In case this is `appreciation` transaction
		if let Some(to) = call.map_appreciation() {
			if T::IdentityProvider::exist_by_identity(&to) {
				let referral = Identity::get_registration_time(who)
					.map(|registration_time| registration_time <= now)
					.unwrap_or_default();

				Appreciation::set_referral_flag(referral);

				Ok((now, call.clone()))
			} else {
				Err(InvalidTransaction::Custom(u8::MAX).into())
			}
		} else {
			Ok((now, call.clone()))
		}
	}

	fn post_dispatch(
		pre: Option<Self::Pre>,
		_info: &DispatchInfoOf<Self::Call>,
		_post_info: &PostDispatchInfoOf<Self::Call>,
		_len: usize,
		result: &DispatchResult,
	) -> Result<(), TransactionValidityError> {
		if result.is_ok() && pre.is_some() {
			let (now, call) = pre.unwrap();
			if let Some((to, _, _)) = call.map_new_user() {
				if !Identity::set_registration_time(&to, now) {
					return Err(InvalidTransaction::Custom(u8::MAX).into())
				}
			}
		}

		Ok(())
	}
}
