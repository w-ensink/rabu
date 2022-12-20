use derive_more::{Add, AddAssign, Sub, SubAssign};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::units::{SampleRate, Seconds};

/// Represents samples in the audio domain.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Add, Sub, AddAssign, SubAssign, Ord, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Samples(pub u64);

impl Samples {
    /// Gives back the raw value as `u64`.
    pub fn value(&self) -> u64 {
        self.0
    }

    /// Converts to seconds using the given sample rate.
    pub fn to_seconds(&self, sr: SampleRate) -> Seconds {
        Seconds(self.value() as f64 / sr.value() as f64)
    }

    /// Gives back the raw value as a `usize`.
    pub fn as_usize(&self) -> usize {
        self.value() as usize
    }

    /// Gives back the raw value as a `f64`.
    pub fn as_f64(&self) -> f64 {
        self.value() as f64
    }
}

#[cfg(test)]
mod tests {
    use crate::units::{SampleRate, Samples, Seconds};
    use test_case::test_case;

    #[test_case(Samples(1000), SampleRate(10) => Seconds(100.0); "case 1")]
    fn samples_to_seconds(samples: Samples, sample_rate: SampleRate) -> Seconds {
        samples.to_seconds(sample_rate)
    }
}
