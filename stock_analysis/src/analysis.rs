use std::fs;
use std::path::Path;
use serde::Deserialize;
use anyhow::Result;

// 定数
// const CSV_PATH: &str = "/workspaces/rust_dev/stock_analysis/data/n255mtm_2013-2022.csv";
// const CSV_PATH: &str = "/workspaces/rust_dev/stock_analysis/data/n255dtd_20210804-20221020.csv";
const CSV_PATH: &str = "/workspaces/rust_dev/stock_analysis/data/topixdtd_20210804-20221020.csv";

// 数値がStringなのは値にカンマが入っているため
#[derive(Debug, Deserialize)]
pub struct Data {
    date: String,  // 日付
    // open: String,  // 始値
    // high: String,  // 高値
    // low: String,  // 安値
    // close: String,  // 終値
    previous: String,  // 前日比・前月比
    previous_ratio: f32,  // 前日比・前日比（％）
    // volume: String,  // 売買高
}

/// CSVデータの取得
///
/// # Returns
///
/// * CSVから取得したデータ
pub fn read_csv() -> Result<Vec<Data>> {
    let csv_path = Path::new(CSV_PATH);
    let mut data = Vec::new();
    let csv_text = fs::read_to_string(csv_path)?;
    let mut rdr = csv::Reader::from_reader(csv_text.as_bytes());
    for record in rdr.records() {
        let tmp = record?.deserialize(None)?;
        data.push(tmp);
    }
    Ok(data)
}

/// 前日比・前月比（％）が基準値（stdval）以上または以下のデータのみ取得
///
/// # Arguments
///
/// * `pre_data` - CSVから取得したデータ
/// * `stdval` - 基準値
/// * `over` - true:基準値位以上、false:基準値以下
///
/// # Returns
///
/// * 整形後データ
pub fn get_data_mtm_percent_over_stdval(pre_data: Vec<Data>, stdval: f32, over: bool) -> Vec<Data> {
    let mut data = Vec::new();
    for val in pre_data {
        // 前日比・前月比が基準値以上だった月だけ取得
        if over {
            if val.previous_ratio >= stdval {
                data.push(val);
            }
        }
        // 前日比・前月比が基準値以下だった月だけ取得
        else {
            if val.previous_ratio <= stdval {
                data.push(val);
            }
        }
    }
    data
}

/// 指定された月のデータのみ取得
///
/// # Arguments
///
/// * `pre_data` - CSVから取得したデータ
/// * `target_year` - 指定する年(指定しない場合は0を設定、指定する場合20xx年のxx(下2桁)指定)
/// * `target_month` - 指定する月(指定しない場合0を設定)
///
/// # Returns
///
/// * 整形後データ
pub fn get_data_target_ym(pre_data: Vec<Data>, target_year: u8, target_month: u8) -> Vec<Data> {
    let mut data = Vec::new();
  // 0−12以外の数値が月指定の引数に設定された場合、空データを返す
    if target_month > 12 {
        println!("An invalid number is passed as an argument: target_month");
        return data;
    }
    for val in pre_data {
        // 日付情報dateを年・月・日に分割し、数値に変換
        let tmp: Vec<&str> = val.date.split('/').collect();
        let year = tmp[0].parse::<u8>().unwrap();
        let month = tmp[1].parse::<u8>().unwrap();
        // 年指定無 and 月指定無
        if target_year == 0 && target_month == 0 {
            data.push(val);
        }
        // 年指定有 and 月指定無
        else if target_year != 0 && target_month == 0 {
            if target_year == year {
                data.push(val);
            }
        }
        // 年指定無 and 月指定有
        else if target_year == 0 && target_month <= 12 {
            if target_month == month {
                data.push(val);
            }
        }
        // 年指定有 and 月指定有
        else if target_year != 0 && target_month <= 12 {
            if target_year == year && target_month == month {
                data.push(val);
            }
        }
    }
    data
}

/// データの表示
///
/// # Arguments
///
/// * `data` - 表示するデータ
pub fn print_data(data: Vec<Data>) {
    for val in data {
        println!("{}  {}  {}%", val.date, val.previous, val.previous_ratio);
    }
}
