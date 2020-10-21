use serde::Deserialize;

#[derive(Deserialize, Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Region {
    #[serde(rename = "ncsa")]
    Ncsa,
    #[serde(rename = "emea")]
    Emea,
    #[serde(rename = "apac")]
    Apac,
}
