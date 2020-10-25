//! The regions in Rainbow 6 Siege.

use serde::Deserialize;

/// The regions
#[derive(Deserialize, Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum Region {
    /// US East, US West, US Central, US South Central, Brazil South
    Ncsa,
    /// EU West, EU North
    Emea,
    /// Asia East, Asia SouthEast, Australia East
    Apac,
}

impl Region {
    /// Returns the string representation for the api.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Ncsa => "ncsa",
            Self::Emea => "emea",
            Self::Apac => "apac",
        }
    }
}
