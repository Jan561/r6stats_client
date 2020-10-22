use super::http::RouteInfo;
use super::{GenericStats, Kind, SeasonalStats};
use crate::http::Http;
use crate::{Error, Platform};
use reqwest::Response;
use std::rc::Rc;

pub struct Client {
    http: Rc<Http>,
}

impl Client {
    pub(crate) fn new(http: Rc<Http>) -> Self {
        Self { http }
    }

    pub async fn generic(&self, username: &str, platform: Platform) -> Result<GenericStats, Error> {
        let response = self.request(username, platform, Kind::Generic).await?;
        let bytes = response.bytes().await?;
        Ok(serde_json::from_slice(&bytes)?)
    }

    pub async fn seasonal(
        &self,
        username: &str,
        platform: Platform,
    ) -> Result<SeasonalStats, Error> {
        let response = self.request(username, platform, Kind::Seasonal).await?;
        let bytes = response.bytes().await?;
        Ok(serde_json::from_slice(&bytes)?)
    }

    async fn request(
        &self,
        username: &str,
        platform: Platform,
        kind: Kind,
    ) -> Result<Response, Error> {
        let path = RouteInfo {
            username: username.to_string(),
            platform,
            kind,
        }
        .path();
        self.http.request(&path).await
    }
}
