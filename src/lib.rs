//! ## Examples
//! Strongly typed units can easily be converted into each other:
//! ```rust
//! use rabu::units::{SampleRate, Samples, Seconds};
//!
//! let seconds = Seconds::from(3.0);
//! let sample_rate = SampleRate::from(44100);
//!
//! let samples = seconds.to_samples(sample_rate);
//!
//! assert_eq!(samples, Samples::from(132_300));
//! ```
//!
//! Audio buffers can be used in a way that makes sense:
//! ```rust
//! use rabu::buffer::Buffer;
//! use rabu::units::{Channels, Samples};
//!
//! let mut buffer = Buffer::allocate(Channels::from(2), Samples::from(4));
//!
//! for channel in buffer.iter_chans_mut() {
//!     for sample in channel.iter_mut() {
//!         *sample = 1.0;
//!     }
//! }
//! ```

pub mod biquad;
pub mod buffer;
pub mod units;
