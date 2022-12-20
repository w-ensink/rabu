use crate::units::{Duration, Seconds};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Represents a sample rate (in Hz.).
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SampleRate(u32);

impl SampleRate {
    /// Gives back the raw value as a `u32`.
    pub fn as_u32(&self) -> u32 {
        self.0
    }

    /// Gives back the raw value as a `usize`.
    pub fn as_usize(&self) -> usize {
        self.as_u32() as usize
    }

    /// Gives back the raw value as a `f64`.
    pub fn as_f64(&self) -> f64 {
        self.as_u32() as f64
    }

    /// Gives back the raw value as a `u64`.
    pub fn as_u64(&self) -> u64 {
        self.as_u32() as u64
    }

    /// Gets the duration between two consecutive samples:
    /// Gets the time in seconds between tho consecutive samples:
    /// ```
    /// use rabu::units::{Duration, SampleRate};
    ///
    /// let sample_rate = SampleRate::from(20);
    /// let period = sample_rate.time_between_samples();
    ///
    /// assert_eq!(period, Duration::from_secs_f64(0.05));
    /// ```
    pub fn time_between_samples(&self) -> Duration {
        Duration::from_secs_f64(1.0 / self.as_f64())
    }

    /// Gets the time in seconds between tho consecutive samples:
    /// ```
    /// use rabu::units::{SampleRate, Seconds};
    ///
    /// let sample_rate = SampleRate::from(10);
    /// let period_secs = sample_rate.secs_between_samples();
    ///
    /// assert_eq!(period_secs, Seconds::from(0.1));
    /// ```
    pub fn secs_between_samples(&self) -> Seconds {
        Seconds::from(1.0 / self.as_f64())
    }
}

macro_rules! impl_int_conversions {
    ($int_type: ty) => {
        impl From<$int_type> for SampleRate {
            fn from(value: $int_type) -> Self {
                Self(value as _)
            }
        }

        impl From<SampleRate> for $int_type {
            fn from(value: SampleRate) -> Self {
                value.0 as _
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
