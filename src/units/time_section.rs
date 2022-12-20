use partial_min_max::{max, min};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::units::{Duration, TimePoint};

/// Represents a time section, e.g. the span of a clip in an arrangement.
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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

#[cfg(test)]
mod test {
    use test_case::test_case;

    use crate::units::{Duration, TimePoint, TimeSection};

    /// creates `TimeSection` with `time!(<start>; <duration>)`
    macro_rules! time {
        ($start:expr; $duration:expr) => {
            TimeSection {
                start: TimePoint::from_secs_f64($start),
                duration: Duration::from_secs_f64($duration),
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
