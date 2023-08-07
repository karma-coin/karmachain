mod utils;

use crate::{
	chain::{get_blockchain_data, get_genesis_data},
	utils::create_runner,
};
use karmachain_node::cli::Cli;
use sc_cli::SubstrateCli;

/// Test API that provide info about current chain state
/// or specified block
/// `get_block_info`, `get_blockchain_data`, `get_genesis_data`
mod chain {
	use crate::utils::{RpcResponse, RPC_URL};
	use karmachain_node_runtime::AccountId;
	use serde_json::json;
	use sp_core::crypto::Ss58Codec;
	use sp_rpc::{BlockchainStats, GenesisData};

	pub async fn get_blockchain_data() -> Result<(), ()> {
		let client = reqwest::Client::new();

		let _response = client
			.post(RPC_URL)
			.json(&json!({
				"id": 1,
				"jsonrpc": "2.0",
				"method": "chain_getBlockchainData",
			}))
			.send()
			.await
			.unwrap()
			.json::<RpcResponse<BlockchainStats>>()
			.await
			.unwrap();

		Ok(())
	}

	pub async fn get_genesis_data() -> Result<(), ()> {
		let client = reqwest::Client::new();

		let response = client
			.post(RPC_URL)
			.json(&json!({
				"id": 1,
				"jsonrpc": "2.0",
				"method": "chain_getGenesisData",
			}))
			.send()
			.await
			.unwrap()
			.json::<RpcResponse<GenesisData<AccountId>>>()
			.await
			.unwrap();

		let alice =
			AccountId::from_string("5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY").unwrap();
		assert!(response.result.unwrap().verifiers.iter().any(|value| value.account_id == alice));

		Ok(())
	}
}

// Using one test for many tests causes because of issue with `create_runner`
// also this safe time (spend time only to run one node)
#[tokio::test(flavor = "multi_thread")]
async fn rpc_tests() -> Result<(), ()> {
	let mut cli = Cli::from_args();
	// Setup chain in developer mode
	cli.run.shared_params.dev = true;

	create_runner(cli, async move {
		get_blockchain_data().await.expect("Get blockchain data fails");
		get_genesis_data().await.expect("Get genesis data fails");

		Ok(())
	})
	.await
}
