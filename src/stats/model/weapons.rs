//! Module for weapon stats.

use chrono::{DateTime, Utc};
use serde::Deserialize;

/// Deserialized weapon stats.
#[derive(Deserialize, Clone, Debug)]
#[non_exhaustive]
pub struct WeaponStats {
    pub username: String,
    pub platform: String,
    pub ubisoft_id: String,
    pub uplay_id: Option<String>,
    pub avatar_url_146: Option<String>,
    pub avatar_url_256: Option<String>,
    pub last_updated: DateTime<Utc>,
    pub weapons: Vec<WeaponsInfo>,
}

/// Deserialized weapon info.
#[derive(Deserialize, Clone, Debug)]
#[non_exhaustive]
pub struct WeaponsInfo {
    pub weapon: String,
    pub category: String,
    pub kills: u32,
    pub deaths: u32,
    pub kd: f32,
    pub headshots: u32,
    pub headshot_percentage: f32,
    pub times_chosen: u16,
    pub bullets_fired: usize,
    pub bullets_hit: usize,
    pub created: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
}
