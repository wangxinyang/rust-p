use chrono::{DateTime, FixedOffset, Local, TimeZone, Utc};

fn main() {
    let now = Local::now();
    println!("now is: {now:?}");
    let mills = now.timestamp_millis();
    println!("mills is: {mills:?}");
    let now_from_mills = Local.timestamp_millis_opt(mills).unwrap();
    println!("now_from_mills is: {now_from_mills:?}");

    let now_utc = Utc::now();
    println!("now_utc is: {now_utc:?}");

    let date_time: DateTime<FixedOffset> = "2022-11-01T15:00:00+0800".parse().unwrap();
    println!("date_time is: {date_time:?}");
}
