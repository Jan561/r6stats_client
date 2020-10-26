/// The platforms Rainbow 6 Siege can be played on.
#[derive(Copy, Clone, Debug)]
pub enum Platform {
    Pc,
    Xbox,
    Playstation,
}

impl Platform {
    /// Returns the string representation for the api.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Pc => "pc",
            Self::Xbox => "xbox",
            Self::Playstation => "ps4",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Platform;

    #[test]
    fn test_platform_for_api() {
        assert_eq!(Platform::Pc.as_str(), "pc");
        assert_eq!(Platform::Xbox.as_str(), "xbox");
        assert_eq!(Platform::Playstation.as_str(), "ps4");
    }
}
