use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Copy, Clone, Debug)]
pub enum Kind {
    Generic,
    Seasonal,
    Operators,
    WeaponCategories,
    Weapons,
}

impl From<Kind> for &str {
    fn from(val: Kind) -> &'static str {
        match val {
            Kind::Generic => "generic",
            Kind::Seasonal => "seasonal",
            Kind::Operators => "operators",
            Kind::WeaponCategories => "weapon-categories",
            Kind::Weapons => "weapons",
        }
    }
}

#[derive(Deserialize, Clone, Debug)]
pub struct SeasonalStats {
    pub username: String,
    pub platform: String,
    pub ubisoft_id: String,
    pub uplay_id: String,
    pub avatar_url_146: String,
    pub avatar_url_256: String,
    #[serde(with = "crate::date_format")]
    pub last_updated: DateTime<Utc>,
}
