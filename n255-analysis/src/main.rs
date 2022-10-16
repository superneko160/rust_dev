use std::fs;
use std::path::Path;
use serde::Deserialize;
use anyhow::Result;

// 数値がStringなのは値にカンマが入っているため
#[derive(Debug, Deserialize)]
struct Data {
    date: String,  // 日付
    open: String,  // 始値
    high: String,  // 高値
    low: String,  // 安値
    close: String,  // 終値
    month_to_month: String,  // 前月比
    month_to_month_percent: String,  // 前月比（％）
    volume: String,  // 売買高
}

/**
 * 日経平均のCSVデータを解析
 */
fn main() {
    // パス設定
    let csv_path = Path::new("/workspaces/rust_dev/n255-analysis/data/n255mtm_2013-2022.csv");
    // データの取得
    let data = match read_csv(csv_path) {
        Ok(data) => data,
        Err(e) => panic!("{}", e),
    };
    // データの表示
    print_data(data);
}

/**
 * CSVデータの取得
 */
fn read_csv(csv_path: &Path) -> Result<Vec<Data>> {
    let mut data = Vec::new();
    let csv_text = fs::read_to_string(csv_path)?;
    let mut rdr = csv::Reader::from_reader(csv_text.as_bytes());
    for result in rdr.records() {
        let record = result?.deserialize(None)?;
        data.push(record);
    }
    Ok(data)
}

/**
 * データの表示
 */
fn print_data(data: Vec<Data>) {
    for val in data {
        // 前月比がプラスだった月だけ表示
        let mtm_percent: f32 = val.month_to_month_percent.parse().unwrap();
        if mtm_percent > 0.0 {
            println!("{} {}", val.date, val.month_to_month_percent);
        }
    }
}