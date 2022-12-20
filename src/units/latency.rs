#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::units::Seconds;

/// Represents a latency in the audio domain.
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Latency(Seconds);

impl Latency {
    /// Gives back the seconds of latency as a `f64`.
    pub fn as_secs_f64(&self) -> f64 {
        self.0.as_f64()
    }

    /// Gives back the latency in seconds.
    pub fn as_seconds(&self) -> Seconds {
        self.0
    }

    /// Creates a new latency from the given number of seconds.
    pub fn from_secs_f64(seconds: f64) -> Self {
        Self(seconds.into())
    }
}

impl From<Seconds> for Latency {
    fn from(value: Seconds) -> Self {
        Self(value)
    }
}

impl From<Latency> for Seconds {
    fn from(value: Latency) -> Self {
        value.as_seconds()
    }
}
