use chrono::{DateTime, Datelike, Duration, Local, NaiveDate, NaiveDateTime, Utc, Weekday};

pub fn current_time_timestamp() -> i64 {
    Utc::now().timestamp()
}

pub fn current_time() -> DateTime<Local> {
    Local::now()
}

pub fn current_date() -> NaiveDate {
    current_time().date_naive()
}

pub fn is_timestamp_on_day(timestamp: i64, adjustment: i64) -> bool {
    let timestamp_date = NaiveDateTime::from_timestamp_opt(timestamp, 0)
        .expect("Invalid timestamp")
        .date();
    let today_date = (Local::now() + Duration::days(adjustment)).date_naive();
    timestamp_date == today_date
}

pub fn todays_weekday(adjustment: i64) -> Weekday {
    (current_time() + Duration::days(adjustment)).weekday()
}

pub fn is_same_date(some_date: NaiveDate) -> bool {
    let now_local = Local::now().date_naive();
    some_date == now_local
}
