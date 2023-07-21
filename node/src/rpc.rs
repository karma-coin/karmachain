//! A collection of node-specific RPC methods.
//! Substrate provides the `sc-rpc` crate, which defines the core RPC layer
//! used by Substrate nodes. This file extends those RPC definitions with
//! capabilities that are specific to this project's runtime configuration.

#![warn(missing_docs)]

use std::sync::Arc;

use frame_system::EventRecord;
use jsonrpsee::RpcModule;
use karmachain_node_runtime::{
	opaque::{Block, UncheckedExtrinsic},
	AccountId, Balance, Hash, Index, PhoneNumber, PhoneNumberHash, RuntimeEvent, Signature,
	Username,
};
use sc_client_api::{BlockBackend, StorageProvider};
pub use sc_rpc_api::DenyUnsafe;
use sc_transaction_pool_api::TransactionPool;
use sp_api::ProvideRuntimeApi;
use sp_block_builder::BlockBuilder;
use sp_blockchain::{Error as BlockChainError, HeaderBackend, HeaderMetadata};
use sp_keystore::SyncCryptoStore;
use sp_rpc::ByPassToken;
use sp_runtime::generic::SignedBlock;

use crate::service::FullBackend;

/// Full client dependencies.
pub struct FullDeps<C, P> {
	/// The client instance to use.
	pub client: Arc<C>,
	/// Transaction pool instance.
	pub pool: Arc<P>,
	/// Whether to deny unsafe calls
	pub deny_unsafe: DenyUnsafe,
	/// The keystore instance to use
	pub keystore: Arc<dyn SyncCryptoStore>,
}

/// Instantiate all full RPC extensions.
pub fn create_full<C, P>(
	deps: FullDeps<C, P>,
	verifier: bool,
	bypass_token: Option<ByPassToken>,
	auth_dst: Option<String>,
) -> Result<RpcModule<()>, Box<dyn std::error::Error + Send + Sync>>
where
	C: ProvideRuntimeApi<Block>,
	C: HeaderBackend<Block> + HeaderMetadata<Block, Error = BlockChainError> + 'static,
	C: Send + Sync + 'static,
	C: BlockBackend<Block>,
	C: StorageProvider<Block, FullBackend>,
	C::Api: substrate_frame_rpc_system::AccountNonceApi<Block, AccountId, Index>,
	C::Api: pallet_transaction_payment_rpc::TransactionPaymentRuntimeApi<Block, Balance>,
	C::Api: BlockBuilder<Block>,
	C::Api: runtime_api::chain::BlockInfoProvider<Block, SignedBlock<Block>, AccountId, Hash>,
	C::Api: runtime_api::events::EventProvider<Block, EventRecord<RuntimeEvent, Hash>>,
	C::Api: runtime_api::identity::IdentityApi<Block, AccountId, Username, PhoneNumberHash>,
	C::Api: runtime_api::transactions::TransactionInfoProvider<
		Block,
		UncheckedExtrinsic,
		AccountId,
		Signature,
		EventRecord<RuntimeEvent, Hash>,
	>,
	C::Api: runtime_api::transactions::TransactionIndexer<Block, AccountId, PhoneNumberHash>,
	C::Api: runtime_api::verifier::VerifierApi<Block, AccountId, Username, PhoneNumberHash>,
	P: TransactionPool + 'static,
{
	use pallet_transaction_payment_rpc::{TransactionPayment, TransactionPaymentApiServer};
	use rpc_api::{
		chain::{client::BlocksProvider, BlocksProviderApiServer},
		events::{client::EventsProvider, EventsProviderApiServer},
		identity::{client::Identity, IdentityApiServer},
		transactions::{client::TransactionsIndexer, TransactionsIndexerApiServer},
		verifier::{client::Verifier, VerifierApiServer},
	};
	use substrate_frame_rpc_system::{System, SystemApiServer};

	let mut module = RpcModule::new(());
	let FullDeps { client, pool, deny_unsafe, keystore } = deps;

	module.merge(System::new(client.clone(), pool, deny_unsafe).into_rpc())?;
	module.merge(TransactionPayment::new(client.clone()).into_rpc())?;
	module.merge(
		IdentityApiServer::<Hash, AccountId, Username, PhoneNumber, PhoneNumberHash>::into_rpc(
			Identity::new(client.clone()),
		),
	)?;
	module.merge(TransactionsIndexer::new(client.clone()).into_rpc())?;
	module.merge(EventsProvider::new(client.clone()).into_rpc())?;
	module.merge(BlocksProvider::new(client.clone()).into_rpc())?;

	if verifier {
		// TODO: better way to handle error
		let bypass_token = bypass_token.expect("Missing bypass token");
		let auth_dst = auth_dst.expect("Missing auth endpoint url");

		module.merge(
			VerifierApiServer::<AccountId, Username, PhoneNumber, PhoneNumberHash>::into_rpc(
				Verifier::new(client, keystore, bypass_token, auth_dst),
			),
		)?;
	}

	Ok(module)
}
