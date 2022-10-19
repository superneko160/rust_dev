use std::process;
use n255analysis::analysis;

/**
 * 日経平均のCSVデータを解析
 */
fn main() {
    // データの取得
    let data = match analysis::read_csv() {
        Ok(data) => data,
        Err(e) => {
            println!("Error: {}", e);
            process::exit(1);
        }
    };
    // データの表示
    // analysis::print_data(analysis::get_data_mtm_percent_over_stdval(data, 4.0, true));  // 前月比+4%位以上
    // analysis::print_data(analysis::get_data_mtm_percent_over_stdval(data, -4.0, false));  // 前月比-4%以上
    // analysis::print_data(analysis::get_data_target_ym(data, 0, 0));  // 全データを取得
    // analysis::print_data(analysis::get_data_target_ym(data, 15, 0));  // 2015年の全データを取得
    // analysis::print_data(analysis::get_data_target_ym(data, 0, 1));  // すべての年の1月のデータを取得
    analysis::print_data(analysis::get_data_target_ym(data, 15, 1));  // 2015年1月のデータを取得
}