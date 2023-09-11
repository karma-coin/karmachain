use crate::*;

pub struct OnChainSeqPhragmen;
impl onchain::Config for OnChainSeqPhragmen {
	type System = Runtime;
	type Solver = SequentialPhragmen<AccountId, sp_runtime::Perbill>;
	type DataProvider = Staking;
	type WeightInfo = frame_election_provider_support::weights::SubstrateWeight<Runtime>;
	type MaxWinners = MaxActiveValidators;
	type VotersBound = MaxElectingVoters;
	type TargetsBound = MaxElectableTargets;
}

pub struct EraPayout<T>(PhantomData<T>);
impl pallet_staking::EraPayout<Balance> for EraPayout<Staking> {
	fn era_payout(
		_total_staked: Balance,
		_total_issuance: Balance,
		_era_duration_millis: u64,
	) -> (Balance, Balance) {
		let era_index = Staking::active_era().unwrap().index;

		let payout = validators_rewards::era_payout(era_index);
		let rest = 0;

		(payout, rest)
	}
}

// Impls Serialize for event type
#[cfg(feature = "std")]
impl serde::Serialize for RuntimeEvent {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		// Serialize the `RuntimeEvent` type as a string
		serializer.serialize_str(&format!("{self:?}"))
	}
}

impl RuntimeCall {
	/// Get `AccountIdentity` of recipient of the transaction
	pub fn get_recipient(&self) -> Option<types::AccountIdentity> {
		match self {
			RuntimeCall::Appreciation(pallet_appreciation::Call::appreciation { to, .. }) =>
				Some(to.clone()),
			RuntimeCall::Identity(pallet_identity::Call::new_user { account_id, .. }) =>
				Some(AccountIdentity::AccountId(account_id.clone())),
			// TODO: cover more cases
			_ => None,
		}
	}

	pub fn map_appreciation(&self) -> Option<types::AccountIdentity> {
		match self {
			RuntimeCall::Appreciation(pallet_appreciation::Call::appreciation { to, .. }) =>
				Some(to.clone()),
			_ => None,
		}
	}

	pub fn map_new_user(&self) -> Option<(AccountId, Username, PhoneNumberHash)> {
		match self {
			RuntimeCall::Identity(pallet_identity::Call::new_user {
				account_id,
				phone_number_hash,
				username,
				..
			}) => Some((account_id.clone(), username.clone(), *phone_number_hash)),
			_ => None,
		}
	}
}

/// Convert a balance to an unsigned 256-bit number, use in nomination pools.
pub struct BalanceToU256;
impl sp_runtime::traits::Convert<Balance, sp_core::U256> for BalanceToU256 {
	fn convert(n: Balance) -> sp_core::U256 {
		n.into()
	}
}

/// Convert an unsigned 256-bit number to balance, use in nomination pools.
pub struct U256ToBalance;
impl sp_runtime::traits::Convert<sp_core::U256, Balance> for U256ToBalance {
	fn convert(n: sp_core::U256) -> Balance {
		use frame_support::traits::Defensive;
		n.try_into().defensive_unwrap_or(Balance::MAX)
	}
}

/// Macro to set a value (e.g. when using the `parameter_types` macro) to either a production value
/// or to an environment variable or testing value (in case the `fast-runtime` feature is selected).
/// Note that the environment variable is evaluated _at compile time_.
///
/// Usage:
/// ```Rust
/// parameter_types! {
/// 	// Note that the env variable version parameter cannot be const.
/// 	pub LaunchPeriod: BlockNumber = prod_or_fast!(7 * DAYS, 1, "KSM_LAUNCH_PERIOD");
/// 	pub const VotingPeriod: BlockNumber = prod_or_fast!(7 * DAYS, 1 * MINUTES);
/// }
/// ```
#[macro_export]
macro_rules! prod_or_fast {
	($prod:expr, $test:expr) => {
		if cfg!(feature = "fast-runtime") {
			$test
		} else {
			$prod
		}
	};
	($prod:expr, $test:expr, $env:expr) => {
		if cfg!(feature = "fast-runtime") {
			core::option_env!($env).map(|s| s.parse().ok()).flatten().unwrap_or($test)
		} else {
			$prod
		}
	};
}
