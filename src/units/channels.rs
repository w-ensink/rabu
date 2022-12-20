use derive_more::{Add, AddAssign, Sub, SubAssign};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Represents a number of audio channels.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Add, Sub, AddAssign, SubAssign, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Channels(pub u32);

impl Channels {
    /// Gives back the raw value as a `u32`.
    pub fn value(&self) -> u32 {
        self.0
    }

    /// Gives back the raw value as a `usize`.
    pub fn as_usize(&self) -> usize {
        self.value() as usize
    }

    /// Gives back the raw value as a `f64`.
    pub fn as_f64(&self) -> f64 {
        self.value() as f64
    }

    /// Gives back the raw value as a `u64`.
    pub fn as_u64(&self) -> u64 {
        self.value() as u64
    }
}