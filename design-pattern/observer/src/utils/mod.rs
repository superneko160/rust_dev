use chrono::{DateTime, Local};

pub fn get_current_datetime() -> String {
    let now: DateTime<Local> = Local::now();
    now.format("%Y/%m/%d %H:%M:%S").to_string()
}
