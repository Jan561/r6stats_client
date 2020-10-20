use crate::Error;
use serde::{Deserialize, Deserializer};

pub fn check_username(username: &str) -> Result<(), Error> {
    if username.contains('/') {
        Err(Error::UsernameMalformed)
    } else {
        Ok(())
    }
}

pub fn serde_parse_f64_option<'de, D>(deserializer: D) -> Result<Option<f64>, D::Error>
where
    D: Deserializer<'de>,
{
    let opt: Option<String> = Option::deserialize(deserializer)?;

    match opt {
        Some(s) => parse_f64(&s).map(Some),
        None => Ok(None),
    }
}

fn parse_f64<E: serde::de::Error>(s: &str) -> Result<f64, E> {
    s.parse().map_err(serde::de::Error::custom)
}
