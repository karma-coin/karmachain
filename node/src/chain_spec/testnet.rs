use super::{backup::BackupGenesisConfig, utils::*};
use hex_literal::hex;
use karmachain_node_runtime::{
	opaque::SessionKeys, AccountId, AppreciationConfig, BabeConfig, BalancesConfig, GrandpaConfig,
	IdentityConfig, NominationPoolsConfig, PhoneNumberHash, RewardConfig, RuntimeGenesisConfig,
	SessionConfig, StakingConfig, SudoConfig, SystemConfig, Treasury, Username, KCENTS, KCOINS,
	MONTHS, WASM_BINARY,
};
use pallet_appreciation::*;
use pallet_staking::{Forcing, StakerStatus};
use sc_service::ChainType;
use scale_info::prelude::string::String;
use sp_common::types::{CharTraitId, CommunityId, Score};
use sp_consensus_babe::AuthorityId as BabeId;
use sp_core::crypto::UncheckedInto;
use sp_finality_grandpa::AuthorityId as GrandpaId;
use sp_runtime::Perbill;
use std::fs::File;

pub fn testnet_config<'a>(backup: Option<&'a str>) -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;

	let mut endowed_accounts = vec![
		(
			// 5GpsQN8PxCcRPAzuEVTASqzRFX3fDQUb1dHvRkAUt8Dxg7su
			hex!["ac9add5297f10ff04001f1f13fc51be3639ab3aacd03e57c000421c3a500a034"].into(),
			1_000 * 500_000 * KCOINS,
		),
		// Tokens for offchain account to allow sign karma reward transactions
		(
			// 5EWQqmfGFzFVr39cGQKFN1gWkFwtLcLwNx4QubwuwM7DHUTv
			hex!["6c139175aee0d20425b97644d7ce20148f8db69655999882531a1fe307f3b811"].into(),
			100 * KCOINS,
		),
	];
	let mut identities = vec![];
	let mut community_membership = vec![];
	let mut trait_scores = vec![];

	// Initial PoA authorities
	let initial_authorities = vec![(
		// 5GpsQN8PxCcRPAzuEVTASqzRFX3fDQUb1dHvRkAUt8Dxg7su
		hex!["ac9add5297f10ff04001f1f13fc51be3639ab3aacd03e57c000421c3a500a034"].into(),
		// 5Fy26cwbon8k8Pfx6YRJdeA6W6P8rbZH7mo57uRiLUyZmMSo
		hex!["ac9add5297f10ff04001f1f13fc51be3639ab3aacd03e57c000421c3a500a034"].into(),
		// 5Fy26cwbon8k8Pfx6YRJdeA6W6P8rbZH7mo57uRiLUyZmMSo
		hex!["ac9add5297f10ff04001f1f13fc51be3639ab3aacd03e57c000421c3a500a034"].unchecked_into(),
		// 5Ek9Ng9rECya5EBYdL8jgpdyUjvALUbhYaCSUC1nmDDS6VDN
		hex!["768cefd0d4abbf1d056d0095f4c3353a6bb9485f833b785eed1f10e9c5251b68"].unchecked_into(),
	)];
	// 5HarYoXkJhxCWWio78TQLXrMf3GoZf8RgHE26fHRNCmt5mPw
	let sudo: AccountId =
		hex!["f42bca078eb5ca56b30b4619c78009965ceda81af098dcaa4c255d14ae84b33c"].into();
	let phone_verifiers = vec![
		// 5EUH4CC5czdqfXbgE1fLkXcqMos1thxJSaj93J6N5bSareuz
		hex!["6a72de3655f40058d341020a2d5339ae3ac4101da6d75dcd98f6c2f787634da8"].into(),
	];
	let offchain_accounts = vec![
		// 5EWQqmfGFzFVr39cGQKFN1gWkFwtLcLwNx4QubwuwM7DHUTv
		hex!["6c139175aee0d20425b97644d7ce20148f8db69655999882531a1fe307f3b811"].into(),
	];

	// Read backup file if given
	if let Some(path) = backup {
		let file = File::open(path)
			.map_err(|e| format!("Failed to open backup file: {e} by path: {path}"))?;
		let json = serde_json::from_reader(file)
			.map_err(|e| format!("Failed to parse backup file: {e}"))?;
		let mut backup = BackupGenesisConfig::from_json(json)?;

		endowed_accounts.append(&mut backup.endowed_accounts);
		identities.append(&mut backup.identities);
		community_membership.append(&mut backup.community_membership);
		trait_scores.append(&mut backup.trait_scores);
		endowed_accounts.push((Treasury::account_id(), backup.treasury));
	}

	Ok(ChainSpec::from_genesis(
		// Name
		"Karmachain TN3",
		// ID
		"karmachain_tn_3",
		ChainType::Live,
		move || {
			testnet_genesis(
				wasm_binary,
				initial_authorities.clone(),
				// Sudo account
				sudo.clone(),
				// Phone versifiers accounts
				phone_verifiers.clone(),
				// Offchain accounts
				offchain_accounts.clone(),
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
fn testnet_genesis(
	wasm_binary: &[u8],
	initial_authorities: Vec<(AccountId, AccountId, BabeId, GrandpaId)>,
	root_key: AccountId,
	phone_verifiers: Vec<AccountId>,
	offchain_accounts: Vec<AccountId>,
	endowed_accounts: Vec<(AccountId, u128)>,
	identities: Vec<(AccountId, Username, PhoneNumberHash)>,
	community_membership: Vec<(AccountId, CommunityId, CommunityRole)>,
	trait_scores: Vec<(AccountId, CommunityId, CharTraitId, Score)>,
) -> RuntimeGenesisConfig {
	RuntimeGenesisConfig {
		system: SystemConfig {
			// Add Wasm runtime to storage.
			code: wasm_binary.to_vec(),
			..Default::default()
		},
		balances: BalancesConfig { balances: endowed_accounts },
		grandpa: GrandpaConfig { ..Default::default() },
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
			..Default::default()
		},
		staking: StakingConfig {
			minimum_validator_count: 1,
			validator_count: initial_authorities.len() as u32,
			stakers: initial_authorities
				.iter()
				.map(|x| (x.0.clone(), x.1.clone(), 500_000 * KCOINS, StakerStatus::Validator))
				.collect(),
			invulnerables: initial_authorities.iter().map(|x| x.0.clone()).collect(),
			force_era: Forcing::NotForcing,
			slash_reward_fraction: Perbill::from_percent(10),
			min_nominator_bond: 5 * KCOINS,
			min_validator_bond: 500_000 * KCOINS,
			max_validator_count: Some(100),
			max_nominator_count: Some(1_000),
			..Default::default()
		},
		nomination_pools: NominationPoolsConfig {
			min_join_bond: KCOINS,
			min_create_bond: 5 * KCOINS,
			max_pools: Some(512),
			max_members_per_pool: Some(5000),
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
			// The first 10 million users get 10 KCs on signup.
			signup_reward_phase1_alloc: 10_000_000 * 10 * KCOINS,
			// The next 200 millions users get 1 KC on signup.
			signup_reward_phase2_alloc: 200_000_000 * 1 * KCOINS,
			signup_reward_phase1_amount: 10 * KCOINS,
			signup_reward_phase2_amount: 1 * KCOINS,
			signup_reward_phase3_amount: 1_000 * KCENTS,
			// The first 10,000 referees get a 100 Kcs reward when their referral signs up.
			referral_reward_phase1_alloc: 10_000 * 100 * KCOINS,
			referral_reward_phase2_alloc: 9_900_000 * 10 * KCOINS,
			referral_reward_phase3_alloc: 200_000_000 * 1 * KCOINS,
			referral_reward_phase1_amount: 100 * KCOINS,
			referral_reward_phase2_amount: 10 * KCOINS,
			referral_reward_phase3_amount: 1 * KCOINS,
			// Each month period
			karma_reward_frequency: MONTHS,
			karma_reward_amount: 10 * KCOINS,
			karma_reward_alloc: 300_000_000 * KCOINS,
			karma_reward_users_participates: 500,
			karma_reward_appreciations_requires: 2,

			tx_fee_subsidy_max_per_user: 10,
			tx_fee_subsidies_alloc: 250_000_000 * KCOINS,
			tx_fee_subsidy_max_amount: 10 * KCENTS,
		},
		treasury: Default::default(),
	}
}
