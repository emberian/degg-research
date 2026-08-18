//! Frozen dimensions of `dark-fba/n4-k4-q15/v0`.
//!
//! Every constant here is a public parameter of the relation. Changing any of
//! them creates a different relation version, not a configuration of this one.

/// Padded slot capacity of one batch.
pub const SLOTS: usize = 4;

/// Number of distinct owners the relation admits.
pub const OWNERS: u8 = 4;

/// Quote atoms per base atom, indexed by tick index.
pub const TICK_PRICES: [u32; 4] = [1, 2, 3, 4];

/// Number of price ticks.
pub const TICKS: usize = TICK_PRICES.len();

/// Smallest quantity an occupied slot may carry.
pub const QUANTITY_FLOOR: u32 = 1;

/// Largest quantity an occupied slot may carry.
pub const QUANTITY_CEILING: u32 = 15;

/// Relation identifier this oracle implements.
pub const RELATION: &str = "dark-fba/n4-k4-q15/v0";

/// Largest matched volume the frozen domain can produce: `4 * 15`.
pub const VOLUME_CEILING: u32 = QUANTITY_CEILING * SLOTS as u32;

/// Largest quote figure the frozen domain can produce: `60 * 4`.
pub const QUOTE_CEILING: u32 = VOLUME_CEILING * TICK_PRICES[TICKS - 1];
