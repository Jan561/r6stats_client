//! Module for seasonal stats.

mod match_result;
mod rank;
mod season;

pub use self::match_result::MatchResult;
pub use self::rank::Rank;
pub use self::season::Season;

use crate::internals::utils::serde_parse_f64_option;
use crate::region::Region;
use chrono::{DateTime, NaiveDate, Utc};
use int_enum::IntEnum;
use serde::{Deserialize, Deserializer};
use std::collections::HashMap;

/// Deserialized seasonal stats.
#[derive(Deserialize, Clone, Debug)]
#[non_exhaustive]
pub struct SeasonalStats {
    pub username: String,
    pub platform: String,
    pub ubisoft_id: String,
    pub uplay_id: Option<String>,
    pub avatar_url_146: Option<String>,
    pub avatar_url_256: Option<String>,
    pub last_updated: DateTime<Utc>,
    pub seasons: HashMap<Season, SeasonInfo>,
}

/// Deserialized season info.
#[derive(Deserialize, Clone, Debug)]
#[non_exhaustive]
pub struct SeasonInfo {
    pub name: String,
    pub start_date: DateTime<Utc>,
    pub end_date: Option<NaiveDate>,
    pub regions: HashMap<Region, Vec<RegionInfo>>,
}

/// Deserialized region info.
#[derive(Deserialize, Clone, Debug)]
#[non_exhaustive]
pub struct RegionInfo {
    #[serde(rename = "season_id")]
    #[serde(deserialize_with = "deserialize_season")]
    pub season: Season,
    pub region: String,
    pub abandons: u16,
    pub losses: u16,
    pub max_mmr: f32,
    pub max_rank: Rank,
    pub mmr: f32,
    pub next_rank_mmr: f32,
    pub prev_rank_mmr: f32,
    pub rank: Rank,
    pub skill_mean: f32,
    pub skill_standard_deviation: f32,
    pub created_for_date: DateTime<Utc>,
    pub wins: u16,
    pub kills: Option<u16>,
    pub deaths: Option<u16>,
    pub last_match_mmr_change: Option<i16>,
    // The endpoint returns a string for this field
    #[serde(deserialize_with = "serde_parse_f64_option")]
    pub last_match_skill_mean_change: Option<f64>,
    // The endpoint returns a string for this field
    #[serde(deserialize_with = "serde_parse_f64_option")]
    pub last_match_skill_standard_deviation_change: Option<f64>,
    pub last_match_result: Option<MatchResult>,
    pub champions_rank_position: Option<u16>,
    pub rank_text: String,
    pub rank_image: String,
    pub max_rank_text: String,
    pub max_rank_image: String,
}

fn deserialize_season<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Season, D::Error> {
    let id = u8::deserialize(deserializer)?;
    Season::from_int(id).map_err(serde::de::Error::custom)
}
