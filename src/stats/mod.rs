pub mod generic;
pub mod operators;
pub mod seasonal;
pub mod weapon_categories;
pub mod weapons;

pub mod client;
mod http;

pub use self::client::Client;

pub use self::generic::GenericStats;
pub use self::operators::OperatorsStats;
pub use self::seasonal::SeasonalStats;
pub use self::weapon_categories::WeaponCategoriesStats;
pub use self::weapons::WeaponsStats;

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
