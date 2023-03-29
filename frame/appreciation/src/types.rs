use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::{traits::Get, BoundedVec, CloneNoBound, RuntimeDebugNoBound};
use sp_std::vec::Vec;
use scale_info::TypeInfo;

#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

pub type CharTraitId = u32;
pub type CommunityId = u32;
pub type Score = u32;

pub type GenesisCommunity =
	(CommunityId, Vec<u8>, Vec<u8>, Vec<u8>, Vec<u8>, Vec<u8>, Vec<u8>, Vec<u8>, Vec<u8>, bool);

#[derive(CloneNoBound, Encode, Decode, MaxEncodedLen, TypeInfo, RuntimeDebugNoBound)]
#[codec(mel_bound())]
#[scale_info(skip_type_params(CharNameLimit))]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct CharTrait<CharNameLimit: Get<u32>> {
	pub id: CharTraitId,
	pub name: BoundedVec<u8, CharNameLimit>,
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
#[scale_info(skip_type_params(NameLimit, DescLimit, EmojiLimit, UrlLimit))]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct Community<
	NameLimit: Get<u32>,
	DescLimit: Get<u32>,
	EmojiLimit: Get<u32>,
	UrlLimit: Get<u32>,
> {
	pub id: CommunityId,
	pub name: BoundedVec<u8, NameLimit>,
	pub desc: BoundedVec<u8, DescLimit>,
	pub emoji: BoundedVec<u8, EmojiLimit>,
	pub website_url: BoundedVec<u8, UrlLimit>,
	pub twitter_url: BoundedVec<u8, UrlLimit>,
	pub insta_url: BoundedVec<u8, UrlLimit>,
	pub face_url: BoundedVec<u8, UrlLimit>,
	pub discord_url: BoundedVec<u8, UrlLimit>,
	/// Closed community - only community manager can invite new members
	/// and only members can appreciate each other in the community
	pub closed: bool,
}
