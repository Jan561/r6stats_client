use serde::Deserialize;

#[derive(Deserialize, Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum Region {
    Ncsa,
    Emea,
    Apac,
}

impl Region {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Ncsa => "ncsa",
            Self::Emea => "emea",
            Self::Apac => "apac",
        }
    }
}
