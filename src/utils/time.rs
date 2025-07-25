use chrono::{Duration, Local, NaiveTime};

pub fn get_today() -> String {
    Local::now().format("%d-%m-%y").to_string()
}

pub fn get_seconds_to_midnight() -> i64 {
    let now = Local::now();
    let tomorrow_midnight = (now + Duration::days(1))
        .date_naive()
        .and_time(NaiveTime::from_hms_opt(0, 0, 0).unwrap())
        .and_local_timezone(Local)
        .unwrap();
    let duration = tomorrow_midnight - now;

    duration.num_seconds()
}
