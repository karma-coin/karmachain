use crate::*;

parameter_types! {
	pub const UncleGenerations: u32 = 0;
}

impl pallet_authorship::Config for Runtime {
	/// Find the author of a block.
	type FindAuthor = pallet_session::FindAccountFromAuthorIndex<Self, Babe>;
	/// The number of blocks back we should accept uncles.
	/// This means that we will deal with uncle-parents that are
	/// `UncleGenerations + 1` before `now`.
	type UncleGenerations = UncleGenerations;
	/// A filter for uncles within a block. This is for implementing
	/// further constraints on what uncles can be included, other than their ancestry.
	///
	/// For PoW, as long as the seals are checked, there is no need to use anything
	/// but the `VerifySeal` implementation as the filter. This is because the cost of making
	/// many equivocating uncles is high.
	///
	/// For PoS, there is no such limitation, so a further constraint must be imposed
	/// beyond a seal check in order to prevent an arbitrary number of
	/// equivocating uncles from being included.
	///
	/// The `OnePerAuthorPerHeight` filter is good for many slot-based PoS
	/// engines.
	type FilterUncle = ();
	/// An event handler for authored blocks.
	type EventHandler = Staking;
}
