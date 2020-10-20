use super::model::Leaderboard;
use crate::internals::Rc;
use crate::leaderboard::http::RouteBuilder;
use crate::{Error, Http, Platform, Region};
use reqwest::Response;

/// Client for the leaderboard endpoint.
#[derive(Clone, Debug)]
pub struct Client {
    http: Rc<Http>,
}

impl Client {
    pub(crate) fn new(http: Rc<Http>) -> Self {
        Self { http }
    }

    /// Gets the current leaderboard.
    ///
    /// # Args
    ///
    /// - `platform`: The [`Platform`] for the leaderboard
    /// - `region`: Optionally filter for a [`Region`]
    ///
    /// [`Platform`]: ../../platform/enum.Platform.html
    /// [`Region`]: ../../region/enum.Region.html
    pub async fn get(
        &self,
        platform: Platform,
        region: Option<Region>,
    ) -> Result<Leaderboard, Error> {
        let response = self.request(platform, region).await?;
        let bytes = response.bytes().await?;
        Ok(serde_json::from_slice(&bytes)?)
    }

    async fn request(&self, platform: Platform, region: Option<Region>) -> Result<Response, Error> {
        let route = RouteBuilder::new()
            .platform(platform)
            .region(region)
            .build();

        let path = route.path();

        self.http.request(&path).await
    }
}
