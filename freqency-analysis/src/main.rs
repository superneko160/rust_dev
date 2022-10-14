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
 * 解析結果の表示
 */
fn print_result(vec: Vec<(char, u32)>) {
    for val in vec {
        if val.0 == ' ' {
            println!("半角空白: {}回", val.1)
        }
        else if val.0 == '　' {
            println!("全角空白: {}回", val.1)
        }
        else {
            println!("{}: {}回", val.0, val.1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_text() {
        let text_path = Path::new(&"/workspaces/rust_dev/freqency-analysis/src/sample.txt");
        let result = String::from("ABa　あCDF/$XYZ XAXXX");
        assert_eq!(result, get_text(text_path));
    }

    #[test]
    fn test_freqency_analysis() {
        let file_contents = String::from("aBBc");
        let result = HashMap::from([('a',1),('B',2),('c',1)]);
        assert_eq!(result, freqency_analysis(file_contents));
    }

    #[test]
    fn test_sort_dict() {
        let dict = HashMap::from([('a',2),('b',1),('c',3)]);
        let result: Vec<(char, u32)> = vec![('c',3),('a',2),('b',1)];
        assert_eq!(result, sort_dict(dict));
    }
}