use chrono::{DateTime, Duration, Utc};
use rand::Rng;

type Error = String;

pub trait GenerateTime {
    fn generate_until(&self, end: &DateTime<Utc>) -> Result<Vec<DateTime<Utc>>, Error>;
    fn generate_until_with_limit(
        &self,
        end: &DateTime<Utc>,
        limit: usize,
    ) -> Result<Vec<DateTime<Utc>>, Error>;
}

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
            return Err(String::from(
                "start date/time is greater than end date/time",
            ));
        }

        let mut rng = rand::thread_rng();
        let mut result = vec![];

        // push the fist one
        result.push(self.clone());

        // generate in-between date-time's
        loop {
            let dt = result.last().unwrap().clone() + Duration::seconds(rng.gen_range(0..15));

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

    fn generate_until_with_limit(
        &self,
        end: &DateTime<Utc>,
        limit: usize,
    ) -> Result<Vec<DateTime<Utc>>, Error> {
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
            return Err(String::from(
                "start date/time is greater than end date/time",
            ));
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



#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{TimeZone, Utc};

    #[test]
    fn test_gen_with_limit() {
        let start = Utc.ymd(2001, 9, 9).and_hms_milli(0, 0, 0, 0);
        let end = Utc.ymd(2001, 9, 10).and_hms_milli(0, 0, 0, 0);

        let date_times = start.generate_until_with_limit(&end, 100);

        assert!(date_times.is_ok());
        assert_eq!(date_times.unwrap().len(), 100)
    }

    #[test]
    fn test_gen() {
        let start = Utc.ymd(2001, 9, 9).and_hms_milli(0, 0, 0, 0);
        let end = Utc.ymd(2001, 9, 10).and_hms_milli(0, 0, 0, 0);

        let date_times = start.generate_until(&end);

        assert!(date_times.is_ok());
    }

    #[test]
    fn test_start_greater_than_end() {
        let start = Utc.ymd(2001, 9, 10).and_hms_milli(0, 0, 0, 0);
        let end = Utc.ymd(2001, 9, 9).and_hms_milli(0, 0, 0, 0);

        let date_times = start.generate_until(&end);
        assert!(date_times.is_err());

        let date_times = start.generate_until_with_limit(&end, 100);
        assert!(date_times.is_err());
    }
}
