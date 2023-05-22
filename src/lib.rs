//! # Ranger

#![no_std]

/// Contains ranged numeric types.
#[cfg(feature = "numeric")]
pub mod numeric;

#[cfg(feature = "numeric")]
pub use numeric::{
    RangedError, RangedI128, RangedI16, RangedI32, RangedI64, RangedI8, RangedU128, RangedU16,
    RangedU32, RangedU64, RangedU8,
};
