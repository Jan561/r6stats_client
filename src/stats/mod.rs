pub mod generic;
pub mod seasonal;

pub mod client;
pub(crate) mod http;

pub use client::Client;

pub use generic::GenericStats;
pub use seasonal::SeasonalStats;

#[derive(Copy, Clone, Debug)]
#[non_exhaustive]
pub enum Kind {
    Generic,
    Seasonal,
    Operators,
    WeaponCategories,
    Weapons,
}

impl Kind {
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
