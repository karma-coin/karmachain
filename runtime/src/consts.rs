use crate::*;

/// This determines the average expected block time that we are targeting.
/// Blocks will be produced at a minimum duration defined by `SLOT_DURATION`.
/// `SLOT_DURATION` is picked up by `pallet_timestamp` which is in turn picked
/// up by `pallet_babe` to implement `fn slot_duration()`.
///
/// Change this to adjust the block time.
pub const MILLISECS_PER_BLOCK: u64 = 12_000;

// NOTE: Currently it is not possible to change the slot duration after the chain has started.
//       Attempting to do so will brick block production.
pub const SLOT_DURATION: u64 = MILLISECS_PER_BLOCK;
pub const EPOCH_DURATION_IN_SLOTS: BlockNumber = prod_or_fast!(HOURS, MINUTES);
pub const ERA_DURATION_IN_EPOCH: u32 = 6;

// Time is measured by number of blocks.
pub const MINUTES: BlockNumber = 60_000 / (MILLISECS_PER_BLOCK as BlockNumber);
pub const HOURS: BlockNumber = MINUTES * 60;
pub const DAYS: BlockNumber = HOURS * 24;
// Assumes month contains 30 days
pub const MONTHS: BlockNumber = DAYS * 30;

/// Existential deposit.
pub const EXISTENTIAL_DEPOSIT: u128 = 100 * KCENTS;

pub const KCENTS: Balance = 1;
pub const KCOINS: Balance = 1_000_000 * KCENTS;
