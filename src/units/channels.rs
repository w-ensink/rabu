use derive_more::{Add, AddAssign, Sub, SubAssign};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Represents a number of audio channels.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Add, Sub, AddAssign, SubAssign, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Channels(u32);

impl Channels {
    /// Gives back the raw value as a `u32`.
    pub fn as_u32(&self) -> u32 {
        self.0
    }

    /// Gives back the raw value as a `usize`.
    pub fn as_usize(&self) -> usize {
        self.as_u32() as usize
    }

    /// Gives back the raw value as a `u64`.
    pub fn as_u64(&self) -> u64 {
        self.as_u32() as u64
    }
}

macro_rules! impl_int_conversions {
    ($int_type:ty) => {
        impl From<$int_type> for Channels {
            fn from(value: $int_type) -> Self {
                Self(value as _)
            }
        }

        impl From<Channels> for $int_type {
            fn from(value: Channels) -> Self {
                value.0 as _
            }
        }
    };
}

impl_int_conversions!(u64);
impl_int_conversions!(u32);
impl_int_conversions!(u16);
impl_int_conversions!(u8);
impl_int_conversions!(usize);

impl_int_conversions!(i64);
impl_int_conversions!(i32);
impl_int_conversions!(i16);
impl_int_conversions!(i8);
impl_int_conversions!(isize);
