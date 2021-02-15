use chrono::{Utc, TimeZone};

use random::datetime::GenerateTime;

fn main() {
    let start = Utc.ymd(2001, 9, 9).and_hms_milli(0, 0, 0, 0);
    let end = Utc.ymd(2001, 9, 10).and_hms_milli(0, 0, 0, 0);

    let between = start.generate_until(&end).unwrap();
    println!("Time range between {} to {}", start, end);
    println!("{:?}", between);
    println!("Total= {}", between.len());
    println!();
    let between = start.generate_until_with_limit(&end, 100).unwrap();
    println!("Time range between {} to {}", start, end);
    println!("{:?}", between);
    println!("Total= {}", between.len());
}