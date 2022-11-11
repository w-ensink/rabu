use std::ops::{Add, AddAssign, Sub};

use derive_more::{Add, AddAssign, Sub, SubAssign};
use partial_min_max::{max, min};
use serde::{Deserialize, Serialize};

/// Represents a percentage, e.g. the export progress.
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct Percentage(pub f64);

/// Represents a bit depth for an audio file.
#[derive(Serialize, Deserialize)]
pub enum BitDepth {
    Bits8,
    Bits16,
    Bits24,
    Bits32,
}

impl BitDepth {
    /// converts the number of bits to an integer.
    pub fn to_u16(&self) -> u16 {
        match self {
            BitDepth::Bits8 => 8,
            BitDepth::Bits16 => 16,
            BitDepth::Bits24 => 24,
            BitDepth::Bits32 => 32,
        }
    }
}

/// Represent seconds in audio domain.
#[derive(
    Serialize,
    Deserialize,
    Copy,
    Clone,
    Debug,
    PartialEq,
    Add,
    Sub,
    AddAssign,
    SubAssign,
    PartialOrd,
)]
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

/// Represents samples in the audio domain.
#[derive(
    Serialize,
    Deserialize,
    Copy,
    Clone,
    Debug,
    Eq,
    PartialEq,
    Add,
    Sub,
    AddAssign,
    SubAssign,
    Ord,
    PartialOrd,
)]
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

/// Represents a sample rate (in Hz.).
#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq, Eq)]
pub struct SampleRate(pub u32);

impl SampleRate {
    /// Gives back the raw value as a `u32`.
    pub fn value(&self) -> u32 {
        self.0
    }

    /// Gives back the raw value as a `usize`.
    pub fn as_usize(&self) -> usize {
        self.value() as usize
    }

    /// Gives back the raw value as a `f64`.
    pub fn as_f64(&self) -> f64 {
        self.value() as f64
    }

    /// Gives back the raw value as a `u64`.
    pub fn as_u64(&self) -> u64 {
        self.value() as u64
    }
}

/// Represents a number of audio channels.
#[derive(
    Serialize,
    Deserialize,
    Copy,
    Clone,
    Debug,
    PartialEq,
    Eq,
    Add,
    Sub,
    AddAssign,
    SubAssign,
    PartialOrd,
    Ord,
)]
pub struct Channels(pub u32);

impl Channels {
    /// Gives back the raw value as a `u32`.
    pub fn value(&self) -> u32 {
        self.0
    }

    /// Gives back the raw value as a `usize`.
    pub fn as_usize(&self) -> usize {
        self.value() as usize
    }

    /// Gives back the raw value as a `f64`.
    pub fn as_f64(&self) -> f64 {
        self.value() as f64
    }

    /// Gives back the raw value as a `u64`.
    pub fn as_u64(&self) -> u64 {
        self.value() as u64
    }
}

/// Represents a latency in the audio domain.
#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
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

/// Represents a time point in the audio domain, e.g. the start position of a file.
#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct TimePoint(pub Seconds);

impl TimePoint {
    /// Gives back the time point in seconds
    pub fn as_seconds(&self) -> Seconds {
        self.0
    }

    /// Creates a new time point from the given seconds.
    pub fn from_seconds(seconds: f64) -> Self {
        Self(Seconds(seconds))
    }
}

impl Add<Duration> for TimePoint {
    type Output = Self;
    fn add(self, rhs: Duration) -> Self::Output {
        Self(self.as_seconds() + rhs.as_seconds())
    }
}

impl Add<Seconds> for TimePoint {
    type Output = Self;

    fn add(self, rhs: Seconds) -> Self::Output {
        Self(self.as_seconds() + rhs)
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
        Duration(self.as_seconds() - rhs.as_seconds())
    }
}

/// Represents a duration in the time domain, e.g. the length of a clip.
#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq, PartialOrd)]
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

/// Represents a time section, e.g. the span of a clip in an arrangement.
#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct TimeSection {
    pub start: TimePoint,
    pub duration: Duration,
}

impl TimeSection {
    /// Returns the overlap (if any) between this time section and another.
    pub fn get_overlap(&self, other: Self) -> Option<Self> {
        if self.end() <= other.start || other.end() <= self.start {
            return None;
        }

        let start = max(self.start, other.start);
        let end = min(self.end(), other.end());
        let duration = end - start;
        Some(Self { start, duration })
    }

    /// Returns the end point of this time section.
    pub fn end(&self) -> TimePoint {
        self.start + self.duration
    }
}

#[macro_export]
macro_rules! time {
    (start:$from:expr, end:$to:expr) => {
        TimeSection {
            start: TimePoint(Seconds($from)),
            duration: Duration(Seconds($to - $from)),
        }
    };
    (duration:$d:expr) => {
        Duration(Seconds($d))
    };
    (start:$from:expr, duration:$d:expr) => {
        TimeSection {
            start: TimePoint(Seconds($from)),
            duration: Duration(Seconds($d)),
        }
    };
    (point:$p:expr) => {
        TimePoint(Seconds($p))
    };
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    use crate::units::*;

    #[test_case(Seconds(3.0), SampleRate(10) => Samples(30); "case 1")]
    #[test_case(Seconds(10.0), SampleRate(2) => Samples(20); "case 2")]
    fn seconds_to_samples(seconds: Seconds, sample_rate: SampleRate) -> Samples {
        seconds.to_samples(sample_rate)
    }

    #[test_case(Samples(1000), SampleRate(10) => Seconds(100.0); "case 1")]
    fn samples_to_seconds(samples: Samples, sample_rate: SampleRate) -> Seconds {
        samples.to_seconds(sample_rate)
    }

    /// creates `TimeSection` with `time!(<start>; <duration>)`
    macro_rules! time {
        ($start:expr; $duration:expr) => {
            TimeSection {
                start: TimePoint(Seconds($start)),
                duration: Duration(Seconds($duration)),
            }
        };
    }

    #[test_case(time!(0.0; 1.0), time!(2.0; 1.0) => None; "not overlapping")]
    #[test_case(time!(2.0; 1.0), time!(2.0; 1.0) => Some(time!(2.0; 1.0)); "same")]
    #[test_case(time!(1.0; 2.0), time!(2.0; 1.0) => Some(time!(2.0; 1.0)); "start overlap")]
    #[test_case(time!(1.0; 4.0), time!(2.0; 1.0) => Some(time!(2.0; 1.0)); "a over spans b")]
    fn time_sections_overlap(a: TimeSection, b: TimeSection) -> Option<TimeSection> {
        a.get_overlap(b)
    }
}
