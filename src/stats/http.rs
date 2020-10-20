use super::Kind;
use crate::internals::utils::check_username;
use crate::platform::Platform;
use crate::Error;

#[derive(Clone, Debug, Default)]
pub(super) struct RouteBuilder {
    username: Option<String>,
    platform: Option<Platform>,
    kind: Option<Kind>,
}

impl RouteBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn username(mut self, username: String) -> Self {
        self.username = Some(username);
        self
    }

    pub fn platform(mut self, platform: Platform) -> Self {
        self.platform = Some(platform);
        self
    }

    pub fn kind(mut self, kind: Kind) -> Self {
        self.kind = Some(kind);
        self
    }

    pub fn build(self) -> Result<RouteInfo, Error> {
        let username = self
            .username
            .expect("Error creating route: Username missing.");
        let platform = self
            .platform
            .expect("Error creating route: Platform missing.");
        let kind = self.kind.expect("Error creating route: Kind missing.");

        check_username(&username)?;

        let route = RouteInfo {
            username,
            platform,
            kind,
        };
        Ok(route)
    }
}

#[derive(Clone, Debug)]
pub(super) struct RouteInfo {
    username: String,
    platform: Platform,
    kind: Kind,
}

impl RouteInfo {
    pub fn path(&self) -> String {
        format!(
            api!("/stats/{u}/{p}/{k}"),
            u = &self.username,
            p = self.platform.as_str(),
            k = self.kind.as_str()
        )
    }
}
