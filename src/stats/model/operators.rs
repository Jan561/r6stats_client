//! Module for operator stats.

use chrono::{DateTime, Utc};
use serde::Deserialize;

/// Deserialized operator stats.
#[derive(Deserialize, Clone, Debug)]
#[non_exhaustive]
pub struct OperatorStats {
    pub username: String,
    pub platform: String,
    pub ubisoft_id: String,
    pub uplay_id: Option<String>,
    pub avatar_url_146: Option<String>,
    pub avatar_url_256: Option<String>,
    pub last_updated: DateTime<Utc>,
    pub operators: Vec<OperatorInfo>,
}

/// Deserialized operator info.
#[derive(Deserialize, Clone, Debug)]
#[non_exhaustive]
pub struct OperatorInfo {
    pub name: String,
    pub ctu: String,
    pub role: String,
    pub kills: u32,
    pub deaths: u32,
    pub kd: f32,
    pub wins: u16,
    pub losses: u16,
    pub wl: f32,
    pub headshots: u32,
    pub dbnos: u32,
    pub melee_kills: u16,
    pub experience: u64,
    pub playtime: u64,
    pub abilities: Option<Vec<AbilityInfo>>,
    pub badge_image: Option<String>,
}

/// Deserialized ability info.
#[derive(Deserialize, Clone, Debug)]
#[non_exhaustive]
pub struct AbilityInfo {
    pub ability: String,
    pub value: Option<u16>,
}
