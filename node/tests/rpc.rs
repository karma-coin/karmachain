mod utils;

use crate::{
	chain::{get_block_info, get_blockchain_data, get_genesis_data},
	utils::create_runner,
};

/// Test API that provide info about current chain state
/// or specified block
/// `get_block_info`, `get_blockchain_data`, `get_genesis_data`
mod chain {
	use karmachain_node_runtime::{AccountId, Hash};
	
	use serde_json::{json, Value};
	use sp_core::crypto::Ss58Codec;
	use sp_rpc::{Block, BlockchainStats, GenesisData};

	const URL: &str = "http://localhost:9933/";

	pub async fn get_block_info() -> Result<(), ()> {
		let client = reqwest::Client::new();

		let response = client
			.post(URL)
			.json(&json!({
				"id": 1,
				"jsonrpc": "2.0",
				"method": "chain_getBlockInfo",
				"params": {
					"block_height": 1
				}
			}))
			.send()
			.await
			.unwrap()
			.json::<Value>()
			.await
			.unwrap();

		let response: Block<AccountId, Hash> =
			serde_json::from_value(response.get("result").unwrap().clone()).unwrap();
		assert_eq!(response.height, 1);

		Ok(())
	}

	pub async fn get_blockchain_data() -> Result<(), ()> {
		let client = reqwest::Client::new();

		let response = client
			.post(URL)
			.json(&json!({
				"id": 1,
				"jsonrpc": "2.0",
				"method": "chain_getBlockchainData",
			}))
			.send()
			.await
			.unwrap()
			.json::<Value>()
			.await
			.unwrap();

		let _response: BlockchainStats =
			serde_json::from_value(response.get("result").unwrap().clone()).unwrap();

		Ok(())
	}

	pub async fn get_genesis_data() -> Result<(), ()> {
		let client = reqwest::Client::new();

		let response = client
			.post(URL)
			.json(&json!({
				"id": 1,
				"jsonrpc": "2.0",
				"method": "chain_getGenesisData",
			}))
			.send()
			.await
			.unwrap()
			.json::<Value>()
			.await
			.unwrap();

		let response: GenesisData<AccountId> =
			serde_json::from_value(response.get("result").unwrap().clone()).unwrap();
		let alice =
			AccountId::from_string("5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY").unwrap();
		assert!(response.verifiers.iter().any(|value| value.account_id == alice));

		Ok(())
	}
}

// Using one test for many tests causes because of issue with `create_runner`
// also this safe time (spend time only to run one node)
#[tokio::test(flavor = "multi_thread")]
async fn rpc_tests() -> Result<(), ()> {
	create_runner(async move {
		// Wait while node runs up
		tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;

		get_block_info().await.expect("Get block fail");
		get_blockchain_data().await.expect("Get blockchain data fails");
		get_genesis_data().await.expect("Get genesis data fails");

		Ok(())
	})
	.await
}
