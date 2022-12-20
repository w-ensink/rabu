#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::units::Seconds;

/// Represents a latency in the audio domain.
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Latency(pub Seconds);

impl Latency {
    /// Gives back the raw value as a `f64`.
    pub fn value(&self) -> f64 {
        self.0.value()
    }

    /// Creates a new latency from the given number of seconds.
    pub fn from_seconds(seconds: f64) -> Self {
        Self(Seconds(seconds))
    }
}
