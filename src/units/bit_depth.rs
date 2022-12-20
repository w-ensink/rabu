#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Represents a bit depth for an audio file.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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
