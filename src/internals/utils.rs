use crate::{Error, Platform};
use serde::{Deserialize, Deserializer};

pub fn check_username(username: &str, platform: Platform) -> Result<(), Error> {
    if username.is_empty() {
        return Err(Error::UsernameMalformed);
    }

    match platform {
        Platform::Pc => check_username_pc(username),
        Platform::Xbox => check_username_xbox(username),
        Platform::Playstation => check_username_playstation(username),
    }
}

fn check_username_pc(username: &str) -> Result<(), Error> {
    const ALLOWED_SPECIAL_CHARS: &str = ".-_";

    _check_username(username, ALLOWED_SPECIAL_CHARS)
}

fn check_username_xbox(username: &str) -> Result<(), Error> {
    _check_username(username, " ")
}

fn check_username_playstation(username: &str) -> Result<(), Error> {
    const ALLOWED_SPECIAL_CHARACTERS: &str = "-_";

    _check_username(username, ALLOWED_SPECIAL_CHARACTERS)
}

fn _check_username(username: &str, special_chars: &str) -> Result<(), Error> {
    if valid_username(username, special_chars) {
        Ok(())
    } else {
        Err(Error::UsernameMalformed)
    }
}

fn valid_username(username: &str, special_chars: &str) -> bool {
    username
        .chars()
        .all(|c| c.is_alphanumeric() || special_chars.contains(c))
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
    use crate::Platform;

    #[test]
    fn test_check_username() {
        // PC
        let valid = "pengu.g2";
        let invalids = ["", "pengu.g2/pc/generic", "pengu g2"];

        assert!(check_username(valid, Platform::Pc).is_ok());

        for &invalid in invalids.iter() {
            assert!(check_username(invalid, Platform::Pc).is_err());
        }

        // Xbox
        let valid = "Hello Hello";
        let invalids = ["", "Hello.Hello", "Hello/Hello"];

        assert!(check_username(valid, Platform::Xbox).is_ok());

        for &invalid in invalids.iter() {
            assert!(check_username(invalid, Platform::Xbox).is_err());
        }

        // Playstation
        let valid = "Hello_Hello-Hello";
        let invalids = ["", "Hello.Hello", "Hello Hello", "Hello/Hello"];

        assert!(check_username(valid, Platform::Playstation).is_ok());

        for &invalid in invalids.iter() {
            assert!(check_username(invalid, Platform::Playstation).is_err());
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
