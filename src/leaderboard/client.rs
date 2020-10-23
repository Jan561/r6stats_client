use super::http::RouteInfo;
use super::model::Leaderboard;
use crate::{Error, Http, Platform, Pointer, Region};
use reqwest::Response;

pub struct Client {
    http: Pointer<Http>,
}

impl Client {
    pub(crate) fn new(http: Pointer<Http>) -> Self {
        Self { http }
    }

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
        let route = RouteInfo { platform, region };
        let path = route.path();
        deref!(self.http).request(&path).await
    }
}
