use crate::Platform;
use crate::Region;

#[derive(Clone, Debug)]
pub struct RouteInfo {
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
