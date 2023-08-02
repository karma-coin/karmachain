use karmachain_node_runtime::{AccountId, PhoneNumberHash, Username};
use pallet_appreciation::CommunityRole;
use scale_info::prelude::string::String;
use sp_common::types::{CharTraitId, CommunityId, Score};
use sp_core::hashing::blake2_512;

/// Contains well prepared genesis configuration for the chain based on backup file
pub struct BackupGenesisConfig {
	pub endowed_accounts: Vec<(AccountId, u128)>,
	pub identities: Vec<(AccountId, Username, PhoneNumberHash)>,
	pub community_membership: Vec<(AccountId, CommunityId, CommunityRole)>,
	pub trait_scores: Vec<(AccountId, CommunityId, CharTraitId, Score)>,
}

impl BackupGenesisConfig {
	/// Creates a new instance of `BackupGenesisConfig` from the given backup file
	pub fn from_json(json: serde_json::Value) -> Result<Self, String> {
		let backup: backup_json::Backup =
			serde_json::from_value(json).map_err(|_| "Invalid JSON format.")?;

		// Filter users with zero balance and skip validator account
		let users: Vec<_> = backup
			.users
			.into_iter()
			.filter(|info| info.balance > 0)
			.filter(|info| info.mobile_number.is_some())
			.collect();

		// Read endowed accounts
		let endowed_accounts =
			users.iter().map(|info| (info.account_id.data.into(), info.balance)).collect();

		// Read identities
		let identities = users
			.iter()
			.cloned()
			.map(|info| {
				let account_id = info.account_id.data.into();
				let username = info.user_name.try_into().unwrap();
				let phone_number_hash =
					blake2_512(String::from(info.mobile_number.unwrap()).as_bytes()).into();

				(account_id, username, phone_number_hash)
			})
			.collect();

		// Read community membership
		let community_membership = users
			.iter()
			.flat_map(|info| {
				let account_id: AccountId = info.account_id.data.into();

				info.community_memberships.iter().map(move |community_membership| {
					let role = match community_membership.is_admin {
						true => CommunityRole::Admin,
						false => CommunityRole::Member,
					};

					(account_id.clone(), community_membership.community_id, role)
				})
			})
			.collect();

		// Read trait scores
		let trait_scores = users
			.iter()
			.flat_map(|info| {
				let account_id: AccountId = info.account_id.data.into();

				info.trait_scores.iter().map(move |trait_score| {
					(
						account_id.clone(),
						trait_score.community_id,
						trait_score.trait_id,
						trait_score.score,
					)
				})
			})
			.collect();

		Ok(Self { endowed_accounts, identities, community_membership, trait_scores })
	}
}

/// Data representation in the backup file
mod backup_json {
	use serde::Deserialize;

	#[derive(Debug, Clone, Deserialize)]
	pub struct Backup {
		pub users: Vec<User>,
	}

	#[derive(Debug, Clone, Deserialize)]
	pub struct User {
		pub account_id: BackupAccountId,
		pub user_name: String,
		pub mobile_number: Option<BackupPhoneNumber>,
		pub balance: u128,
		pub trait_scores: Vec<TraitScore>,
		pub community_memberships: Vec<CommunityMembership>,
	}

	#[derive(Debug, Clone, Deserialize)]
	pub struct BackupAccountId {
		pub data: [u8; 32],
	}

	#[derive(Debug, Clone, Deserialize)]
	pub struct BackupPhoneNumber {
		pub number: String,
	}

	impl From<BackupPhoneNumber> for String {
		fn from(backup_phone_number: BackupPhoneNumber) -> Self {
			backup_phone_number.number
		}
	}

	#[derive(Debug, Clone, Deserialize)]
	pub struct TraitScore {
		pub trait_id: u32,
		pub community_id: u32,
		pub score: u32,
	}

	#[derive(Debug, Clone, Deserialize)]
	pub struct CommunityMembership {
		pub community_id: u32,
		pub is_admin: bool,
	}
}
