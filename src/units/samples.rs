use derive_more::{Add, AddAssign, Sub, SubAssign};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::units::{SampleRate, Seconds};

/// Represents samples in the audio domain.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Add, Sub, AddAssign, SubAssign, Ord, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Samples(u64);

impl Samples {
    /// Gives back the raw value as `u64`.
    pub fn as_u64(&self) -> u64 {
        self.0
    }

    /// Converts to seconds using the given sample rate.
    pub fn to_seconds(&self, sr: SampleRate) -> Seconds {
        Seconds::from(self.as_u64() as f64 / sr.value() as f64)
    }

    /// Gives back the raw value as a `usize`.
    pub fn as_usize(&self) -> usize {
        self.as_u64() as usize
    }

    /// Gives back the raw value as a `f64`.
    pub fn as_f64(&self) -> f64 {
        self.as_u64() as f64
    }
}

macro_rules! impl_int_conversions {
    ($int_type:ty) => {
        impl From<$int_type> for Samples {
            fn from(value: $int_type) -> Self {
                Self(value as _)
            }
        }

        impl From<Samples> for $int_type {
            fn from(value: Samples) -> Self {
                value.as_u64() as _
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

#[cfg(test)]
mod tests {
    use crate::units::{SampleRate, Samples, Seconds};
    use test_case::test_case;

    #[test_case(Samples::from(1000), SampleRate::from(10) => Seconds::from(100.0); "case 1")]
    fn samples_to_seconds(samples: Samples, sample_rate: SampleRate) -> Seconds {
        samples.to_seconds(sample_rate)
    }
}
