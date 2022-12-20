//! This module contains a lot of strong types for units and quantities that are audio related.
//! These types have their corresponding conversion functions that make sense for that particular
//! type. For example: a `Seconds` object can be converted to a `Samples` object
//! when given a `SampleRate` value.

pub use bit_depth::BitDepth;
pub use channels::Channels;
pub use duration::Duration;
pub use frequency::Frequency;
pub use latency::Latency;
pub use percentage::Percentage;
pub use sample_rate::SampleRate;
pub use samples::Samples;
pub use seconds::Seconds;
pub use time_point::TimePoint;
pub use time_section::TimeSection;

mod bit_depth;
mod channels;
mod duration;
mod frequency;
mod latency;
mod percentage;
mod sample_rate;
mod samples;
mod seconds;
mod time_point;
mod time_section;
