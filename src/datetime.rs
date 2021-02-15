#[cfg(feature = "timegenerate")]
use chrono::{DateTime, Utc, Duration};
#[cfg(feature = "tz")]
use csv;
#[cfg(feature = "tz")]
use rand::{Rng, seq::SliceRandom, thread_rng};
#[cfg(feature = "tz")]
use serde::{Serialize, Deserialize, Deserializer};

type Error = String;

#[cfg(feature = "timegenerate")]
pub trait GenerateTime {
    fn generate_until(&self, end: &DateTime<Utc>) -> Result<Vec<DateTime<Utc>>, Error>;
    fn generate_until_with_limit(&self, end: &DateTime<Utc>, limit: usize) -> Result<Vec<DateTime<Utc>>, Error>;
}

#[cfg(feature = "timegenerate")]
impl GenerateTime for DateTime<Utc> {
    fn generate_until(&self, end: &DateTime<Utc>) -> Result<Vec<DateTime<Utc>>, Error> {
        //! Generate times between `self` (start) and `end`
        //!
        //! ## Example
        //! ```rust
        //! use chrono::{Utc, DateTime, TimeZone};
        //! use random::datetime::GenerateTime;
        //!
        //! fn main () {
        //!     let start = Utc.ymd(2001, 9, 9).and_hms_milli(0, 0, 0, 0);
        //!     let end = Utc.ymd(2001, 9, 10).and_hms_milli(0, 0, 0, 0);
        //!
        //!     let date_times = start.generate_until(&end);
        //!
        //!     println!("{:#?}", date_times);
        //!     println!("total: {}", date_times.unwrap().len());
        //! }
        //! ```
        if self.ge(&end) {
            return Err(
                String::from("start date/time is greater than end date/time")
            )
        }

        let mut rng = rand::thread_rng();
        let mut result = vec![];

        // push the fist one
        result.push(self.clone());

        // generate in-between date-time's
        loop {
            let dt = result
                .last()
                .unwrap()
                .clone() + Duration::seconds(rng.gen_range(0..15));

            if end <= &dt {
                break;
            } else {
                result.push(dt);
            }
        }

        // push the last one
        result.push(end.clone());

        result.sort();
        Ok(result)
    }

    fn generate_until_with_limit(&self, end: &DateTime<Utc>, limit: usize) -> Result<Vec<DateTime<Utc>>, Error> {
        //! Generate times between `self` (start) and `end`
        //! with a range limit
        //!
        //! ## Example
        //! ```rust
        //! use chrono::{Utc, DateTime, TimeZone};
        //! use random::datetime::GenerateTime;
        //!
        //! fn main () {
        //!     let start = Utc.ymd(2001, 9, 9).and_hms_milli(0, 0, 0, 0);
        //!     let end = Utc.ymd(2001, 9, 10).and_hms_milli(0, 0, 0, 0);
        //!
        //!     let date_times = start.generate_until_with_limit(&end, 100);
        //!
        //!     println!("{:#?}", date_times);
        //!     println!("total: {}", date_times.unwrap().len());
        //! }
        //! ```
        if self.ge(&end) {
            return Err(
                String::from("start date/time is greater than end date/time")
            )
        }
        let time_delta = end.signed_duration_since(*self);
        let mut rng = rand::thread_rng();
        let mut result = vec![];

        // generate in-between date-time's
        for _ in 0..limit {
            let nanosecs = rng.gen_range(0..time_delta.num_nanoseconds().unwrap());
            result.push(*self + Duration::nanoseconds(nanosecs));
        }

        result.sort();
        Ok(result)
    }
}

#[cfg(feature = "tz")]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Tz {
    alpha_2_code: String,
    alpha_3_code: String,
    continent: String,
    capital: String,
    name: String,
    #[serde(deserialize_with = "string_to_vec")]
    timezones: Vec<String>
}

#[cfg(feature = "tz")]
fn string_to_vec<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
    where
        D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    Ok(s
        .split(",")
        .map(|x|x.to_string())
        .collect::<Vec<String>>()
    )
}

#[cfg(feature = "tz")]
impl Tz {
    fn load() -> Result<Vec<Self>, Error> {
        //! Loads the Timezones Database
        read_from_tz()
    }

