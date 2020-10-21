use crate::regions::Region;
use chrono::{DateTime, NaiveDate, Utc};
use serde::Deserialize;
use serde_repr::Deserialize_repr;
use std::collections::HashMap;

#[derive(Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub enum Season {
    #[serde(rename = "health")]
    Health,
    #[serde(rename = "blood_orchid")]
    BloodOrchid,
    #[serde(rename = "white_noise")]
    WhiteNoise,
    #[serde(rename = "chimera")]
    Chimera,
    #[serde(rename = "para_bellum")]
    ParaBellum,
    #[serde(rename = "grim_sky")]
    GrimSky,
    #[serde(rename = "wind_bastion")]
    WindBastion,
    #[serde(rename = "burnt_horizon")]
    BurntHorizon,
    #[serde(rename = "phantom_sight")]
    PhantomSight,
    #[serde(rename = "ember_rise")]
    EmberRise,
    #[serde(rename = "shifting_tides")]
    ShiftingTides,
    #[serde(rename = "void_edge")]
    VoidEdge,
    #[serde(rename = "steel_wave")]
    SteelWave,
    #[serde(rename = "shadow_legacy")]
    ShadowLegacy,
    #[serde(other)]
    Unknown,
}

#[derive(Deserialize_repr, Copy, Clone, Debug)]
#[repr(u8)]
pub enum MatchResult {
    Vacant = 0,
    Win = 1,
    Loss = 2,
    Abandon = 3,
}

#[derive(Deserialize, Clone, Debug)]
#[non_exhaustive]
pub struct SeasonalStats {
    pub username: String,
    pub platform: String,
    pub ubisoft_id: String,
    pub uplay_id: String,
    pub avatar_url_146: String,
    pub avatar_url_256: String,
    pub last_updated: DateTime<Utc>,
    pub seasons: HashMap<Season, SeasonInfo>,
}

#[derive(Deserialize, Clone, Debug)]
#[non_exhaustive]
pub struct SeasonInfo {
    pub name: String,
    pub start_date: DateTime<Utc>,
    pub end_date: Option<NaiveDate>,
    pub regions: HashMap<Region, Vec<RegionInfo>>,
}

#[derive(Deserialize, Clone, Debug)]
#[non_exhaustive]
pub struct RegionInfo {
    pub season_id: u16,
    pub region: String,
    pub abandons: u16,
    pub losses: u16,
    pub max_mmr: f32,
    pub max_rank: u8,
    pub mmr: f32,
    pub next_rank_mmr: f32,
    pub prev_rank_mmr: f32,
    pub rank: u8,
    pub skill_mean: f32,
    pub skill_standard_deviation: f32,
    pub created_for_date: DateTime<Utc>,
    pub wins: u16,
    pub kills: Option<u16>,
    pub deaths: Option<u16>,
    pub last_match_mmr_change: Option<i16>,
    // FIXME: Float as String
    pub last_match_skill_mean_change: Option<String>,
    // FIXME: Float as String
    pub last_match_skill_standard_deviation_change: Option<String>,
    pub last_match_result: Option<MatchResult>,
    pub champions_rank_position: Option<u16>,
    pub rank_text: String,
    pub rank_image: String,
    pub max_rank_text: String,
    pub max_rank_image: String,
}
