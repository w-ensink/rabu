use derive_more::{Add, AddAssign, Sub, SubAssign};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::units::{SampleRate, Samples, TimePoint};

/// Represent seconds in audio domain.
#[derive(Copy, Clone, Debug, PartialEq, Add, Sub, AddAssign, SubAssign, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Seconds(f64);

impl Seconds {
    /// Convert to samples using the given sample rate.
    pub fn to_samples(&self, sr: SampleRate) -> Samples {
        Samples::from((self.as_f64() * sr.as_u32() as f64).round() as u64)
    }

    /// Gives back the raw value in f64.
    pub fn as_f64(&self) -> f64 {
        self.0
    }

    /// Returns itself as a time point.
    pub fn as_time_point(&self) -> TimePoint {
        TimePoint::from_secs_f64(self.as_f64())
    }
}

macro_rules! impl_float_conversions {
    ($float_type: ty) => {
        impl From<$float_type> for Seconds {
            fn from(value: $float_type) -> Self {
                Self(value as _)
            }
        }

        impl From<Seconds> for $float_type {
            fn from(value: Seconds) -> Self {
                value.as_f64() as _
            }
        }
    };
}

impl_float_conversions!(f32);
impl_float_conversions!(f64);

impl From<std::time::Duration> for Seconds {
    fn from(value: std::time::Duration) -> Self {
        Self(value.as_secs_f64())
    }
}

impl From<Seconds> for std::time::Duration {
    fn from(value: Seconds) -> Self {
        Self::from_secs_f64(value.into())
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use crate::units::{SampleRate, Samples, Seconds};

    #[test_case(Seconds::from(3.0), SampleRate::from(10) => Samples::from(30); "case 1")]
    #[test_case(Seconds::from(10.0), SampleRate::from(2) => Samples::from(20); "case 2")]
    #[test_case(Seconds::from(10.4), SampleRate::from(2.0) => Samples::from(21); "round important")]
    fn seconds_to_samples(seconds: Seconds, sample_rate: SampleRate) -> Samples {
        seconds.to_samples(sample_rate)
    }
}
