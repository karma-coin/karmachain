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
