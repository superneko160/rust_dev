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
    month_to_month_percent: f32,  // 前月比（％）
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
    // print_data(get_data_mtm_percent_over_stdval(data, 4.0, true));  // 前月比+4%位以上
    // print_data(get_data_mtm_percent_over_stdval(data, -4.0, false));  // 前月比-4%以上
    print_data(get_data_target_month(data, 12));
}

/**
 * CSVデータの取得
 * @csv_path: &Path CSVファイルのパス
 * @return: Result<Vec<Data>> CSVから取得したデータ
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
 * 前月比（％）が基準値（stdval）以上または以下のデータのみ取得
* @pre_data: Vec<Data> CSVから取得したデータ
* @stdval: f32 基準値
* @over: bool true:基準値位以上、false:基準値以下
* @return: Vec<Data> 整形後データ
 */
fn get_data_mtm_percent_over_stdval(pre_data: Vec<Data>, stdval: f32, over: bool) -> Vec<Data> {
    let mut data = Vec::new();
    for val in pre_data {
        // 前月比が基準値以上だった月だけ取得
        if over {
            if val.month_to_month_percent >= stdval {
                data.push(val);
            }
        }
        // 前月比が基準値以下だった月だけ取得
        else {
            if val.month_to_month_percent <= stdval {
                data.push(val);
            }
        }
    }
    data
}

/**
 * 指定された月（target_month）のデータのみ取得
 * @pre_data: Vec<Data> CSVから取得したデータ
 * @target_month: u8 指定する月(1-12)
 * @return: Vec<Data> 整形後データ
 */
fn get_data_target_month(pre_data: Vec<Data>, target_month: u8) -> Vec<Data> {
    let mut data = Vec::new();
    // 1−12以外の数値が引数に指定された場合、空データを返す
    if target_month > 12 || target_month < 1 {
        println!("An invalid number is passed as an argument!");
        return data;
    }
    for val in pre_data {
        let tmp: Vec<&str> = val.date.split('/').collect();
        let month = tmp[1].parse::<u8>().unwrap();
        // 指定された月のデータのみ取得
        if month == target_month {
             data.push(val);
        }
    }
    data
}

/**
 * データの表示
 * @data: Vec<Data> 表示するデータ
 */
fn print_data(data: Vec<Data>) {
    for val in data {
        println!("{}  {}  {}%", val.date, val.month_to_month, val.month_to_month_percent);
    }
}