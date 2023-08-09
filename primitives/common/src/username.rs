use crate::*;

/// Same as `BoundedString` but always trimed and casted to lowercase
#[derive(
	DebugNoBound, CloneNoBound, EqNoBound, PartialEqNoBound, MaxEncodedLen, Encode, DefaultNoBound,
)]
pub struct Username<MaxLength: Get<u32>>(BoundedString<MaxLength>);

impl<MaxLength: Get<u32>> TryFrom<Vec<u8>> for Username<MaxLength> {
	type Error = &'static str;

	fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
		let value = String::from_utf8(value).map_err(|_| "Invalid UTF-8")?;
		let value = value.trim().to_lowercase();

		BoundedString::try_from(value)
			.map(|v| Self(v))
			.map_err(|_| "Out of bound. The length is too long.")
	}
}

impl<MaxLength: Get<u32>> From<Username<MaxLength>> for Vec<u8> {
	fn from(value: Username<MaxLength>) -> Self {
		value.0.into()
	}
}

impl<MaxLength: Get<u32>> TryFrom<&str> for Username<MaxLength> {
	type Error = &'static str;

	fn try_from(value: &str) -> Result<Self, Self::Error> {
		value.trim().to_lowercase().try_into().map(|v| Self(v))
	}
}

impl<MaxLength: Get<u32>> TryFrom<String> for Username<MaxLength> {
	type Error = &'static str;

	fn try_from(value: String) -> Result<Self, Self::Error> {
		value.trim().to_lowercase().try_into().map(|v| Self(v))
	}
}

impl<MaxLength: Get<u32>> From<Username<MaxLength>> for String {
	fn from(value: Username<MaxLength>) -> Self {
		// Safety: Username is always valid UTF-8
		String::from_utf8(value.into()).unwrap()
	}
}

#[cfg(feature = "std")]
impl<MaxLength: Get<u32>> Serialize for Username<MaxLength> {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		let bytes: Vec<u8> = self.clone().into();
		let str = std::str::from_utf8(&bytes).map_err(ser::Error::custom)?;
		serializer.serialize_str(str)
	}
}

#[cfg(feature = "std")]
impl<'de, MaxLength: Get<u32>> Deserialize<'de> for Username<MaxLength> {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>,
	{
		String::deserialize(deserializer)?
			.trim()
			.to_lowercase()
			.as_bytes()
			.to_vec()
			.try_into()
			.map_err(de::Error::custom)
	}
}

impl<MaxLength: Get<u32>> TypeInfo for Username<MaxLength> {
	type Identity = str;

	fn type_info() -> Type {
		Self::Identity::type_info()
	}
}

impl<MaxLength: Get<u32>> Decode for Username<MaxLength> {
	fn decode<I: codec::Input>(input: &mut I) -> Result<Self, codec::Error> {
		Vec::decode(input)?.try_into().map_err(|e: &str| e.into())
	}

	fn skip<I: codec::Input>(input: &mut I) -> Result<(), codec::Error> {
		Vec::<u8>::skip(input)
	}
}

// `Username`s encode to something which will always decode as a `BoundedString`.
impl<MaxLength: Get<u32>> EncodeLike<BoundedString<MaxLength>> for Username<MaxLength> {}

impl<MaxLength: Get<u32>> PartialOrd<Username<MaxLength>> for Username<MaxLength> {
	fn partial_cmp(&self, other: &Username<MaxLength>) -> Option<sp_std::cmp::Ordering> {
		self.0.partial_cmp(&other.0)
	}
}

impl<MaxLength: Get<u32>> PartialEq<String> for Username<MaxLength> {
	fn eq(&self, other: &String) -> bool {
		String::try_from(self.clone()).map(|v| v.eq(other)).unwrap_or_default()
	}
}

impl<MaxLength: Get<u32>> PartialOrd<String> for Username<MaxLength> {
	fn partial_cmp(&self, other: &String) -> Option<sp_std::cmp::Ordering> {
		String::try_from(self.clone()).ok().and_then(|v| v.partial_cmp(other))
	}
}

impl<MaxLength: Get<u32>> PartialEq<Username<MaxLength>> for String {
	fn eq(&self, other: &Username<MaxLength>) -> bool {
		String::try_from(other.clone()).map(|v| v.eq(self)).unwrap_or_default()
	}
}

impl<MaxLength: Get<u32>> PartialOrd<Username<MaxLength>> for String {
	fn partial_cmp(&self, other: &Username<MaxLength>) -> Option<sp_std::cmp::Ordering> {
		String::try_from(other.clone()).ok().and_then(|v| v.partial_cmp(self))
	}
}

impl<MaxLength: Get<u32>> Ord for Username<MaxLength> {
	fn cmp(&self, other: &Self) -> sp_std::cmp::Ordering {
		self.0.cmp(&other.0)
	}
}

impl<MaxLength: Get<u32>> Username<MaxLength> {
	pub fn as_slice(&self) -> &[u8] {
		self.0.as_slice()
	}
}
