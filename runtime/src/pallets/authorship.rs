use crate::*;

parameter_types! {
	pub const UncleGenerations: u32 = 0;
}

impl pallet_authorship::Config for Runtime {
	/// Find the author of a block.
	type FindAuthor = pallet_session::FindAccountFromAuthorIndex<Self, Babe>;
	/// An event handler for authored blocks.
	type EventHandler = Staking;
}