    pub fn get_random_tz() -> Result<Tz, Error> {
        //! Gets a Random timezone
        let tzs = match Self::load() {
            Ok(t) => t,
            Err(e) => return Err(format!("Error: {}", e.to_string()))
        };
        let res = tzs
            .choose(&mut thread_rng())
            .unwrap();
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
            Err(e) => return Err(format!("Error: {}", e.to_string()))
        };

        let result: String = tzs
            .drain(..)
            .filter(|x|{
                match q.len() {
                    2 => x.alpha_2_code.eq_ignore_ascii_case(q),
                    3 => x.alpha_3_code.eq_ignore_ascii_case(q),
                    _ => x.name.eq_ignore_ascii_case(q)
                }
            })
            .map(|x|x.timezones.join(","))
            .collect();
        if !result.is_empty() {
            Ok(
                Some(result)
            )
        } else { Ok(None) }
    }
}

#[cfg(feature = "tz")]
fn csv_de(csv_text: &str) -> Result<Vec<Tz>, csv::Error> {
    csv::Reader::from_reader(csv_text.as_bytes())
        .deserialize()
        .collect()
}

#[cfg(feature = "tz")]
pub(crate) fn read_from_tz() -> Result<Vec<Tz>, Error> {
    let csv_text = include_str!("../assets/tz.csv");
    let records = match csv_de(csv_text.trim()) {
        Ok(r) => r,
        Err(e) => return Err(
            format!("Error: Not a valid tz csv file. {}", e.to_string())
        )
    };
    Ok(records)
}

#[cfg(test)]
mod tests {
    use chrono::{Utc, TimeZone};
    use super::*;

    #[test]
    #[cfg(feature = "timegenerate")]
    fn test_gen_with_limit() {
        let start = Utc.ymd(2001, 9, 9).and_hms_milli(0, 0, 0, 0);
        let end = Utc.ymd(2001, 9, 10).and_hms_milli(0, 0, 0, 0);

        let date_times = start.generate_until_with_limit(&end, 100);

        assert!(date_times.is_ok());
        assert_eq!(date_times.unwrap().len(), 100)
    }

    #[test]
    #[cfg(feature = "timegenerate")]
    fn test_gen() {
        let start = Utc.ymd(2001, 9, 9).and_hms_milli(0, 0, 0, 0);
        let end = Utc.ymd(2001, 9, 10).and_hms_milli(0, 0, 0, 0);

        let date_times = start.generate_until(&end);

        assert!(date_times.is_ok());
    }

    #[test]
    #[cfg(feature = "timegenerate")]
    fn test_start_greater_than_end() {
        let start = Utc.ymd(2001, 9, 10).and_hms_milli(0, 0, 0, 0);
        let end = Utc.ymd(2001, 9, 9).and_hms_milli(0, 0, 0, 0);

        let date_times = start.generate_until(&end);
        assert!(date_times.is_err());

        let date_times = start.generate_until_with_limit(&end, 100);
        assert!(date_times.is_err());
    }

    #[test]
    #[cfg(feature = "tz")]
    fn test_load_tz() {
        let tzs = read_from_tz();
        assert!(tzs.is_ok())
    }

    #[test]
    #[cfg(feature = "tz")]
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
    #[cfg(feature = "tz")]
    fn test_tz_lookup_by_iso_code_bad_code() {
        let tz = Tz::tz_by_iso_code("I");
        assert!(tz.is_ok());

        let tz = tz.unwrap();
        assert!(tz.is_none());
    }

    #[test]
    #[cfg(feature = "tz")]
    fn test_tz_lookup_by_country_name() {
        let tz = Tz::tz_by_country("Monaco");
        assert!(tz.is_ok());

        let tz = tz.unwrap();
        assert!(tz.is_some());
        assert_eq!(tz.unwrap(), "Europe/Monaco".to_string());
    }

    #[test]
    #[cfg(feature = "tz")]
    fn test_tz_lookup_by_country_name_bad_name() {
        let tz = Tz::tz_by_country("Monako");
        assert!(tz.is_ok());

        let tz = tz.unwrap();
        assert!(tz.is_none());
    }

    #[test]
    #[cfg(feature = "tz")]
    fn test_get_random_tz() {
        let tz = Tz::get_random_tz();
        assert!(tz.is_ok());
    }
}