use chrono::{Datelike, Local};

pub fn get_current_year() -> i32 {
    Local::now().year()
}