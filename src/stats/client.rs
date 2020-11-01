use super::http::RouteBuilder;
use super::model::{GenericStats, OperatorStats, SeasonalStats, WeaponCategoryStats, WeaponStats};
use super::Kind;
use crate::internals::Rc;
use crate::{Error, Http, Platform};
use reqwest::Response;

/// Client for the stats endpoint.
#[derive(Clone, Debug)]
pub struct Client {
    http: Rc<Http>,
}

impl Client {
    pub(crate) fn new(http: Rc<Http>) -> Self {
        Self { http }
    }

    /// Returns the generic stats of a player.
    ///
    /// # Args
    ///
    /// - `username` - The username of the player
    /// - `platform` - The [`Platform`] of the player
    ///
    /// [`Platform`]: ../../platform/enum.Platform.html
    pub async fn generic(
        &self,
        username: impl AsRef<str>,
        platform: Platform,
    ) -> Result<GenericStats, Error> {
        let response = self
            .request(username.as_ref(), platform, Kind::Generic)
            .await?;
        let bytes = response.bytes().await?;
        Ok(serde_json::from_slice(&bytes)?)
    }

    /// Returns the generic stats of a player.
    ///
    /// # Args
    ///
    /// - `username` - The username of the player
    /// - `platform` - The [`Platform`] of the player
    ///
    /// [`Platform`]: ../../platform/enum.Platform.html
    pub async fn seasonal(
        &self,
        username: impl AsRef<str>,
        platform: Platform,
    ) -> Result<SeasonalStats, Error> {
        let response = self
            .request(username.as_ref(), platform, Kind::Seasonal)
            .await?;
        let bytes = response.bytes().await?;
        Ok(serde_json::from_slice(&bytes)?)
    }

    /// Returns the operator stats of a player.
    ///
    /// # Args
    ///
    /// - `username` - The username of the player
    /// - `platform` - The [`Platform`] of the player
    ///
    /// [`Platform`]: ../../platform/enum.Platform.html
    pub async fn operators(
        &self,
        username: impl AsRef<str>,
        platform: Platform,
    ) -> Result<OperatorStats, Error> {
        let response = self
            .request(username.as_ref(), platform, Kind::Operators)
            .await?;
        let bytes = response.bytes().await?;
        Ok(serde_json::from_slice(&bytes)?)
    }

    /// Returns the weapon-category stats of a player.
    ///
    /// # Args
    ///
    /// - `username` - The username of the player
    /// - `platform` - The [`Platform`] of the player
    ///
    /// [`Platform`]: ../../platform/enum.Platform.html
    pub async fn weapon_categories(
        &self,
        username: impl AsRef<str>,
        platform: Platform,
    ) -> Result<WeaponCategoryStats, Error> {
        let response = self
            .request(username.as_ref(), platform, Kind::WeaponCategories)
            .await?;
        let bytes = response.bytes().await?;
        Ok(serde_json::from_slice(&bytes)?)
    }

    /// Returns the weapon stats of a player.
    ///
    /// # Args
    ///
    /// - `username` - The username of the player
    /// - `platform` - The [`Platform`] of the player
    ///
    /// [`Platform`]: ../../platform/enum.Platform.html
    pub async fn weapons(
        &self,
        username: impl AsRef<str>,
        platform: Platform,
    ) -> Result<WeaponStats, Error> {
        let response = self
            .request(username.as_ref(), platform, Kind::Weapons)
            .await?;
        let bytes = response.bytes().await?;
        Ok(serde_json::from_slice(&bytes)?)
    }

    async fn request(
        &self,
        username: &str,
        platform: Platform,
        kind: Kind,
    ) -> Result<Response, Error> {
        let route = RouteBuilder::new()
            .username(username)
            .platform(platform)
            .kind(kind)
            .build()?;
        let path = route.path();
        self.http.request(&path).await
    }
}
