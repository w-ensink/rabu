#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::units::{SampleRate, Samples, Seconds};

/// Represents a duration in the time domain, e.g. the length of a clip.
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Duration(Seconds);

impl Duration {
    /// Converts the duration to samples using the given sample rate:
    /// ```
    /// use rabu::units::{Duration, SampleRate, Samples};
    ///
    /// let duration = Duration::from_secs_f64(5.0);
    /// let sample_rate = SampleRate::from(44100);
    /// let num_samples = duration.to_samples(sample_rate);
    ///
    /// assert_eq!(num_samples, Samples::from(220500));
    /// ```
    pub fn to_samples(&self, sr: SampleRate) -> Samples {
        self.0.to_samples(sr)
    }

    /// Gives back the duration as seconds.
    pub fn as_seconds(&self) -> Seconds {
        self.0
    }

    pub fn as_secs_f64(&self) -> f64 {
        self.as_seconds().as_f64()
    }

    /// Creates a new duration from the given number of seconds.
    pub fn from_secs_f64(seconds: f64) -> Self {
        Self(Seconds::from(seconds))
    }
}

impl PartialEq<Seconds> for Duration {
    fn eq(&self, other: &Seconds) -> bool {
        self.as_seconds() == *other
    }
}

impl From<Seconds> for Duration {
    fn from(value: Seconds) -> Self {
        Self::from_secs_f64(value.as_f64())
    }
}

impl From<Duration> for Seconds {
    fn from(value: Duration) -> Self {
        value.as_seconds()
    }
}

impl From<std::time::Duration> for Duration {
    fn from(value: std::time::Duration) -> Self {
        Self::from_secs_f64(value.as_secs_f64())
    }
}

impl From<Duration> for std::time::Duration {
    fn from(value: Duration) -> Self {
        Self::from_secs_f64(value.as_secs_f64())
    }
}
