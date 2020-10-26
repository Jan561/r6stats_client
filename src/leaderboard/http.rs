use crate::{Platform, Region};

#[derive(Clone, Debug, Default)]
pub(crate) struct RouteBuilder {
    platform: Option<Platform>,
    region: Option<Region>,
}

impl RouteBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn platform(mut self, platform: Platform) -> Self {
        self.platform = Some(platform);
        self
    }

    pub fn region(mut self, region: Option<Region>) -> Self {
        self.region = region;
        self
    }

    pub fn build(self) -> RouteInfo {
        RouteInfo {
            platform: self
                .platform
                .expect("Error creating route: Platform missing."),
            region: self.region,
        }
    }
}

#[derive(Clone, Debug)]
pub(crate) struct RouteInfo {
    pub platform: Platform,
    pub region: Option<Region>,
}

impl RouteInfo {
    pub fn path(&self) -> String {
        let region = self.region.map(Region::as_str).unwrap_or("all");
        format!(
            api!("/leaderboard/{p}/{r}"),
            p = self.platform.as_str(),
            r = region
        )
    }
}

#[cfg(test)]
mod tests {
    use super::RouteBuilder;
    use crate::{Platform, Region};

    #[test]
    fn test_routing_all_regions() {
        let route = RouteBuilder::new().platform(Platform::Pc).build();

        assert_eq!(route.path(), api!("/leaderboard/pc/all"));
    }

    #[test]
    fn test_routing_with_region() {
        let route = RouteBuilder::new()
            .platform(Platform::Pc)
            .region(Some(Region::Apac))
            .build();

        assert_eq!(route.path(), api!("/leaderboard/pc/apac"));
    }
}
