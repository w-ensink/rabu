use std::ops::{Add, AddAssign, Sub};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::units::{Duration, Seconds};

/// Represents a time point in the audio domain, e.g. the start position of a file.
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TimePoint(Seconds);

impl TimePoint {
    /// Gives back the time point in seconds.
    pub fn as_seconds(&self) -> Seconds {
        self.0
    }

    /// Gives back the time point in seconds as a `f64`.
    pub fn as_secs_f64(&self) -> f64 {
        self.as_seconds().as_f64()
    }

    /// Creates a new time point from the given seconds.
    pub fn from_secs_f64(seconds: f64) -> Self {
        Self(Seconds::from(seconds))
    }
}

impl From<Seconds> for TimePoint {
    fn from(value: Seconds) -> Self {
        value.as_time_point()
    }
}

impl Sub<Duration> for TimePoint {
    type Output = Self;

    fn sub(self, rhs: Duration) -> Self::Output {
        Self::from(self.as_seconds() - rhs.as_seconds())
    }
}

impl Add<Duration> for TimePoint {
    type Output = Self;
    fn add(self, rhs: Duration) -> Self::Output {
        Self::from(self.as_seconds() + rhs.as_seconds())
    }
}

impl Sub<Seconds> for TimePoint {
    type Output = Self;

    fn sub(self, rhs: Seconds) -> Self::Output {
        Self::from(self.as_seconds() - rhs)
    }
}

impl Add<Seconds> for TimePoint {
    type Output = Self;

    fn add(self, rhs: Seconds) -> Self::Output {
        Self::from(self.as_seconds() + rhs)
    }
}

impl AddAssign<Seconds> for TimePoint {
    fn add_assign(&mut self, rhs: Seconds) {
        self.0 += rhs;
    }
}

impl Sub<Self> for TimePoint {
    type Output = Duration;
    fn sub(self, rhs: Self) -> Self::Output {
        Duration::from(self.as_seconds() - rhs.as_seconds())
    }
}
