use super::Kind;
use crate::platform::Platform;

#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct RouteInfo {
    pub username: String,
    pub platform: Platform,
    pub kind: Kind,
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
