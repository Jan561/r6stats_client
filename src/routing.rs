use crate::platform::Platform;
use crate::stats::Kind;

#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum Route {
    /// Route for the `/stats/:username/:platform/:kind` path
    Stats(StatsInfo),
}

impl Route {
    pub fn path(&self) -> String {
        match *self {
            Self::Stats(StatsInfo {
                ref username,
                platform,
                kind,
            }) => format!(
                api!("/stats/{u}/{p}/{k}"),
                u = username,
                p = platform.as_str(),
                k = kind.as_str(),
            ),
        }
    }
}

#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct StatsInfo {
    pub username: String,
    pub platform: Platform,
    pub kind: Kind,
}
