//! Storage byte-days dimension.
//!
//! Grounded in `MARKET.md:9` (`storage_byte_days`) and `MARKET.md:15-19`
//! (physical resources are booked in native units). Reserved storage is billed
//! by duration as well as size, so it is neither a token balance nor a wallet.

/// A booked storage allocation in byte-days.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Storage {
    /// Byte-days reserved (`MARKET.md:9`).
    pub byte_days: u64,
}

impl Storage {
    /// Native-unit key in the resource vector.
    pub const KEY: &'static str = "storage_byte_days";

    /// Construct a storage allocation.
    #[must_use]
    pub const fn of(byte_days: u64) -> Self {
        Self { byte_days }
    }
}
