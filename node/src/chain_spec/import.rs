use super::utils::*;
use karmachain_node_runtime::{
	AccountId, AppreciationConfig, BabeConfig, BalancesConfig, GenesisConfig, GrandpaConfig,
	NominationPoolsConfig, StakingConfig, SystemConfig, KCOINS, WASM_BINARY,
};

use pallet_staking::Forcing;
use sc_service::ChainType;
use scale_info::prelude::string::String;
use serde::Deserialize;
use sp_runtime::Perbill;

pub fn import_config(path: String) -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;

	Ok(ChainSpec::from_genesis(
		// Name
		"Testnet",
		// ID
		"testnet",
		ChainType::Live,
		move || import_genesis(wasm_binary, path.clone()),
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
fn import_genesis(wasm_binary: &[u8], path: String) -> GenesisConfig {
	GenesisConfig {
		system: SystemConfig {
			// Add Wasm runtime to storage.
			code: wasm_binary.to_vec(),
		},
		balances: BalancesConfig { balances: vec![] },
		grandpa: GrandpaConfig { authorities: Default::default() },
		sudo: Default::default(),
		session: Default::default(),
		transaction_payment: Default::default(),
		babe: BabeConfig {
			authorities: Default::default(),
			epoch_config: Some(karmachain_node_runtime::BABE_GENESIS_EPOCH_CONFIG),
		},
		staking: StakingConfig {
			minimum_validator_count: 1,
			validator_count: 1,
			stakers: Default::default(),
			invulnerables: Default::default(),
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
			..Default::default()
		},
		identity: Default::default(),
		reward: Default::default(),
	}
}

#[derive(Deserialize)]
pub struct Backup {
	pub users: Vec<User>,
}

#[derive(Deserialize)]
pub struct User {
	pub account_id: BackupAccountId,
	pub user_name: String,
	pub mobile_number: BackupPhoneNumber,
	pub balance: u128,
	pub trait_scores: Vec<TraitScore>,
	pub community_memberships: Vec<CommunityMembership>,
}

#[derive(Deserialize)]
pub struct BackupAccountId {
	pub data: [u8; 32],
}

#[derive(Deserialize)]
pub struct BackupPhoneNumber {
	pub number: String,
}

impl From<BackupPhoneNumber> for String {
	fn from(backup_phone_number: BackupPhoneNumber) -> Self {
		backup_phone_number.number
	}
}

#[derive(Deserialize)]
pub struct TraitScore {
	pub trait_id: u32,
	pub community_id: u32,
	pub score: u64,
}

#[derive(Deserialize)]
pub struct CommunityMembership {
	pub community_id: u32,
	pub is_admin: bool,
}
