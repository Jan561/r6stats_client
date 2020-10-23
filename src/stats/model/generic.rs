use chrono::{DateTime, Utc};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Clone, Debug)]
pub struct GenericStats {
    pub username: String,
    pub platform: String,
    pub ubisoft_id: String,
    pub uplay_id: Option<String>,
    pub avatar_url_146: Option<String>,
    pub avatar_url_256: Option<String>,
    pub last_updated: DateTime<Utc>,
    pub aliases: Vec<Alias>,
    pub progression: Progression,
    pub stats: Stats,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Alias {
    pub username: String,
    pub last_seen_at: DateTime<Utc>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Progression {
    pub level: u16,
    pub lootbox_probability: f32,
    pub total_xp: u32,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Stats {
    pub general: GeneralStats,
    pub queue: HashMap<QueueMode, QueueInfo>,
    pub gamemode: Gamemode,
    pub timestamps: Timestamps,
}

#[derive(Deserialize, Clone, Debug)]
pub struct GeneralStats {
    pub assists: u32,
    pub barricades_deployed: u16,
    pub blind_kills: u16,
    pub bullets_fired: usize,
    pub bullets_hit: usize,
    pub dbnos: u32,
    pub deaths: u32,
    pub distance_travelled: isize,
    pub draws: u16,
    pub gadgets_destroyed: u32,
    pub games_played: u16,
    pub headshots: u32,
    pub kd: f32,
    pub kills: u32,
    pub losses: u32,
    pub melee_kills: u16,
    pub penetration_kills: u16,
    pub playtime: usize,
    pub rappel_breaches: u16,
    pub reinforcements_deployed: u16,
    pub revives: u16,
    pub suicides: u16,
    pub wins: u16,
    pub wl: f32,
}

#[derive(Deserialize, Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum QueueMode {
    Casual,
    Ranked,
    Other,
}

#[derive(Deserialize, Clone, Debug)]
pub struct QueueInfo {
    pub deaths: u32,
    pub draws: u16,
    pub games_played: u16,
    pub kd: f32,
    pub kills: u32,
    pub losses: u16,
    pub playtime: usize,
    pub wins: u16,
    pub wl: f32,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Gamemode {
    pub bomb: BombInfo,
    pub secure_area: SecureAreaInfo,
    pub hostage: HostageInfo,
}

#[derive(Deserialize, Clone, Debug)]
pub struct BombInfo {
    pub best_score: u16,
    pub games_played: u16,
    pub losses: u16,
    pub playtime: usize,
    pub wins: u16,
    pub wl: f32,
}

#[derive(Deserialize, Clone, Debug)]
pub struct SecureAreaInfo {
    pub best_score: u16,
    pub games_played: u16,
    pub kills_as_attacker_in_objective: u32,
    pub kills_as_defender_in_objective: u32,
    pub losses: u16,
    pub playtime: usize,
    pub times_objective_secured: u16,
    pub wins: u16,
    pub wl: f32,
}

#[derive(Deserialize, Clone, Debug)]
pub struct HostageInfo {
    pub best_score: u16,
    pub games_played: u16,
    pub losses: u16,
    pub playtime: usize,
    pub extractions_denied: u16,
    pub wins: u16,
    pub wl: f32,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Timestamps {
    pub created: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
}
