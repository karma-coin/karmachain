use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::{traits::Get, BoundedVec, CloneNoBound, RuntimeDebugNoBound};
use scale_info::TypeInfo;
use sp_std::vec::Vec;
use scale_info::prelude::string::String;

#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use sp_common::{
	types::{CharTraitId, CommunityId},
	BoundedString,
};

pub type Score = u32;

pub type GenesisCommunity = (
	CommunityId,
	String,
	String,
	String,
	String,
	String,
	String,
	String,
	String,
	Vec<CharTraitId>,
	bool,
);

#[derive(CloneNoBound, Encode, Decode, MaxEncodedLen, TypeInfo, RuntimeDebugNoBound)]
#[codec(mel_bound())]
#[scale_info(skip_type_params(CharNameLimit, EmojiLimit))]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct CharTrait<CharNameLimit: Get<u32>, EmojiLimit: Get<u32>> {
	pub id: CharTraitId,
	pub name: BoundedString<CharNameLimit>,
	pub emoji: BoundedString<EmojiLimit>,
}

#[derive(Encode, Decode, MaxEncodedLen, TypeInfo, RuntimeDebugNoBound, Default)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum CommunityRole {
	Admin,
	Member,
	#[default]
	None,
}

impl CommunityRole {
	pub fn is_admin(&self) -> bool {
		matches!(self, Self::Admin)
	}
}

#[derive(CloneNoBound, Encode, Decode, MaxEncodedLen, TypeInfo, RuntimeDebugNoBound)]
#[codec(mel_bound())]
#[scale_info(skip_type_params(NameLimit, DescLimit, EmojiLimit, UrlLimit, MaxCharTrait))]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct Community<
	NameLimit: Get<u32>,
	DescLimit: Get<u32>,
	EmojiLimit: Get<u32>,
	UrlLimit: Get<u32>,
	MaxCharTrait: Get<u32>,
> {
	pub id: CommunityId,
	pub name: BoundedString<NameLimit>,
	pub desc: BoundedString<DescLimit>,
	pub emoji: BoundedString<EmojiLimit>,
	pub website_url: BoundedString<UrlLimit>,
	pub twitter_url: BoundedString<UrlLimit>,
	pub insta_url: BoundedString<UrlLimit>,
	pub face_url: BoundedString<UrlLimit>,
	pub discord_url: BoundedString<UrlLimit>,
	pub char_traits: BoundedVec<CharTraitId, MaxCharTrait>,
	/// Closed community - only community manager can invite new members
	/// and only members can appreciate each other in the community
	pub closed: bool,
}
