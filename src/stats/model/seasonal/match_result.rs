use serde_repr::Deserialize_repr;
use std::fmt::{self, Display, Formatter};

/// The match result.
#[derive(Deserialize_repr, Copy, Clone, Debug)]
#[repr(u8)]
pub enum MatchResult {
    NotAvailable = 0,
    Win = 1,
    Loss = 2,
    Abandoned = 3,
}

impl Display for MatchResult {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::NotAvailable => write!(f, "Not Available"),
            Self::Win => write!(f, "Win"),
            Self::Loss => write!(f, "Loss"),
            Self::Abandoned => write!(f, "Abandoned"),
        }
    }
}
