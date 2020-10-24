//! Models for leaderboard endpoint.

use serde::Deserialize;

/// The leaderboard
#[derive(Deserialize, Clone, Debug)]
#[serde(transparent)]
#[non_exhaustive]
pub struct Leaderboard {
    /// Players in the leaderboard, sorted ascending
    pub players: Vec<Player>,
}

/// Deserialized player.
#[derive(Deserialize, Clone, Debug)]
#[non_exhaustive]
pub struct Player {
    pub username: String,
    pub platform: String,
    pub ubisoft_id: String,
    pub uplay_id: Option<String>,
    pub avatar_url_146: Option<String>,
    pub avatar_url_256: Option<String>,
    pub stats: Stats,
    pub score: f32,
    pub position: u16,
}

/// Deserialized stats.
#[derive(Deserialize, Clone, Debug)]
#[non_exhaustive]
pub struct Stats {
    pub level: u16,
    pub kd: f32,
    pub wl: f32,
}
