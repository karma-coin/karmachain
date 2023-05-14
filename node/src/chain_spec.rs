use karmachain_node_runtime::{
	opaque::SessionKeys, AccountId, AppreciationConfig, BabeConfig, BalancesConfig, GenesisConfig,
	GrandpaConfig, IdentityConfig, PhoneNumber, SessionConfig, Signature, StakingConfig,
	SudoConfig, SystemConfig, KCOINS, WASM_BINARY,
};
use pallet_appreciation::*;
use pallet_staking::{Forcing, StakerStatus};
use sc_service::ChainType;
use scale_info::prelude::string::String;
use sp_consensus_babe::AuthorityId as BabeId;
use sp_core::{sr25519, Pair, Public};
use sp_finality_grandpa::AuthorityId as GrandpaId;
use sp_runtime::{
	traits::{IdentifyAccount, Verify},
	Perbill,
};

use sp_common::types::CommunityId;

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
					(vec![0].try_into().unwrap(), 1, CommunityRole::Admin),
					(vec![1].try_into().unwrap(), 1, CommunityRole::Member),
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
					(vec![0].try_into().unwrap(), 1, CommunityRole::Admin),
					(vec![1].try_into().unwrap(), 1, CommunityRole::Member),
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
#[allow(clippy::too_many_arguments)]
fn testnet_genesis(
	wasm_binary: &[u8],
	initial_authorities: Vec<(AccountId, AccountId, BabeId, GrandpaId)>,
	root_key: AccountId,
	endowed_accounts: Vec<AccountId>,
	phone_verifiers: Vec<AccountId>,
	community_membership: Vec<(PhoneNumber, CommunityId, CommunityRole)>,
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
			char_traits: vec![
				(1, "a Karma Grower".into(), "💚".into()),
				(2, "a Karma Appreciator".into(), "🙏".into()),
				(3, "Kind".into(), "🤗".into()),
				(4, "Helpful".into(), "🤗".into()),
				(5, "an Uber Geek".into(), "🤓".into()),
				(6, "Awesome".into(), "🤩".into()),
				(7, "Smart".into(), "🧠".into()),
				(8, "Sexy".into(), "🔥".into()),
				(9, "Patient".into(), "🐛".into()),
				(10, "Grateful".into(), "🦒".into()),
				(11, "Spiritual".into(), "🕊️".into()),
				(12, "Funny".into(), "🤣".into()),
				(13, "Caring".into(), "🤲".into()),
				(14, "Loving".into(), "💕".into()),
				(15, "Generous".into(), "🎁".into()),
				(16, "Honest".into(), "🤝".into()),
				(17, "Respectful".into(), "🎩".into()),
				(18, "Creative".into(), "🎨".into()),
				(19, "Intelligent".into(), "📚".into()),
				(20, "Loyal".into(), "🦒".into()),
				(21, "Trustworthy".into(), "👌".into()),
				(22, "Humble".into(), "🌱".into()),
				(23, "Courageous".into(), "🦁".into()),
				(24, "Confident".into(), "🌞".into()),
				(25, "Passionate".into(), "🌹".into()),
				(26, "Optimistic".into(), "😃".into()),
				(27, "Adventurous".into(), "🧗".into()),
				(28, "Determined".into(), "🏹".into()),
				(29, "Selfless".into(), "😇".into()),
				(30, "Self-aware".into(), "🤔".into()),
				(31, "Present".into(), "🦢".into()),
				(32, "Self-disciplined".into(), "💪".into()),
				(33, "Mindful".into(), "🧘".into()),
				(34, "My Guardian Angel".into(), "👼".into()),
				(35, "a Fairy".into(), "🧚".into()),
				(36, "a Wizard".into(), "🧙‍".into()),
				(37, "a Witch".into(), "🔮".into()),
				(38, "a Warrior".into(), "🥷".into()),
				(39, "a Healer".into(), "🌿".into()),
				(40, "a Guardian".into(), "🛡️".into()),
				(41, "a Karma Ambassador".into(), "💌".into()),
				(42, "an Inspiration".into(), "🌟".into()),
				(43, "a Sleeping Beauty".into(), "👸".into()),
				(44, "a Healer".into(), "❤️‍🩹".into()),
				(45, "a Master Mind".into(), "💡".into()),
				(46, "a Counselor".into(), "🫶".into()),
				(47, "an Architect".into(), "🏛️".into()),
				(48, "a Champion".into(), "🏆".into()),
				(49, "a Commander".into(), "👨‍✈️".into()),
				(50, "a Visionary".into(), "👁️".into()),
				(51, "a Teacher".into(), "👩‍🏫".into()),
				(52, "a Craftsperson".into(), "🛠️".into()),
				(53, "an Inspector".into(), "🔍".into()),
				(54, "a Composer".into(), "📝".into()),
				(55, "a Protector".into(), "⚔️".into()),
				(56, "a Provider".into(), "🤰".into()),
				(57, "a Performer".into(), "🎭".into()),
				(58, "a Supervisor".into(), "🕵️‍♀️".into()),
				(59, "a Dynamo".into(), "🚀".into()),
				(60, "an Imaginative Motivator".into(), "🌻".into()),
				(61, "a Campaigner".into(), "📣".into()),
				(62, "A Karma Rewards Winner".into(), ":trophy:".into()),
			],
			communities: vec![
				(
					1,
					"Grateful Giraffes".into(),
					"A global community of of leaders that come together for powerful wellness experiences".into(),
					"🦒".into(),
					"https://www.gratefulgiraffes.com".into(),
					"https://twitter.com/TheGratefulDAO".into(),
					"https://www.instagram.com/gratefulgiraffes".into(),
					"".into(),
					"https://discord.gg/7FMTXavy8N".into(),
					vec![10, 4, 3, 11, 15, 18, 39, 42, 60],
					true,
				),
			],
			community_membership,
			..Default::default()
		},
		identity: IdentityConfig { phone_verifiers },
		reward: Default::default(),
	}
}
