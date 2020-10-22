use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
#[non_exhaustive]
pub struct OperatorsStats {
    pub username: String,
    pub platform: String,
    pub ubisoft_id: String,
    pub uplay_id: String,
    pub avatar_url_146: String,
    pub avatar_url_256: String,
    pub last_updated: DateTime<Utc>,
    pub operators: Vec<OperatorsInfo>,
}

#[derive(Deserialize, Clone, Debug)]
#[non_exhaustive]
pub struct OperatorsInfo {
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
    pub experience: usize,
    pub playtime: usize,
    pub abilities: Option<Vec<AbilitiesInfo>>,
    pub badge_image: Option<String>,
}

#[derive(Deserialize, Clone, Debug)]
#[non_exhaustive]
pub struct AbilitiesInfo {
    pub ability: String,
    pub value: Option<u16>,
}
