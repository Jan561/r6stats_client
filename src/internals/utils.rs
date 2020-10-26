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

#[cfg(test)]
mod tests {
    use super::{check_username, serde_parse_f64_option};

    #[test]
    fn test_check_username() {
        let valid = "pengu.g2";
        let invalid = "pengu.g2/pc/generic";

        assert!(check_username(valid).is_ok());
        assert!(check_username(invalid).is_err());
    }

    #[test]
    fn test_parse_f64() {
        #[derive(serde::Deserialize)]
        struct Test {
            #[serde(deserialize_with = "serde_parse_f64_option")]
            value: Option<f64>,
        }

        let json = r#"
        {
            "value": "0.000017"
        }
        "#;

        let test: Test = serde_json::from_str(json).unwrap();
        assert_eq!(test.value.unwrap(), 0.000017);
    }
}
