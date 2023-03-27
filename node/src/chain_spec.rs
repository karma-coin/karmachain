use karmachain_node_runtime::{
	opaque::SessionKeys, AccountId, AppreciationConfig, BabeConfig, BalancesConfig, GenesisConfig,
	GrandpaConfig, IdentityConfig, SessionConfig, Signature, StakingConfig, SudoConfig,
	SystemConfig, KCOINS, WASM_BINARY,
};
use pallet_appreciation::*;
use pallet_staking::{Forcing, StakerStatus};
use sc_service::ChainType;
use sp_consensus_babe::AuthorityId as BabeId;
use sp_core::{sr25519, Pair, Public};
use sp_finality_grandpa::AuthorityId as GrandpaId;
use sp_runtime::{
	traits::{IdentifyAccount, Verify},
	Perbill,
};

use sp_common::String;

// The URL for the telemetry server.
// const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig>;

/// Generate a crypto pair from seed.
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

type AccountPublic = <Signature as Verify>::Signer;

/// Generate an account ID from seed.
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
	AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

pub fn development_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;

	Ok(ChainSpec::from_genesis(
		// Name
		"Development",
		// ID
		"dev",
		ChainType::Development,
		move || {
			testnet_genesis(
				wasm_binary,
				// Initial PoA authorities
				vec![(
					get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					get_from_seed::<BabeId>("Alice"),
					get_from_seed::<GrandpaId>("Alice"),
				)],
				// Sudo account
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				// Pre-funded accounts
				vec![
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					get_account_id_from_seed::<sr25519::Public>("Bob"),
					get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
					get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
				],
				vec![get_account_id_from_seed::<sr25519::Public>("Alice")],
				vec![
					(1, "SignUp".into(), "".into()),
					(2, "Spender".into(), "".into()),
					(3, "Smart".into(), "".into()),
					(4, "Helpfull".into(), "".into()),
				],
				vec![
					(
						1,
						"Public".into(),
						"".into(),
						"".into(),
						"".into(),
						"".into(),
						"".into(),
						"".into(),
						"".into(),
						true,
					),
					(
						2,
						"Private".into(),
						"".into(),
						"".into(),
						"".into(),
						"".into(),
						"".into(),
						"".into(),
						"".into(),
						false,
					),
				],
				vec![
					(get_account_id_from_seed::<sr25519::Public>("Alice"), 1, CommunityRole::Admin),
					(get_account_id_from_seed::<sr25519::Public>("Bob"), 1, CommunityRole::Member),
					(get_account_id_from_seed::<sr25519::Public>("Alice"), 2, CommunityRole::Admin),
				],
				true,
			)
		},
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		None,
		None,
		// Properties
		Some(
			serde_json::json!({
			  "tokenDecimals": 0,
			  "tokenSymbol": "KCent",
			})
			.as_object()
			.expect("Map given")
			.clone(),
		),
		// Extensions
		None,
	))
}

