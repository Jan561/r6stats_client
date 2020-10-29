//! Module for seasonal stats.

use crate::internals::utils::serde_parse_f64_option;
use crate::region::Region;
use chrono::{DateTime, NaiveDate, Utc};
use int_enum::IntEnum;
use serde::{Deserialize, Deserializer};
use serde_repr::Deserialize_repr;
use std::collections::HashMap;

/// The rank of the player.
#[derive(Deserialize_repr, Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum Rank {
    Unranked = 0,

    CopperV = 1,
    CopperIV = 2,
    CopperIII = 3,
    CopperII = 4,
    CopperI = 5,

    BronzeV = 6,
    BronzeIV = 7,
    BronzeIII = 8,
    BronzeII = 9,
    BronzeI = 10,

    SilverV = 11,
    SilverIV = 12,
    SilverIII = 13,
    SilverII = 14,
    SilverI = 15,

    GoldIII = 16,
    GoldII = 17,
    GoldI = 18,

    PlatinumIII = 19,
    PlatinumII = 20,
    PlatinumI = 21,

    Diamond = 22,
    Champions = 23,
}

impl Rank {
    /// Returns true if [`Rank`] is unranked.
    ///
    /// [`Rank`]: struct.Rank.html
    pub fn is_unranked(self) -> bool {
        Self::Unranked == self
    }

    /// Returns true if [`Rank`] is copper.
    ///
    /// [`Rank`]: struct.Rank.html
    pub fn is_copper(self) -> bool {
        match self {
            Self::CopperI | Self::CopperII | Self::CopperIII | Self::CopperIV | Self::CopperV => {
                true
            }
            _ => false,
        }
    }

    /// Returns true if [`Rank`] is bronze.
    ///
    /// [`Rank`]: struct.Rank.html
    pub fn is_bronze(self) -> bool {
        match self {
            Self::BronzeI | Self::BronzeII | Self::BronzeIII | Self::BronzeIV | Self::BronzeV => {
                true
            }
            _ => false,
        }
    }

    /// Returns true if [`Rank`] is silver.
    ///
    /// [`Rank`]: struct.Rank.html
    pub fn is_silver(self) -> bool {
        match self {
            Self::SilverI | Self::SilverII | Self::SilverIII | Self::SilverIV | Self::SilverV => {
                true
            }
            _ => false,
        }
    }

    /// Returns true if [`Rank`] is gold.
    ///
    /// [`Rank`]: struct.Rank.html
    pub fn is_gold(self) -> bool {
        match self {
            Self::GoldI | Self::GoldII | Self::GoldIII => true,
            _ => false,
        }
    }

    /// Returns true if [`Rank`] is platinum.
    ///
    /// [`Rank`]: struct.Rank.html
    pub fn is_platinum(self) -> bool {
        match self {
            Self::PlatinumI | Self::PlatinumII | Self::PlatinumIII => true,
            _ => false,
        }
    }

    /// Returns true if [`Rank`] is diamond.
    ///
    /// [`Rank`]: struct.Rank.html
    pub fn is_diamond(self) -> bool {
        Self::Diamond == self
    }

    /// Returns true if [`Rank`] is champion.
    ///
    /// [`Rank`]: struct.Rank.html
    pub fn is_champion(self) -> bool {
        Self::Champions == self
    }
}

/// All seasons available in the api.
#[derive(IntEnum, Deserialize, Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
#[repr(u8)]
pub enum Season {
    // Year 2
    Health = 6,
    BloodOrchid = 7,
    WhiteNoise = 8,

    // Year 3
    Chimera = 9,
    ParaBellum = 10,
    GrimSky = 11,
    WindBastion = 12,

    // Year 4
    BurntHorizon = 13,
    PhantomSight = 14,
    EmberRise = 15,
    ShiftingTides = 16,

    // Year 5
    VoidEdge = 17,
    SteelWave = 18,
    ShadowLegacy = 19,

    /// For new seasons not yet implemented in this client.
    #[serde(other)]
    Unknown = 20,
}

impl Season {
    pub const fn current_season() -> Self {
        Self::ShadowLegacy
    }
}

/// The match result.
#[derive(Deserialize_repr, Copy, Clone, Debug)]
#[repr(u8)]
pub enum MatchResult {
    NotAvailable = 0,
    Win = 1,
    Loss = 2,
    Abandoned = 3,
}

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
