use crate::types::{CharTraitId, CommunityId};
use frame_support::dispatch::DispatchResult;

pub trait Hooks<AccountId, Balance, Username, PhoneNumber> {
	/// New user registered via `new_user` transactions. Implement to have something happen.
	/// This hook called after all checks performed and all values wrote to the storage.
	///
	/// # Arguments
	///
	/// * `verifier` - phone verifier, account that have permission to approve registration
	/// * `who` - `AccountId` of registered user
	/// * `name` - name of registered user, represented as byte vector
	/// * `phone_number` - phone number of registered user, represented as byte vector
	///
	/// # Returns
	///
	/// `Err` cause to abort transaction and revert state
	fn on_new_user(
		_verifier: AccountId,
		_who: AccountId,
		_name: Username,
		_phone_number: PhoneNumber,
	) -> DispatchResult {
		Ok(())
	}

	/// User info updated via `update_user` transactions. Implement to have something happen.
	/// This hook called after all checks performed and all values wrote to the storage.
	///
	/// # Arguments
	///
	/// # Returns
	///
	/// `Err` cause to abort transaction and revert state
	fn on_update_user() -> DispatchResult {
		Ok(())
	}

	/// User appreciated via `update_user` transactions. Implement to have something happen.
	/// This hook called after all checks performed and all values wrote to the storage.
	///
	/// # Arguments
	///
	/// * `payer` - `AccountId` of account who appreciate
	/// * `payee` - `AccountId` of account whom appreciate
	/// * `amount` - amount of tokens to transfer with appreciation
	/// * `community_id` - determine in which community appreciation happen
	/// * `char_trait_id` - determine trait of appreciation
	///
	/// # Returns
	///
	/// `Err` cause to abort transaction and revert state
	fn on_appreciation(
		_payer: AccountId,
		_payee: AccountId,
		_amount: Balance,
		_community_id: CommunityId,
		_char_trait_id: CharTraitId,
	) -> DispatchResult {
		Ok(())
	}

	/// New admin set for community via `set_admin` transactions. Implement to have something
	/// happen. This hook called after all checks performed and all values wrote to the storage.
	///
	/// # Arguments
	///
	/// * `_who` - `AccountId` of admin account who set new one
	/// * `_new_admin` - `AccountId` of new admin account
	///
	/// # Returns
	///
	/// `Err` cause to abort transaction and revert state
	fn on_set_admin(_who: AccountId, _new_admin: AccountId) -> DispatchResult {
		Ok(())
	}
}

impl<AccountId, Balance, Username, PhoneNumber, H1, H2>
	Hooks<AccountId, Balance, Username, PhoneNumber> for (H1, H2)
where
	AccountId: Clone,
	Balance: Clone,
	Username: Clone,
	PhoneNumber: Clone,
	H1: Hooks<AccountId, Balance, Username, PhoneNumber>,
	H2: Hooks<AccountId, Balance, Username, PhoneNumber>,
{
	fn on_new_user(
		verifier: AccountId,
		who: AccountId,
		name: Username,
		phone_number: PhoneNumber,
	) -> DispatchResult {
		H1::on_new_user(verifier.clone(), who.clone(), name.clone(), phone_number.clone())?;
		H2::on_new_user(verifier, who, name, phone_number)
	}

	fn on_update_user() -> DispatchResult {
		H1::on_update_user()?;
		H2::on_update_user()
	}

	fn on_appreciation(
		payer: AccountId,
		payee: AccountId,
		amount: Balance,
		community_id: CommunityId,
		char_trait_id: CharTraitId,
	) -> DispatchResult {
		H1::on_appreciation(
			payer.clone(),
			payee.clone(),
			amount.clone(),
			community_id,
			char_trait_id,
		)?;
		H2::on_appreciation(payer, payee, amount, community_id, char_trait_id)
	}

	fn on_set_admin(who: AccountId, new_admin: AccountId) -> DispatchResult {
		H1::on_set_admin(who.clone(), new_admin.clone())?;
		H2::on_set_admin(who, new_admin)
	}
}
