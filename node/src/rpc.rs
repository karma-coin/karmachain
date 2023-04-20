//! A collection of node-specific RPC methods.
//! Substrate provides the `sc-rpc` crate, which defines the core RPC layer
//! used by Substrate nodes. This file extends those RPC definitions with
//! capabilities that are specific to this project's runtime configuration.

#![warn(missing_docs)]

use std::sync::Arc;

use jsonrpsee::RpcModule;
use karmachain_node_runtime::{
	opaque::{Block, UncheckedExtrinsic},
	AccountId, Balance, Index, RuntimeEvent, Signature, Hash,
};
use sc_client_api::BlockBackend;
use sc_transaction_pool_api::TransactionPool;
use sp_api::ProvideRuntimeApi;
use sp_block_builder::BlockBuilder;
use sp_blockchain::{Error as BlockChainError, HeaderBackend, HeaderMetadata};

pub use sc_rpc_api::DenyUnsafe;
use sp_runtime::generic::SignedBlock;

/// Full client dependencies.
pub struct FullDeps<C, P> {
	/// The client instance to use.
	pub client: Arc<C>,
	/// Transaction pool instance.
	pub pool: Arc<P>,
	/// Whether to deny unsafe calls
	pub deny_unsafe: DenyUnsafe,
}

/// Instantiate all full RPC extensions.
pub fn create_full<C, P>(
	deps: FullDeps<C, P>,
) -> Result<RpcModule<()>, Box<dyn std::error::Error + Send + Sync>>
where
	C: ProvideRuntimeApi<Block>,
	C: HeaderBackend<Block> + HeaderMetadata<Block, Error = BlockChainError> + 'static,
	C: Send + Sync + 'static,
	C: BlockBackend<Block>,
	C::Api: substrate_frame_rpc_system::AccountNonceApi<Block, AccountId, Index>,
	C::Api: pallet_transaction_payment_rpc::TransactionPaymentRuntimeApi<Block, Balance>,
	C::Api: BlockBuilder<Block>,
	C::Api: pallet_identity_rpc::IdentityRuntimeApi<Block, AccountId>,
	C::Api: runtime_api::transactions::TransactionInfoProvider<
		Block,
		UncheckedExtrinsic,
		AccountId,
		Signature,
	>,
	C::Api: runtime_api::transactions::TransactionIndexer<Block, AccountId>,
	C::Api: runtime_api::events::EventProvider<Block, RuntimeEvent>,
	C::Api:
		runtime_api::chain::BlockInfoProvider<Block, SignedBlock<Block>, AccountId, Hash>,
	P: TransactionPool + 'static,
{
	use pallet_identity_rpc::{Identity, IdentityApiServer};
	use pallet_transaction_payment_rpc::{TransactionPayment, TransactionPaymentApiServer};
	use rpc_api::{
		chain::{client::BlocksProvider, BlocksProviderApiServer},
		events::{client::EventsProvider, EventsProviderApiServer},
		transactions::{client::TransactionsIndexer, TransactionsIndexerApiServer},
	};
	use substrate_frame_rpc_system::{System, SystemApiServer};

	let mut module = RpcModule::new(());
	let FullDeps { client, pool, deny_unsafe } = deps;

	module.merge(System::new(client.clone(), pool, deny_unsafe).into_rpc())?;
	module.merge(TransactionPayment::new(client.clone()).into_rpc())?;
	module.merge(Identity::new(client.clone()).into_rpc())?;
	module.merge(TransactionsIndexer::new(client.clone()).into_rpc())?;
	module.merge(EventsProvider::new(client.clone()).into_rpc())?;
	module.merge(BlocksProvider::new(client.clone()).into_rpc())?;

	Ok(module)
}
