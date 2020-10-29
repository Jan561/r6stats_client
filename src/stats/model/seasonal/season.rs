use int_enum::IntEnum;
use serde::Deserialize;
use std::fmt::{self, Display, Formatter};

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

impl Display for Season {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::Health => write!(f, "Health"),
            Self::BloodOrchid => write!(f, "Blood Orchid"),
            Self::WhiteNoise => write!(f, "White Noise"),

            Self::Chimera => write!(f, "Chimera"),
            Self::ParaBellum => write!(f, "Para Bellum"),
            Self::GrimSky => write!(f, "Grim Sky"),
            Self::WindBastion => write!(f, "Wind Bastion"),

            Self::BurntHorizon => write!(f, "Burnt Horizon"),
            Self::PhantomSight => write!(f, "Phantom Sight"),
            Self::EmberRise => write!(f, "Ember Rise"),
            Self::ShiftingTides => write!(f, "Shifting Tides"),

            Self::VoidEdge => write!(f, "Void Edge"),
            Self::SteelWave => write!(f, "Steel Wave"),
            Self::ShadowLegacy => write!(f, "Shadow Legacy"),

            Self::Unknown => write!(f, "Unknown"),
        }
    }
}
