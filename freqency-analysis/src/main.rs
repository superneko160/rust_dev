use std::env;
use std::path::Path;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

/**
 * コマンドライン引数で指定されたテキストファイルに書かれた文字を頻度分析
 * ex) cargo run src/sample.txt
 * ex) cargo run /workspaces/rust_dev/readme.md
 */
fn main() {
    // 解析するファイルのパス設定
    let args: Vec<String> = env::args().collect();
    let text_path = Path::new(&args[1]);
    // 頻度分析
    print_result(sort_dict(freqency_analysis(get_text(text_path))));
}

/**
 * テキストを取得
 */
fn get_text(text_path: &Path) -> String {
    let file = match File::open(text_path) {
        Err(why) => panic!("{}", why),
        Ok(file) => file,
    };
    let mut file_contents = String::new();
    // ファイルを1行ずつ読み込み
    for line in BufReader::new(file).lines() {
        let l = line.expect("something went wrong reading the line");
        file_contents.push_str(&l.trim());  // 先頭と末尾の空白を除去
    }
    file_contents.to_string()
}

/**
 * ファイルに記された文字の出現回数を記録した辞書（連想配列）を作成
 */
fn freqency_analysis(file_contents: String) -> HashMap<char, u32> {
    let mut dict = HashMap::new();
    for c in file_contents.chars() {
        *dict.entry(c).or_insert(0) += 1;
    }
    dict
}

/**
 * 辞書（連想配列）をベクタ型に変換し、カウント数が大きい順にソート
 */
fn sort_dict(dict: HashMap<char, u32>) -> Vec<(char, u32)> {
    let mut vec: Vec<(char, u32)> = dict.into_iter().collect();
    vec.sort_by(|a, b| (b.1).cmp(&(a.1)));
    vec
}

/**
 * すべての文字数の合計を算出
 */
 fn count_all_char(vec: &Vec<(char, u32)>) -> u32 {
    let mut count = 0;
    for val in vec {
        count += val.1;
    }
    count
 }

/*
 * 割合を計算
 */
fn calc_rate(count: f32, all_count: f32) -> f32 {
   (count / all_count) * 100.0
}

/**
 * 解析結果の表示
 */
fn print_result(vec: Vec<(char, u32)>) {
    // 各文字の総合計を取得
    let all_char_count = count_all_char(&vec) as f32; 
    // テキスト内での文字の出現回数と比率を表示
    for val in vec {
        if val.0 == ' ' {
            println!("半角空白: {}回  {}%", val.1, calc_rate(val.1 as f32, all_char_count));
        }
        else if val.0 == '　' {
            println!("全角空白: {}回  {}%", val.1, calc_rate(val.1 as f32, all_char_count));
        }
        else {
            println!("{}: {}回  {}%", val.0, val.1, calc_rate(val.1 as f32, all_char_count));
        }
    }
}

#[cfg(test)]
mod test;