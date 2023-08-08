use crate::*;
use frame_support::PalletId;
use frame_system::EnsureRootWithSuccess;

parameter_types! {
	pub const ProposalBond: Permill = Permill::from_percent(5);
	pub const ProposalBondMinimum: Balance = 1 * KCENTS;
	pub const ProposalBondMaximum: Balance = 1_000 * KCOINS;
	pub const SpendPeriod: BlockNumber = 6 * DAYS;
	pub const Burn: Permill = Permill::zero();
	pub const TreasuryPalletId: PalletId = PalletId(*b"py/trsry");
	pub const MaxApprovals: u32 = 100;
	pub const MaxBalance: Balance = Balance::max_value();
}

impl pallet_treasury::Config for Runtime {
	/// The staking balance.
	type Currency = Balances;
	/// Origin from which approvals must come.
	type ApproveOrigin = EnsureRoot<AccountId>;
	/// Origin from which rejections must come.
	type RejectOrigin = EnsureRoot<AccountId>;
	/// The overarching event type.
	type RuntimeEvent = RuntimeEvent;
	/// Handler for the unbalanced decrease when slashing for a rejected proposal or bounty.
	type OnSlash = Treasury;
	/// Fraction of a proposal's value that should be bonded in order to place the proposal.
	/// An accepted proposal gets these back. A rejected proposal does not.
	type ProposalBond = ProposalBond;
	/// Minimum amount of funds that should be placed in a deposit for making a proposal.
	type ProposalBondMinimum = ProposalBondMinimum;
	/// Maximum amount of funds that should be placed in a deposit for making a proposal.
	type ProposalBondMaximum = ProposalBondMaximum;
	/// Period between successive spends.
	type SpendPeriod = SpendPeriod;
	/// Percentage of spare funds (if any) that are burnt per spend period.
	type Burn = Burn;
	/// The treasury's pallet id, used for deriving its sovereign account ID.
	type PalletId = TreasuryPalletId;
	/// Handler for the unbalanced decrease when treasury funds are burned.
	type BurnDestination = ();
	/// Weight information for extrinsics in this pallet.
	type WeightInfo = pallet_treasury::weights::SubstrateWeight<Runtime>;
	/// Runtime hooks to external pallet using treasury to compute spend funds.
	type SpendFunds = ();
	/// The maximum number of approvals that can wait in the spending queue.
	///
	/// NOTE: This parameter is also used within the Bounties Pallet extension if enabled.
	type MaxApprovals = MaxApprovals;
	/// The origin required for approving spends from the treasury outside of the proposal
	/// process. The `Success` value is the maximum amount that this origin is allowed to
	/// spend at a time.
	type SpendOrigin = EnsureRootWithSuccess<AccountId, MaxBalance>;
}
