use serde_repr::Deserialize_repr;
use std::fmt::{self, Display, Formatter};

/// The rank of the player.
#[derive(Deserialize_repr, Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum Rank {
    // Unranked
    Unranked = 0,

    //Copper
    CopperV = 1,
    CopperIV = 2,
    CopperIII = 3,
    CopperII = 4,
    CopperI = 5,

    // Bronze
    BronzeV = 6,
    BronzeIV = 7,
    BronzeIII = 8,
    BronzeII = 9,
    BronzeI = 10,

    // Silver
    SilverV = 11,
    SilverIV = 12,
    SilverIII = 13,
    SilverII = 14,
    SilverI = 15,

    // Gold
    GoldIII = 16,
    GoldII = 17,
    GoldI = 18,

    // Platinum
    PlatinumIII = 19,
    PlatinumII = 20,
    PlatinumI = 21,

    // Diamond+
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
        matches!(
            self,
            Self::CopperI | Self::CopperII | Self::CopperIII | Self::CopperIV | Self::CopperV
        )
    }

    /// Returns true if [`Rank`] is bronze.
    ///
    /// [`Rank`]: struct.Rank.html
    pub fn is_bronze(self) -> bool {
        matches!(
            self,
            Self::BronzeI | Self::BronzeII | Self::BronzeIII | Self::BronzeIV | Self::BronzeV
        )
    }

    /// Returns true if [`Rank`] is silver.
    ///
    /// [`Rank`]: struct.Rank.html
    pub fn is_silver(self) -> bool {
        matches!(
            self,
            Self::SilverI | Self::SilverII | Self::SilverIII | Self::SilverIV | Self::SilverV
        )
    }

    /// Returns true if [`Rank`] is gold.
    ///
    /// [`Rank`]: struct.Rank.html
    pub fn is_gold(self) -> bool {
        matches!(self, Self::GoldI | Self::GoldII | Self::GoldIII)
    }

    /// Returns true if [`Rank`] is platinum.
    ///
    /// [`Rank`]: struct.Rank.html
    pub fn is_platinum(self) -> bool {
        matches!(self, Self::PlatinumI | Self::PlatinumII | Self::PlatinumIII)
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

impl Display for Rank {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::Unranked => write!(f, "Unranked"),

            Self::CopperV => write!(f, "Copper V"),
            Self::CopperIV => write!(f, "Copper IV"),
            Self::CopperIII => write!(f, "Copper III"),
            Self::CopperII => write!(f, "Copper II"),
            Self::CopperI => write!(f, "Copper I"),

            Self::BronzeV => write!(f, "Bronze V"),
            Self::BronzeIV => write!(f, "Bronze IV"),
            Self::BronzeIII => write!(f, "Bronze III"),
            Self::BronzeII => write!(f, "Bronze II"),
            Self::BronzeI => write!(f, "Bronze I"),

            Self::SilverV => write!(f, "Silver V"),
            Self::SilverIV => write!(f, "Silver IV"),
            Self::SilverIII => write!(f, "Silver III"),
            Self::SilverII => write!(f, "Silver II"),
            Self::SilverI => write!(f, "Silver I"),

            Self::GoldIII => write!(f, "Gold III"),
            Self::GoldII => write!(f, "Gold II"),
            Self::GoldI => write!(f, "Gold I"),

            Self::PlatinumIII => write!(f, "Platinum III"),
            Self::PlatinumII => write!(f, "Platinum II"),
            Self::PlatinumI => write!(f, "Platinum I"),

            Self::Diamond => write!(f, "Platinum"),
            Self::Champions => write!(f, "Champions"),
        }
    }
}
