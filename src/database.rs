/*
Database related generators
*/

pub fn mongo_objectid() -> String {
    let nowtime = chrono::Utc::now();

    let timestamp = nowtime.timestamp();
    println!("timestamp [{:02x}]", timestamp);

    let mut tsstr = format!("{:02x}", timestamp);
    for _ in 0..8 {
        let x = format!("{:02x}", rand::random::<u8>());
        tsstr.push_str(&x);
    }

    tsstr
}
