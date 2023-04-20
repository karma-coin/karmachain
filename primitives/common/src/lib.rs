#![cfg_attr(not(feature = "std"), no_std)]

pub mod hooks;
pub mod identity;
pub mod traits;
pub mod types;

use codec::{Decode, Encode};
use frame_support::{traits::Get, BoundedVec};
use scale_info::prelude::vec::Vec;

#[cfg(feature = "std")]
use serde::{ser, Deserialize, Deserializer, Serialize, Serializer};

#[derive(Clone, Encode, Decode)]
pub struct String(pub Vec<u8>);

impl From<Vec<u8>> for String {
	fn from(v: Vec<u8>) -> String {
		String(v)
	}
}

impl From<String> for Vec<u8> {
	fn from(v: String) -> Vec<u8> {
		v.0
	}
}

#[cfg(feature = "std")]
impl From<std::string::String> for String {
	fn from(v: std::string::String) -> String {
		String(v.as_bytes().to_vec())
	}
}

#[cfg(feature = "std")]
impl From<String> for std::string::String {
	fn from(v: String) -> std::string::String {
		std::str::from_utf8(&v.0).unwrap_or_default().into()
	}
}

impl From<&str> for String {
	fn from(v: &str) -> String {
		String(v.as_bytes().to_vec())
	}
}

impl<S: Get<u32>> TryFrom<String> for BoundedVec<u8, S> {
	type Error = Vec<u8>;
	fn try_from(t: String) -> Result<Self, Self::Error> {
		t.0.try_into()
	}
}

#[cfg(feature = "std")]
impl Serialize for String {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		serializer.serialize_str(std::str::from_utf8(&self.0).map_err(ser::Error::custom)?)
	}
}

#[cfg(feature = "std")]
impl<'de> Deserialize<'de> for String {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>,
	{
		Ok(Self(std::string::String::deserialize(deserializer)?.as_bytes().to_vec()))
	}
}
