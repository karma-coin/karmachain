use crate::*;

impl pallet_authorship::Config for Runtime {
	/// Find the author of a block.
	type FindAuthor = pallet_session::FindAccountFromAuthorIndex<Self, Babe>;
	/// An event handler for authored blocks.
	type EventHandler = Staking;
}
