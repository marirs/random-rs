use random::datetime::Tz;

fn main() {
    let tz = Tz::tz_by_iso_code("IN");
    println!("{:?}", tz);

    let random_tz = Tz::get_random_tz().unwrap();
    println!("{:#?}", random_tz);
}