#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::units::{SampleRate, Samples, Seconds};

/// Represents a duration in the time domain, e.g. the length of a clip.
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Duration(pub Seconds);

impl Duration {
    /// Converts the duration to samples using the given sample rate.
    pub fn to_samples(&self, sr: SampleRate) -> Samples {
        self.0.to_samples(sr)
    }

    /// Gives back the duration as seconds.
    pub fn as_seconds(&self) -> Seconds {
        self.0
    }

    /// Creates a new duration from the given number of seconds.
    pub fn from_seconds(seconds: f64) -> Self {
        Self(Seconds(seconds))
    }
}
