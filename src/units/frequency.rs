use crate::units::{Duration, Seconds};
use derive_more::{Add, AddAssign, Sub, SubAssign};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Represent a frequency in Hz.
#[derive(Copy, Clone, Debug, PartialEq, Add, Sub, AddAssign, SubAssign, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Frequency(f64);

impl Frequency {
    /// Gets the raw value of the frequency as `f64`.
    pub fn as_f64(&self) -> f64 {
        self.0
    }

    /// Gets the raw value of the frequency as `f64`.
    pub fn as_f32(&self) -> f64 {
        self.as_f64()
    }

    /// Gets the period of the frequency (cycle duration):
    /// ```
    /// use rabu::units::{Duration, Frequency};
    ///
    /// let frequency = Frequency::from(20.0);
    /// let period = frequency.period();
    ///
    /// assert_eq!(period, Duration::from_secs_f64(0.05));
    /// ```
    pub fn period(&self) -> Duration {
        Duration::from_secs_f64(1.0 / self.as_f64())
    }

    /// Gets the period of the frequency (cycle duration):
    /// ```
    /// use rabu::units::{Duration, Frequency, Seconds};
    ///
    /// let frequency = Frequency::from(20.0);
    /// let period = frequency.period_seconds();
    ///
    /// assert_eq!(period, Seconds::from(0.05));
    /// ```
    pub fn period_seconds(&self) -> Seconds {
        (1.0 / self.as_f64()).into()
    }
}

macro_rules! impl_float_conversions {
    ($float_type: ty) => {
        impl From<$float_type> for Frequency {
            fn from(value: $float_type) -> Self {
                Self(value as _)
            }
        }

        impl From<Frequency> for $float_type {
            fn from(value: Frequency) -> Self {
                value.0 as _
            }
        }
    };
}

impl_float_conversions!(f32);
impl_float_conversions!(f64);

macro_rules! impl_from_int_type {
    ($int_type: ty) => {
        impl From<$int_type> for Frequency {
            fn from(value: $int_type) -> Self {
                Self(value as _)
            }
        }
    };
}

impl_from_int_type!(u64);
impl_from_int_type!(u32);
impl_from_int_type!(u16);
impl_from_int_type!(u8);
impl_from_int_type!(usize);

impl_from_int_type!(i64);
impl_from_int_type!(i32);
impl_from_int_type!(i16);
impl_from_int_type!(i8);
impl_from_int_type!(isize);
