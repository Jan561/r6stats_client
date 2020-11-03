use crate::Error;
use serde::{Deserialize, Deserializer};

pub fn check_username(username: &str) -> Result<(), Error> {
    if valid_username(username) {
        Ok(())
    } else {
        Err(Error::UsernameMalformed)
    }
}

fn valid_username(username: &str) -> bool {
    const ALLOWED_SPECIAL_CHARS: &str = ".-_ ";

    if username.is_empty() {
        return false;
    }

    username
        .chars()
        .all(|c| c.is_alphanumeric() || ALLOWED_SPECIAL_CHARS.contains(c))
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
        let valid = "Hello.Hello Hello-Hello_Hello";
        let invalids = ["", "Hello/Hello", "Hello#Hello"];

        assert!(check_username(valid).is_ok());

        for &invalid in invalids.iter() {
            assert!(check_username(invalid).is_err());
        }
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
