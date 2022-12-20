#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Represents a percentage, e.g. the export progress.
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Percentage(f64);

macro_rules! impl_float_conversions {
    ($float_type: ty) => {
        impl From<$float_type> for Percentage {
            fn from(value: $float_type) -> Self {
                Self(value as _)
            }
        }

        impl From<Percentage> for $float_type {
            fn from(value: Percentage) -> Self {
                value.0 as _
            }
        }
    };
}

impl_float_conversions!(f32);
impl_float_conversions!(f64);
