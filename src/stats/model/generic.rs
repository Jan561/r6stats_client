use chrono::{DateTime, Utc};
use serde::Deserialize;
use std::collections::HashMap;

/// Deserialized generic stats.
#[derive(Deserialize, Clone, Debug)]
#[non_exhaustive]
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

/// Deserialized aliases.
#[derive(Deserialize, Clone, Debug)]
#[non_exhaustive]
pub struct Alias {
    pub username: String,
    pub last_seen_at: DateTime<Utc>,
}

/// Deserialized progression.
#[derive(Deserialize, Clone, Debug)]
#[non_exhaustive]
pub struct Progression {
    pub level: u16,
    pub lootbox_probability: f32,
    pub total_xp: u32,
}

/// Deserialized stats.
#[derive(Deserialize, Clone, Debug)]
#[non_exhaustive]
pub struct Stats {
    pub general: GeneralStats,
    pub queue: HashMap<QueueMode, QueueInfo>,
    pub gamemode: Gamemode,
    pub timestamps: Timestamps,
}

/// Deserialized general stats.
#[derive(Deserialize, Clone, Debug)]
#[non_exhaustive]
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

/// The queue type.
#[derive(Deserialize, Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum QueueMode {
    /// Casual Queue
    Casual,
    /// Ranked Queue
    Ranked,
    /// Unranked Queue and other
    Other,
}

/// Deserialized info for the [`QueueMode`].
///
/// [`QueueMode`]: enum.QueueMode.html
#[derive(Deserialize, Clone, Debug)]
#[non_exhaustive]
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

/// Deserialized gamemodes.
#[derive(Deserialize, Clone, Debug)]
pub struct Gamemode {
    pub bomb: BombInfo,
    pub secure_area: SecureAreaInfo,
    pub hostage: HostageInfo,
}

/// Deserialized info for bomb gamemode.
#[derive(Deserialize, Clone, Debug)]
#[non_exhaustive]
pub struct BombInfo {
    pub best_score: u16,
    pub games_played: u16,
    pub losses: u16,
    pub playtime: usize,
    pub wins: u16,
    pub wl: f32,
}

/// Deserialized info for secure area gamemode.
#[derive(Deserialize, Clone, Debug)]
#[non_exhaustive]
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

/// Deserialized info for hostage gamemode.
#[derive(Deserialize, Clone, Debug)]
#[non_exhaustive]
pub struct HostageInfo {
    pub best_score: u16,
    pub games_played: u16,
    pub losses: u16,
    pub playtime: usize,
    pub extractions_denied: u16,
    pub wins: u16,
    pub wl: f32,
}

/// Deserialized timestamps.
#[derive(Deserialize, Clone, Debug)]
#[non_exhaustive]
pub struct Timestamps {
    pub created: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
}
