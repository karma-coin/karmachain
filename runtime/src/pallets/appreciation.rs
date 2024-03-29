use crate::*;

parameter_types! {
	pub const MaxCharTrait: u32 = 100;
	pub const CharNameLimit: u32 = 25;
	pub const CommunityNameLimit: u32 = 25;
	pub const CommunityDescriptionLimit: u32 = 100;
	pub const EmojiLimit: u32 = 20;
	pub const CommunityUrlLimit: u32 = 100;
	pub const MaxCommunities: u32 = 1000;
}

impl pallet_appreciation::Config for Runtime {
	/// The overarching event type.
	type RuntimeEvent = RuntimeEvent;
	/// Max number of `CharTrait`
	type MaxCharTrait = MaxCharTrait;
	/// Max length of `CharTrait`'s name
	type CharNameLimit = CharNameLimit;
	/// Max number of `Communities`
	type MaxCommunities = MaxCommunities;
	/// Max length of `Community`'s name
	type CommunityNameLimit = CommunityNameLimit;
	/// Max length of `Community`'s description
	type CommunityDescriptionLimit = CommunityDescriptionLimit;
	/// Max length of `Community`'s emoji
	type EmojiLimit = EmojiLimit;
	/// Max length of `Community`'s urls
	type CommunityUrlLimit = CommunityUrlLimit;

	type IdentityProvider = Identity;
}
