use std::fs;
use std::process;
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
        Err(e) => {
            println!("Error: {}", e);
            process::exit(1);
        }
    };
    // データの表示
    print_data(get_data_mtm_percent_over_stdval(data, 2.0));
}

/**
 * CSVデータの取得
 */
fn read_csv(csv_path: &Path) -> Result<Vec<Data>> {
    let mut data = Vec::new();
    let csv_text = fs::read_to_string(csv_path)?;
    let mut rdr = csv::Reader::from_reader(csv_text.as_bytes());
    for record in rdr.records() {
        let tmp = record?.deserialize(None)?;
        data.push(tmp);
    }
    Ok(data)
}

/**
 * 前月比（％）が基準値（stdval）以上のデータだけを取得
 */
fn get_data_mtm_percent_over_stdval(pre_data: Vec<Data>, stdval: f32) -> Vec<Data> {
    let mut data = Vec::new();
    for val in pre_data {
        let mtm_percent: f32 = val.month_to_month_percent.parse().unwrap();
        // 前月比が基準値以上だった月だけ表示
        if mtm_percent >= stdval {
            data.push(val);
        }
    }
    data
}

/**
 * データの表示
 */
fn print_data(data: Vec<Data>) {
    for val in data {
        println!("{}  {}  {}%", val.date, val.month_to_month, val.month_to_month_percent);
    }
}