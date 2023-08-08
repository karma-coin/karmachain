#![cfg_attr(not(feature = "std"), no_std)]

pub mod hooks;
pub mod identity;
pub mod traits;
pub mod types;

use crate::traits::MaybeLowercase;
use codec::{Decode, Encode, EncodeLike, MaxEncodedLen};
use frame_support::{
	traits::Get, BoundedVec, CloneNoBound, DebugNoBound, DefaultNoBound, EqNoBound,
	PartialEqNoBound,
};
use scale_info::{
	prelude::{
		string::{FromUtf8Error, String},
		vec::Vec,
	},
	Type, TypeInfo,
};
#[cfg(feature = "std")]
use serde::{de, ser, Deserialize, Deserializer, Serialize, Serializer};

/// Always trimed
#[derive(
	DebugNoBound, CloneNoBound, EqNoBound, PartialEqNoBound, MaxEncodedLen, Encode, DefaultNoBound,
)]
pub struct BoundedString<MaxLength: Get<u32>>(BoundedVec<u8, MaxLength>);

impl<MaxLength: Get<u32>> TryFrom<Vec<u8>> for BoundedString<MaxLength> {
	type Error = &'static str;

	fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
		let binding = String::from_utf8(value).map_err(|_| "Invalid UTF-8 string")?;
		let string = binding.trim();

		BoundedVec::try_from(string.as_bytes().to_vec())
			.map(|v| BoundedString(v))
			.map_err(|_| "Out of bound. The length is too long.")
	}
}

impl<MaxLength: Get<u32>> From<BoundedString<MaxLength>> for Vec<u8> {
	fn from(value: BoundedString<MaxLength>) -> Self {
		value.0.into_inner()
	}
}

impl<MaxLength: Get<u32>> From<BoundedString<MaxLength>> for BoundedVec<u8, MaxLength> {
	fn from(value: BoundedString<MaxLength>) -> Self {
		value.0
	}
}

impl<MaxLength: Get<u32>> TryFrom<&str> for BoundedString<MaxLength> {
	type Error = &'static str;

	fn try_from(value: &str) -> Result<Self, Self::Error> {
		value.as_bytes().to_vec().try_into()
	}
}

impl<MaxLength: Get<u32>> TryFrom<String> for BoundedString<MaxLength> {
	type Error = &'static str;

	fn try_from(value: String) -> Result<Self, Self::Error> {
		value.as_bytes().to_vec().try_into()
	}
}

impl<MaxLength: Get<u32>> TryFrom<BoundedString<MaxLength>> for String {
	type Error = FromUtf8Error;

	fn try_from(value: BoundedString<MaxLength>) -> Result<Self, Self::Error> {
		String::from_utf8(value.0.into_inner())
	}
}

#[cfg(feature = "std")]
impl<MaxLength: Get<u32>> Serialize for BoundedString<MaxLength> {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		let bytes = self.0.clone().into_inner();
		let str = std::str::from_utf8(&bytes).map_err(ser::Error::custom)?;
		serializer.serialize_str(str)
	}
}

#[cfg(feature = "std")]
impl<'de, MaxLength: Get<u32>> Deserialize<'de> for BoundedString<MaxLength> {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>,
	{
		String::deserialize(deserializer)?
			.as_bytes()
			.to_vec()
			.try_into()
			.map_err(de::Error::custom)
	}
}

impl<MaxLength: Get<u32>> TypeInfo for BoundedString<MaxLength> {
	type Identity = str;

	fn type_info() -> Type {
		Self::Identity::type_info()
	}
}

impl<MaxLength: Get<u32>> Decode for BoundedString<MaxLength> {
	fn decode<I: codec::Input>(input: &mut I) -> Result<Self, codec::Error> {
		Vec::decode(input)?.try_into().map_err(|e: &str| e.into())
	}

	fn skip<I: codec::Input>(input: &mut I) -> Result<(), codec::Error> {
		Vec::<u8>::skip(input)
	}
}

// `BoundedString`s encode to something which will always decode as a `BoundedVec`.
impl<MaxLength: Get<u32>> EncodeLike<BoundedVec<u8, MaxLength>> for BoundedString<MaxLength> {}

impl<MaxLength: Get<u32>> PartialOrd<BoundedString<MaxLength>> for BoundedString<MaxLength> {
	fn partial_cmp(&self, other: &BoundedString<MaxLength>) -> Option<sp_std::cmp::Ordering> {
		self.0.partial_cmp(&other.0)
	}
}

impl<MaxLength: Get<u32>> PartialEq<String> for BoundedString<MaxLength> {
	fn eq(&self, other: &String) -> bool {
		String::try_from(self.clone()).map(|v| v.eq(other)).unwrap_or_default()
	}
}

impl<MaxLength: Get<u32>> PartialOrd<String> for BoundedString<MaxLength> {
	fn partial_cmp(&self, other: &String) -> Option<sp_std::cmp::Ordering> {
		String::try_from(self.clone()).ok().and_then(|v| v.partial_cmp(other))
	}
}

impl<MaxLength: Get<u32>> PartialEq<BoundedString<MaxLength>> for String {
	fn eq(&self, other: &BoundedString<MaxLength>) -> bool {
		String::try_from(other.clone()).map(|v| v.eq(self)).unwrap_or_default()
	}
}

impl<MaxLength: Get<u32>> PartialOrd<BoundedString<MaxLength>> for String {
	fn partial_cmp(&self, other: &BoundedString<MaxLength>) -> Option<sp_std::cmp::Ordering> {
		String::try_from(other.clone()).ok().and_then(|v| v.partial_cmp(self))
	}
}

impl<MaxLength: Get<u32>> Ord for BoundedString<MaxLength> {
	fn cmp(&self, other: &Self) -> sp_std::cmp::Ordering {
		self.0.cmp(&other.0)
	}
}

impl<MaxLength: Get<u32>> MaybeLowercase for BoundedString<MaxLength> {
	fn to_lowercase(self) -> Self {
		self.0
			.into_iter()
			.map(|b| b.to_ascii_lowercase())
			.collect::<Vec<_>>()
			.try_into()
			.unwrap()
	}
}

impl<MaxLength: Get<u32>> BoundedString<MaxLength> {
	pub fn as_slice(&self) -> &[u8] {
		self.0.as_slice()
	}
}
