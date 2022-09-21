extern crate chrono;
extern crate chrono_tz;

use chrono::prelude::*;
use chrono::Duration;
use chrono_tz::Asia::Tokyo;

/**
 * 現在の月のカレンダーを表示
 */
fn main() {
    // 現在の月を取得
    let utc = Utc::now().naive_utc();
    let now = Tokyo.from_utc_datetime(&utc);
    let (_is_common_era, year) = now.year_ce();
    println!("今月:{}-{:02}-{:02}({:?})", year as i32, now.month(), now.day(), now.weekday());
    // 現在の月の1日目の曜日を取得
    let first_dt = Tokyo.ymd(year as i32, now.month(), 1);
    println!("月初:{}-{:02}-{:02}({:?})", year, first_dt.month(), first_dt.day(), first_dt.weekday());
    // 月末日を取得（今月の初日から+1ヶ月して-1日した日付を取得）
    let last_dt = Tokyo.ymd(year as i32, now.month()+1, 1) + Duration::days(-1);
    println!("月末:{}-{:02}-{:02}({:?})", year, last_dt.month(), last_dt.day(), last_dt.weekday());
    // 曜日
    let weekday: [&str; 7] = ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];
    // 月初日の曜日からスタート地点を決定
    let mut start_p = 0;
    for i in 0..7 {
        if weekday[i] == &first_dt.weekday().to_string() {
            start_p = i;
            break;
        }
    }
    // カレンダ作成
    let mut cal = Vec::new();
    for _i in 0..start_p {
        cal.push(99);
    }
    for i in 1..last_dt.day()+1 {
        cal.push(i);
    }
   // カレンダ表示
    for elm in weekday {
        print!(" {}", elm);
    }
    for (i, val) in cal.iter().enumerate() {  // val: &u32
        if i % 7 == 0 {
            println!();
        }
        if val == &99 {
            print!("    ");
        }
        else if val != &99 && val >= &10 {
            print!("  {}", val);
        }
        else {
            print!("   {}", val);
        }
    }
    println!();
}