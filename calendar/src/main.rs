use chrono::prelude::*;
use chrono::Duration;
use chrono_tz::Asia::Tokyo;

#[derive(Debug)]
struct CalendarMonth {
    year: i32,
    month: u32,
    days: Vec<Option<u32>>,
}

/**
 * 現在の月のカレンダーを表示
 */
fn main() {
    let now = Utc::now().with_timezone(&Tokyo);
    let calendar = generate_calendar(now.year(), now.month());
    display_calendar(&calendar);
}

/// カレンダを作成
/// @param i32 year
/// @param u32 month
/// @return CalendarMonth
fn generate_calendar(year: i32, month: u32) -> CalendarMonth {
    // 現在の月の1日目の曜日を取得
    let first_dt = Tokyo.with_ymd_and_hms(year, month, 1, 0, 0, 0).unwrap();

    // 月末日を取得（今月の初日から+1ヶ月して-1日した日付を取得）
    let last_dt = (first_dt + Duration::days(32)).date_naive().with_day(1).unwrap() - Duration::days(1);

    let mut days = vec![None; first_dt.weekday().num_days_from_sunday() as usize];
    days.extend((1..=last_dt.day()).map(Some));

    CalendarMonth {
        year,
        month,
        days,
    }
}

/// カレンダを表示
/// @param &CalendarMonth
fn display_calendar(calendar: &CalendarMonth) {
    println!("{}年{}月のカレンダー", calendar.year, calendar.month);
    println!(" Sun Mon Tue Wed Thu Fri Sat");

    for (i, day) in calendar.days.iter().enumerate() {
        if i % 7 == 0 && i != 0 {
            println!();
        }
        match day {
            Some(d) => print!("{:4}", d),
            None => print!("    "),
        }
    }
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_calendar() {
        let calendar = generate_calendar(2023, 5);
        assert_eq!(calendar.year, 2023);
        assert_eq!(calendar.month, 5);
        assert_eq!(calendar.days[0], None);
        assert_eq!(calendar.days[1], Some(1));
        assert_eq!(calendar.days[31], Some(31));
    }

    #[test]
    fn test_display_calendar() {
        let calendar = CalendarMonth {
            year: 2023,
            month: 5,
            days: vec![None, Some(1), Some(2), Some(3), Some(4), Some(5), Some(6)],
        };

        // このテストは該当関数がパニックにならないことを保証するだけ
        display_calendar(&calendar);
    }
}
