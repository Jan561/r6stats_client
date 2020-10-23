use super::http::RouteInfo;
use super::model::Leaderboard;
use crate::{Error, Http, Platform, Region};
use reqwest::Response;
use std::rc::Rc;

pub struct Client {
    http: Rc<Http>,
}

impl Client {
    pub(crate) fn new(http: Rc<Http>) -> Self {
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
        self.http.request(&path).await
    }
}
