use derive_more::{Add, AddAssign, Sub, SubAssign};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::units::{SampleRate, Samples, TimePoint};

/// Represent seconds in audio domain.
#[derive(Copy, Clone, Debug, PartialEq, Add, Sub, AddAssign, SubAssign, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Seconds(pub f64);

impl Seconds {
    /// Convert to samples using the given sample rate.
    pub fn to_samples(&self, sr: SampleRate) -> Samples {
        Samples((self.value() * sr.value() as f64) as u64)
    }

    /// Gives back the raw value in f64.
    pub fn value(&self) -> f64 {
        self.0
    }

    /// Returns itself as a time point.
    pub fn as_time_point(&self) -> TimePoint {
        TimePoint(*self)
    }
}

#[cfg(test)]
mod tests {
    use crate::units::{SampleRate, Samples, Seconds};
    use test_case::test_case;

    #[test_case(Seconds(3.0), SampleRate(10) => Samples(30); "case 1")]
    #[test_case(Seconds(10.0), SampleRate(2) => Samples(20); "case 2")]
    fn seconds_to_samples(seconds: Seconds, sample_rate: SampleRate) -> Samples {
        seconds.to_samples(sample_rate)
    }
}
