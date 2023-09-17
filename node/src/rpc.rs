//! A collection of node-specific RPC methods.
//! Substrate provides the `sc-rpc` crate, which defines the core RPC layer
//! used by Substrate nodes. This file extends those RPC definitions with
//! capabilities that are specific to this project's runtime configuration.

#![warn(missing_docs)]

use crate::service::FullBackend;
use frame_system::EventRecord;
use jsonrpsee::RpcModule;
use karmachain_node_runtime::{
	opaque::{Block, UncheckedExtrinsic},
	AccountId, Balance, BlockNumber, Hash, Index, PhoneNumber, PhoneNumberHash, RuntimeEvent,
	Signature, Username,
};
use sc_client_api::{BlockBackend, StorageProvider};
pub use sc_rpc_api::DenyUnsafe;
use sc_transaction_pool_api::TransactionPool;
use sp_api::ProvideRuntimeApi;
use sp_block_builder::BlockBuilder;
use sp_blockchain::{Error as BlockChainError, HeaderBackend, HeaderMetadata};
use sp_consensus::SelectChain;
use sp_consensus_babe::BabeApi;
use sp_keystore::KeystorePtr;
use sp_runtime::generic::SignedBlock;
use std::sync::Arc;

/// Extra dependencies for BABE.
pub struct BabeDeps {
	/// A handle to the BABE worker for issuing requests.
	pub babe_worker_handle: sc_consensus_babe::BabeWorkerHandle<Block>,
	/// The keystore that manages the keys of the node.
	pub keystore: KeystorePtr,
}

/// Full client dependencies.
pub struct FullDeps<C, P, SC> {
	/// The client instance to use.
	pub client: Arc<C>,
	/// Transaction pool instance.
	pub pool: Arc<P>,
	/// The [`SelectChain`] Strategy
	pub select_chain: SC,
	/// Whether to deny unsafe calls
	pub deny_unsafe: DenyUnsafe,
	/// BABE specific dependencies.
	pub babe: BabeDeps,
}

/// Instantiate all full RPC extensions.
pub fn create_full<C, P, SC>(
	deps: FullDeps<C, P, SC>,
	network_id: String,
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
	C::Api: runtime_api::chain::ChainDataProvider<Block, SignedBlock<Block>, AccountId, Hash>,
	C::Api: runtime_api::events::EventProvider<Block, EventRecord<RuntimeEvent, Hash>>,
	C::Api: runtime_api::identity::IdentityApi<Block, AccountId, Username, PhoneNumberHash>,
	C::Api: runtime_api::transactions::TransactionInfoProvider<
		Block,
		UncheckedExtrinsic,
		AccountId,
		Signature,
		EventRecord<RuntimeEvent, Hash>,
	>,
	C::Api:
		runtime_api::nomination_pools::NominationPoolsApi<Block, AccountId, Balance, BlockNumber>,
	C::Api: runtime_api::staking::StakingApi<Block, AccountId>,
	C::Api: runtime_api::transactions::TransactionIndexer<Block, AccountId, PhoneNumberHash>,
	C::Api: BabeApi<Block>,
	P: TransactionPool + 'static,
	SC: SelectChain<Block> + 'static,
{
	use pallet_transaction_payment_rpc::{TransactionPayment, TransactionPaymentApiServer};
	use rpc_api::{
		chain::{client::ChainDataProvider, ChainDataProviderApiServer},
		events::{client::EventsProvider, EventsProviderApiServer},
		identity::{client::Identity, IdentityApiServer},
		nomination_pools::{client::NominationPools, NominationPoolsApiServer},
		staking::{client::Staking, StakingApiServer},
		transactions::{client::TransactionsIndexer, TransactionsIndexerApiServer},
	};
	use sc_consensus_babe_rpc::{Babe, BabeApiServer};
	use substrate_frame_rpc_system::{System, SystemApiServer};

	let mut module = RpcModule::new(());
	let FullDeps { client, pool, select_chain, deny_unsafe, babe } = deps;
	let BabeDeps { babe_worker_handle, keystore } = babe;

	module.merge(System::new(client.clone(), pool, deny_unsafe).into_rpc())?;
	module.merge(TransactionPayment::new(client.clone()).into_rpc())?;
	module.merge(
		Babe::new(
			client.clone(),
			babe_worker_handle.clone(),
			keystore.clone(),
			select_chain,
			deny_unsafe,
		)
		.into_rpc(),
	)?;

	module.merge(
		IdentityApiServer::<Hash, AccountId, Username, PhoneNumber, PhoneNumberHash>::into_rpc(
			Identity::new(client.clone()),
		),
	)?;
	module.merge(TransactionsIndexer::new(client.clone()).into_rpc())?;
	module.merge(EventsProvider::new(client.clone()).into_rpc())?;
	module.merge(ChainDataProvider::new(client.clone(), network_id).into_rpc())?;
	module.merge(NominationPools::new(client.clone()).into_rpc())?;
	module.merge(Staking::new(client.clone()).into_rpc())?;

	Ok(module)
}
