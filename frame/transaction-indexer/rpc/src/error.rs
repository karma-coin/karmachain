use jsonrpsee::types::error::{CallError, ErrorObject};

/// Error type of this RPC api.
pub enum Error {
	/// The call to runtime failed.
	RuntimeError,
	/// Failed to get block details
	BlockNotFound,
	/// Failed to get tx
	TxNotFound,
}

impl From<Error> for i32 {
	fn from(e: Error) -> i32 {
		match e {
			Error::RuntimeError => 1,
			Error::BlockNotFound => 2,
			Error::TxNotFound => 3,
		}
	}
}

// Convert custom error type to generic RPC error
pub fn map_err(error: impl ToString, desc: &'static str) -> CallError {
	CallError::Custom(ErrorObject::owned(Error::RuntimeError.into(), desc, Some(error.to_string())))
}
