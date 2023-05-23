use jsonrpsee::types::{error::CallError, ErrorObject};

/// Error type of this RPC api.
pub enum Error {
	/// No verifier keys found.`Verifier` node is not run properly,
	/// use RPC call `author_insertKey` to setup keys
	KeyNotFound,
	/// Internal `SyncCryptoStore` error
	SignatureFailed,
	/// Invalid bypass token passed
	BypassTokenMismatch,
	/// Failed to connect to `AuthService`
	AuthServiceOffline,
	/// Passed number cannot be cast to string
	InvalidString,
	/// The call to runtime failed
	RuntimeError,
}

impl From<Error> for i32 {
	fn from(e: Error) -> i32 {
		match e {
			Error::KeyNotFound => 1,
			Error::SignatureFailed => 2,
			Error::BypassTokenMismatch => 3,
			Error::AuthServiceOffline => 4,
			Error::InvalidString => 5,
			Error::RuntimeError => 6,
		}
	}
}

impl ToString for Error {
	fn to_string(&self) -> String {
		match self {
			Error::KeyNotFound => "Verifier keys not found",
			Error::SignatureFailed => "Failed to sign",
			Error::BypassTokenMismatch => "Invalid bypass token passed",
			Error::AuthServiceOffline => "Failed to connect to auth service",
			Error::InvalidString => "Passed number cannot be cast to string",
			Error::RuntimeError => "The call to runtime failed",
		}
		.to_owned()
	}
}

// Convert custom error type to generic RPC error
pub fn map_err(error: Error, data: impl ToString) -> CallError {
	let desc = error.to_string();
	CallError::Custom(ErrorObject::owned(error.into(), desc, Some(data.to_string())))
}
