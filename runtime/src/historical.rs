use crate::*;

impl pallet_session::historical::Config for Runtime {
	/// Full identification of the validator.
	type FullIdentification = pallet_staking::Exposure<AccountId, Balance>;
	/// A conversion from validator ID to full identification.
	///
	/// This should contain any references to economic actors associated with the
	/// validator, since they may be outdated by the time this is queried from a
	/// historical trie.
	///
	/// It must return the identification for the current session index.
	type FullIdentificationOf = pallet_staking::ExposureOf<Runtime>;
}