use std::sync::Arc;

use codec::Codec;
use jsonrpsee::{
	core::RpcResult,
	proc_macros::rpc,
	types::error::{CallError, ErrorObject},
};
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_runtime::{generic::BlockId, traits::Block as BlockT};

pub use pallet_identity_rpc_runtime_api::{IdentityApi as IdentityRuntimeApi, UserInfo};




