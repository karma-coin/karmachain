use codec::{Decode, Encode};
use scale_info::{prelude::vec::Vec, TypeInfo};
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use sp_runtime::Perbill;

pub type EraIndex = u32;

/// A record of the nominations made by a specific account.
#[derive(PartialEq, Eq, Clone, Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
pub struct Nominations<AccountId> {
	/// The targets of nomination.
	pub targets: Vec<AccountId>,
	/// The era the nominations were submitted.
	///
	/// Except for initial nominations which are considered submitted at era 0.
	pub submitted_in: EraIndex,
	/// Whether the nominations have been suppressed. This can happen due to slashing of the
	/// validators, or other events that might invalidate the nomination.
	///
	/// NOTE: this for future proofing and is thus far not used.
	pub suppressed: bool,
}

impl<T: pallet_staking::Config> From<pallet_staking::Nominations<T>> for Nominations<T::AccountId> {
	fn from(nominations: pallet_staking::Nominations<T>) -> Self {
		Self {
			targets: nominations.targets.into(),
			submitted_in: nominations.submitted_in,
			suppressed: nominations.suppressed,
		}
	}
}

/// Preference of what happens regarding validation.
#[derive(PartialEq, Eq, Clone, Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
pub struct ValidatorPrefs<AccountId> {
	/// Validator account id.
	pub account_id: AccountId,
	/// Reward that validator takes up-front; only the rest is split between themselves and
	/// nominators.
	#[codec(compact)]
	pub commission: Perbill,
	/// Whether or not this validator is accepting more nominations. If `true`, then no nominator
	/// who is not already nominating this validator may nominate them. By default, validators
	/// are accepting nominations.
	pub blocked: bool,
}

impl<AccountId> From<(AccountId, pallet_staking::ValidatorPrefs)> for ValidatorPrefs<AccountId> {
	fn from((account_id, validator_prefs): (AccountId, pallet_staking::ValidatorPrefs)) -> Self {
		Self {
			account_id,
			commission: validator_prefs.commission,
			blocked: validator_prefs.blocked,
		}
	}
}