pub fn local_testnet_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;

	Ok(ChainSpec::from_genesis(
		// Name
		"Local Testnet",
		// ID
		"local_testnet",
		ChainType::Local,
		move || {
			testnet_genesis(
				wasm_binary,
				// Initial PoA authorities
				vec![
					(
						get_account_id_from_seed::<sr25519::Public>("Alice"),
						get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
						get_from_seed::<BabeId>("Alice"),
						get_from_seed::<GrandpaId>("Alice"),
					),
					(
						get_account_id_from_seed::<sr25519::Public>("Bob"),
						get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
						get_from_seed::<BabeId>("Bob"),
						get_from_seed::<GrandpaId>("Bob"),
					),
				],
				// Sudo account
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				// Pre-funded accounts
				vec![
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					get_account_id_from_seed::<sr25519::Public>("Bob"),
					get_account_id_from_seed::<sr25519::Public>("Charlie"),
					get_account_id_from_seed::<sr25519::Public>("Dave"),
					get_account_id_from_seed::<sr25519::Public>("Eve"),
					get_account_id_from_seed::<sr25519::Public>("Ferdie"),
					get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
					get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
					get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
					get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
					get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
					get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
				],
				vec![get_account_id_from_seed::<sr25519::Public>("Alice")],
				vec![
					(1, "SignUp".into(), "".into()),
					(2, "Spender".into(), "".into()),
					(3, "Smart".into(), "".into()),
					(4, "Helpfull".into(), "".into()),
				],
				vec![
					(
						1,
						"Public".into(),
						"".into(),
						"".into(),
						"".into(),
						"".into(),
						"".into(),
						"".into(),
						"".into(),
						true,
					),
					(
						2,
						"Private".into(),
						"".into(),
						"".into(),
						"".into(),
						"".into(),
						"".into(),
						"".into(),
						"".into(),
						false,
					),
				],
				vec![
					(get_account_id_from_seed::<sr25519::Public>("Alice"), 1, CommunityRole::Admin),
					(get_account_id_from_seed::<sr25519::Public>("Bob"), 1, CommunityRole::Member),
					(get_account_id_from_seed::<sr25519::Public>("Alice"), 2, CommunityRole::Admin),
				],
				true,
			)
		},
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		None,
		None,
		// Properties
		Some(
			serde_json::json!({
			  "tokenDecimals": 0,
			  "tokenSymbol": "KCent",
			})
			.as_object()
			.expect("Map given")
			.clone(),
		),
		// Extensions
		None,
	))
}

/// Configure initial storage state for FRAME modules.
fn testnet_genesis(
	wasm_binary: &[u8],
	initial_authorities: Vec<(AccountId, AccountId, BabeId, GrandpaId)>,
	root_key: AccountId,
	endowed_accounts: Vec<AccountId>,
	phone_verifiers: Vec<AccountId>,
	char_traits: Vec<(CharTraitId, String, String)>,
	communities: Vec<(
		CommunityId,
		String,
		String,
		String,
		String,
		String,
		String,
		String,
		String,
		bool,
	)>,
	community_membership: Vec<(AccountId, CommunityId, CommunityRole)>,
	_enable_println: bool,
) -> GenesisConfig {
	const ENDOWMENT: u128 = 1_000_000_000 * KCOINS;
	const STASH: u128 = 2_500_000 * KCOINS;

	GenesisConfig {
		system: SystemConfig {
			// Add Wasm runtime to storage.
			code: wasm_binary.to_vec(),
		},
		balances: BalancesConfig {
			// Configure endowed accounts with initial balance of 1 << 60.
			balances: endowed_accounts.iter().cloned().map(|k| (k, ENDOWMENT)).collect(),
		},
		grandpa: GrandpaConfig { authorities: Default::default() },
		sudo: SudoConfig {
			// Assign network admin rights.
			key: Some(root_key),
		},
		session: SessionConfig {
			keys: initial_authorities
				.iter()
				.map(|x| {
					(
						x.0.clone(),
						x.0.clone(),
						SessionKeys { babe: x.2.clone(), grandpa: x.3.clone() },
					)
				})
				.collect::<Vec<_>>(),
		},
		transaction_payment: Default::default(),
		babe: BabeConfig {
			authorities: Default::default(),
			epoch_config: Some(karmachain_node_runtime::BABE_GENESIS_EPOCH_CONFIG),
		},
		staking: StakingConfig {
			minimum_validator_count: 1,
			validator_count: initial_authorities.len() as u32,
			stakers: initial_authorities
				.iter()
				.map(|x| (x.0.clone(), x.1.clone(), STASH, StakerStatus::Validator))
				.collect(),
			invulnerables: initial_authorities.iter().map(|x| x.0.clone()).collect(),
			force_era: Forcing::NotForcing,
			slash_reward_fraction: Perbill::from_percent(10),
			..Default::default()
		},
		appreciation: AppreciationConfig {
			char_traits,
			communities,
			community_membership,
			..Default::default()
		},
		identity: IdentityConfig { phone_verifiers },
	}
}
