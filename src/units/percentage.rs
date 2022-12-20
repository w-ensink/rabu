#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Represents a percentage, e.g. the export progress.
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Percentage(pub f64);
