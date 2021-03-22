use csv;
use rand::{seq::SliceRandom, thread_rng};
use serde::{Deserialize, Deserializer, Serialize};

type Error = String;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Tz {
    alpha_2_code: String,
    alpha_3_code: String,
    continent: String,
    capital: String,
    name: String,
    #[serde(deserialize_with = "string_to_vec")]
    timezones: Vec<String>,
}

fn string_to_vec<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
    where
        D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    Ok(s.split(",").map(|x| x.to_string()).collect::<Vec<String>>())
}

impl Tz {
    fn load() -> Result<Vec<Self>, Error> {
        //! Loads the Timezones Database
        read_from_tz()
    }

    pub fn get_random_tz() -> Result<Tz, Error> {
        //! Gets a Random timezone
        let tzs = match Self::load() {
            Ok(t) => t,
            Err(e) => return Err(format!("Error: {}", e.to_string())),
        };
        let res = tzs.choose(&mut thread_rng()).unwrap();
        Ok(res.clone())
    }

    pub fn tz_by_iso_code(iso_code: &str) -> Result<Option<String>, Error> {
        //! Get the countries time zone information based upon
        //! a 2 letter iso country code or 3 letter iso country code
        Self::tz_query(iso_code)
    }

    pub fn tz_by_country(country_name: &str) -> Result<Option<String>, Error> {
        //! Get the countries time zone information based upon
        //! a given country name
        Self::tz_query(country_name)
    }

    fn tz_query(q: &str) -> Result<Option<String>, Error> {
        let mut tzs = match Self::load() {
            Ok(t) => t,
            Err(e) => return Err(format!("Error: {}", e.to_string())),
        };

        let result: String = tzs
            .drain(..)
            .filter(|x| match q.len() {
                2 => x.alpha_2_code.eq_ignore_ascii_case(q),
                3 => x.alpha_3_code.eq_ignore_ascii_case(q),
                _ => x.name.eq_ignore_ascii_case(q),
            })
            .map(|x| x.timezones.join(","))
            .collect();
        if !result.is_empty() {
            Ok(Some(result))
        } else {
            Ok(None)
        }
    }
}

fn csv_de(csv_text: &str) -> Result<Vec<Tz>, csv::Error> {
    csv::Reader::from_reader(csv_text.as_bytes())
        .deserialize()
        .collect()
}

pub(crate) fn read_from_tz() -> Result<Vec<Tz>, Error> {
    let csv_text = include_str!("../assets/tz.csv");
    let records = match csv_de(csv_text.trim()) {
        Ok(r) => r,
        Err(e) => return Err(format!("Error: Not a valid tz csv file. {}", e.to_string())),
    };
    Ok(records)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_load_tz() {
        let tzs = read_from_tz();
        assert!(tzs.is_ok())
    }

    #[test]
    fn test_tz_lookup_by_iso_code() {
        let tz = Tz::tz_by_iso_code("IN");
        assert!(tz.is_ok());

        let tz = tz.unwrap();
        assert!(tz.is_some());
        assert_eq!(tz.unwrap(), "Asia/Calcutta,Asia/Kolkata".to_string());

        let tz = Tz::tz_by_iso_code("InD");
        assert!(tz.is_ok());

        let tz = tz.unwrap();
        assert!(tz.is_some());
        assert_eq!(tz.unwrap(), "Asia/Calcutta,Asia/Kolkata".to_string());
    }

    #[test]
    fn test_tz_lookup_by_iso_code_bad_code() {
        let tz = Tz::tz_by_iso_code("I");
        assert!(tz.is_ok());

        let tz = tz.unwrap();
        assert!(tz.is_none());
    }

    #[test]
    fn test_tz_lookup_by_country_name() {
        let tz = Tz::tz_by_country("Monaco");
        assert!(tz.is_ok());

        let tz = tz.unwrap();
        assert!(tz.is_some());
        assert_eq!(tz.unwrap(), "Europe/Monaco".to_string());
    }

    #[test]
    fn test_tz_lookup_by_country_name_bad_name() {
        let tz = Tz::tz_by_country("Monako");
        assert!(tz.is_ok());

        let tz = tz.unwrap();
        assert!(tz.is_none());
    }

    #[test]
    fn test_get_random_tz() {
        let tz = Tz::get_random_tz();
        assert!(tz.is_ok());
    }
}