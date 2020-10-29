use serde::Deserialize;
use std::fmt::{self, Display, Formatter};

/// The queue mode.
#[derive(Deserialize, Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum QueueMode {
    /// Casual Queue
    Casual,
    /// Ranked Queue
    Ranked,
    /// Unranked Queue and other
    Other,
}

impl Display for QueueMode {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::Casual => write!(f, "Casual"),
            Self::Ranked => write!(f, "Ranked"),
            Self::Other => write!(f, "Other"),
        }
    }
}
