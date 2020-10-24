use serde::Deserialize;

/// The regions
#[derive(Deserialize, Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum Region {
    /// US, Latin America
    Ncsa,
    /// Europe
    Emea,
    /// Asia, Australia
    Apac,
}

impl Region {
    /// Returns the string representation of the region in the api.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Ncsa => "ncsa",
            Self::Emea => "emea",
            Self::Apac => "apac",
        }
    }
}
