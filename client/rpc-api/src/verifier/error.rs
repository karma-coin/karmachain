use jsonrpsee::types::{error::CallError, ErrorObject};

/// Error type of this RPC api.
pub enum Error {
	Keystore,
}

impl From<Error> for i32 {
	fn from(e: Error) -> i32 {
		match e {
			Error::Keystore => 1,
		}
	}
}

// Convert custom error type to generic RPC error
pub fn map_err(error: impl ToString, desc: &'static str) -> CallError {
	CallError::Custom(ErrorObject::owned(Error::Keystore.into(), desc, Some(error.to_string())))
}
