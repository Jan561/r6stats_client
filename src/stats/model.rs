//! Models for stats endpoint.

pub mod generic;
pub mod operators;
pub mod seasonal;
pub mod weapon_categories;
pub mod weapons;

pub use self::generic::GenericStats;
pub use self::operators::OperatorStats;
pub use self::seasonal::SeasonalStats;
pub use self::weapon_categories::WeaponCategoryStats;
pub use self::weapons::WeaponStats;
