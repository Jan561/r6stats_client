use serde::Deserialize;

/// The regions the datacenters are grouped in.
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

#[cfg(test)]
mod tests {
    use super::Region;

    #[test]
    fn test_region_for_api() {
        assert_eq!(Region::Ncsa.as_str(), "ncsa");
        assert_eq!(Region::Emea.as_str(), "emea");
        assert_eq!(Region::Apac.as_str(), "apac");
    }
}
