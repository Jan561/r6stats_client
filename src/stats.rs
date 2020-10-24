//! The stats endpoint of the api.

pub mod model;

mod client;
mod http;

pub use self::client::Client;

/// Type of stats to request.
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
