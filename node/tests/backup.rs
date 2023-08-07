mod utils;

use crate::utils::{create_runner, RpcResponse, RPC_URL};
use karmachain_node::cli::Cli;
use karmachain_node_runtime::AccountId;
use sc_cli::SubstrateCli;
use serde_json::json;
use sp_rpc::{CommunityMembership, TraitScore, UserInfo};

macro_rules! test_file {
	($fname:expr) => {
		concat!(env!("CARGO_MANIFEST_DIR"), "/tests/resources/", $fname) // assumes Linux ('/')!
	};
}

#[tokio::test(flavor = "multi_thread")]
async fn accounts_from_backup_exists_on_genesis() -> Result<(), ()> {
	let mut cli = Cli::from_args();
	// Setup chain in developer mode
	let chain_type = "dev";
	// Specify path to backup file
	let backup_file_path = test_file!("backup.json");
	// Set chain type and backup file path into config
	cli.run.shared_params.chain = Some(format!("{chain_type}:{backup_file_path}"));

	create_runner(cli, async move {
		let client = reqwest::Client::new();

		// Check ZeroUser is not on chain because of 0 balance
		let response = client
			.post(RPC_URL)
			.json(&json!({
				"id": 1,
				"jsonrpc": "2.0",
				"method": "identity_getUserInfoByUsername",
				"params": {
					"username": "ZeroUser"
				}
			}))
			.send()
			.await
			.expect("Fail to send request")
			.json::<RpcResponse<UserInfo<AccountId>>>()
			.await
			.expect("Fail to parse response");
		assert!(response.result.is_none());

		// Check TraitScoreUser has balance and trait score
		let response = client
			.post(RPC_URL)
			.json(&json!({
				"id": 1,
				"jsonrpc": "2.0",
				"method": "identity_getUserInfoByUsername",
				"params": {
					"username": "TraitScoreUser"
				}
			}))
			.send()
			.await
			.expect("Fail to send request")
			.json::<RpcResponse<UserInfo<AccountId>>>()
			.await
			.expect("Fail to parse response");
		assert!(response.result.is_some());
		let mut user_info = response.result.unwrap();
		assert_eq!(user_info.balance, 10007000);
		assert_eq!(
			user_info.trait_scores.sort(),
			vec![
				TraitScore { trait_id: 1, karma_score: 1, community_id: 0 },
				TraitScore { trait_id: 20, karma_score: 1, community_id: 0 },
				TraitScore { trait_id: 32, karma_score: 1, community_id: 0 },
				TraitScore { trait_id: 6, karma_score: 4, community_id: 0 },
				TraitScore { trait_id: 40, karma_score: 1, community_id: 0 },
				TraitScore { trait_id: 30, karma_score: 1, community_id: 0 },
				TraitScore { trait_id: 2, karma_score: 1, community_id: 0 },
			]
			.sort()
		);
		assert_eq!(user_info.karma_score, 10);

		// Check CommunityMembershipUser has balance, trait score and community membership
		let response = client
			.post(RPC_URL)
			.json(&json!({
				"id": 1,
				"jsonrpc": "2.0",
				"method": "identity_getUserInfoByUsername",
				"params": {
					"username": "CommunityMembershipUser"
				}
			}))
			.send()
			.await
			.expect("Fail to send request")
			.json::<RpcResponse<UserInfo<AccountId>>>()
			.await
			.expect("Fail to parse response");
		assert!(response.result.is_some());
		let mut user_info = response.result.unwrap();
		assert_eq!(user_info.balance, 10001001);
		assert_eq!(
			user_info.trait_scores.sort(),
			vec![
				TraitScore { trait_id: 1, karma_score: 1, community_id: 0 },
				TraitScore { trait_id: 2, karma_score: 2, community_id: 0 },
				TraitScore { trait_id: 15, karma_score: 1, community_id: 0 },
				TraitScore { trait_id: 35, karma_score: 1, community_id: 0 },
				TraitScore { trait_id: 10, karma_score: 1, community_id: 1 },
				TraitScore { trait_id: 6, karma_score: 1, community_id: 0 },
			]
			.sort()
		);
		assert_eq!(user_info.karma_score, 8);
		assert_eq!(
			user_info.community_membership.sort(),
			vec![CommunityMembership { community_id: 1, karma_score: 3, is_admin: false },].sort()
		);

		// Check BlockProducer has no balance and no identity
		let response = client
			.post(RPC_URL)
			.json(&json!({
				"id": 1,
				"jsonrpc": "2.0",
				"method": "identity_getUserInfoByUsername",
				"params": {
					"username": "Block producer 1"
				}
			}))
			.send()
			.await
			.expect("Fail to send request")
			.json::<RpcResponse<UserInfo<AccountId>>>()
			.await
			.expect("Fail to parse response");
		assert!(response.result.is_none());

		Ok(())
	})
	.await
}
