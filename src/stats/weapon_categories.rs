use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
pub struct WeaponCategoriesStats {
    pub username: String,
    pub platform: String,
    pub ubisoft_id: String,
    pub avatar_url_146: String,
    pub avatar_url_256: String,
    pub last_updated: DateTime<Utc>,
    pub categories: Vec<CategoriesInfo>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct CategoriesInfo {
    pub category: String,
    pub kills: u32,
    pub deaths: u32,
    pub kd: f32,
    pub headshots: u32,
    pub headshot_percentage: f32,
    pub times_chosen: u32,
    pub bullets_fired: usize,
    pub bullets_hit: usize,
    pub created: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
}
