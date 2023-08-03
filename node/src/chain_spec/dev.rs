use super::{backup::BackupGenesisConfig, utils::*};
use karmachain_node_runtime::{
	opaque::SessionKeys, AccountId, AppreciationConfig, BabeConfig, BalancesConfig, GenesisConfig,
	GrandpaConfig, IdentityConfig, NominationPoolsConfig, PhoneNumberHash, RewardConfig,
	SessionConfig, StakingConfig, SudoConfig, SystemConfig, Username, KCOINS, WASM_BINARY,
};
use pallet_appreciation::*;
use pallet_staking::{Forcing, StakerStatus};
use sc_service::ChainType;
use scale_info::prelude::string::String;
use sp_common::types::{CharTraitId, CommunityId, Score};
use sp_consensus_babe::AuthorityId as BabeId;
use sp_core::sr25519;
use sp_finality_grandpa::AuthorityId as GrandpaId;
use sp_runtime::Perbill;
use std::fs::File;

const ENDOWMENT: u128 = 1_000_000_000 * KCOINS;
const STASH: u128 = 2_500_000 * KCOINS;

pub fn development_config<'a>(backup: Option<&'a str>) -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;

	let mut endowed_accounts = vec![
		(get_account_id_from_seed::<sr25519::Public>("Alice"), ENDOWMENT),
		(get_account_id_from_seed::<sr25519::Public>("Bob"), ENDOWMENT),
		(get_account_id_from_seed::<sr25519::Public>("Alice//stash"), ENDOWMENT),
		(get_account_id_from_seed::<sr25519::Public>("Bob//stash"), ENDOWMENT),
	];
	let mut identities = vec![];
	let mut community_membership = vec![];
	let mut trait_scores = vec![];

	// Read backup file if given
	if let Some(path) = backup {
		let file = File::open(path).map_err(|_| "Failed to open backup file")?;
		let json = serde_json::from_reader(file).map_err(|_| "Failed to parse backup file")?;
		let mut backup = BackupGenesisConfig::from_json(json)?;

		endowed_accounts.append(&mut backup.endowed_accounts);
		identities.append(&mut backup.identities);
		community_membership.append(&mut backup.community_membership);
		trait_scores.append(&mut backup.trait_scores);
	}

	Ok(ChainSpec::from_genesis(
		// Name
		"Development",
		// ID
		"dev",
		ChainType::Development,
		move || {
			development_genesis(
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
				// Phone versifiers accounts
				vec![get_account_id_from_seed::<sr25519::Public>("Alice")],
				// Offchain accounts
				vec![get_account_id_from_seed::<sr25519::Public>("Alice")],
				// Pre-funded accounts
				endowed_accounts.clone(),
				identities.clone(),
				community_membership.clone(),
				trait_scores.clone(),
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
			  "tokenDecimals": 6,
			  "tokenSymbol": "KCoin",
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
fn development_genesis(
	wasm_binary: &[u8],
	initial_authorities: Vec<(AccountId, AccountId, BabeId, GrandpaId)>,
	root_key: AccountId,
	phone_verifiers: Vec<AccountId>,
	offchain_accounts: Vec<AccountId>,
	endowed_accounts: Vec<(AccountId, u128)>,
	identities: Vec<(AccountId, Username, PhoneNumberHash)>,
	community_membership: Vec<(AccountId, CommunityId, CommunityRole)>,
	trait_scores: Vec<(AccountId, CommunityId, CharTraitId, Score)>,
) -> GenesisConfig {
	GenesisConfig {
		system: SystemConfig {
			// Add Wasm runtime to storage.
			code: wasm_binary.to_vec(),
		},
		balances: BalancesConfig { balances: endowed_accounts },
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
		nomination_pools: NominationPoolsConfig {
			min_join_bond: KCOINS,
			min_create_bond: KCOINS,
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
			trait_scores,
			..Default::default()
		},
		identity: IdentityConfig { phone_verifiers, identities: identities.clone() },
		reward: RewardConfig {
			accounts: identities.into_iter().map(|(account_id, _, _)| account_id).collect(),
			offchain_accounts,
			..Default::default()
		},
		treasury: Default::default(),
	}
}
