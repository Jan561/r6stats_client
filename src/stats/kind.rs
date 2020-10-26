/// Type of stats to request.
#[derive(Copy, Clone, Debug)]
#[non_exhaustive]
pub(super) enum Kind {
    Generic,
    Seasonal,
    Operators,
    WeaponCategories,
    Weapons,
}

impl Kind {
    /// Returns the string representation for the api.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Generic => "generic",
            Self::Seasonal => "seasonal",
            Self::Operators => "operators",
            Self::WeaponCategories => "weapon-categories",
            Self::Weapons => "weapons",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Kind;

    #[test]
    fn test_kind_for_api() {
        assert_eq!(Kind::Generic.as_str(), "generic");
        assert_eq!(Kind::Seasonal.as_str(), "seasonal");
        assert_eq!(Kind::Operators.as_str(), "operators");
        assert_eq!(Kind::WeaponCategories.as_str(), "weapon-categories");
        assert_eq!(Kind::Weapons.as_str(), "weapons");
    }
}
