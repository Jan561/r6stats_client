use serde::Deserialize;

#[derive(Deserialize, Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum Region {
    Ncsa,
    Emea,
    Apac,
}
