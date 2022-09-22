use std::num::ParseIntError;

type AliasedResult<T> = Result<T, ParseIntError>;

/**
 * 与えられた文字列の数を数値に直して乗算する
 */
fn maltiply(first_num_str: &str, second_num_str: &str) -> AliasedResult<i32> {
    first_num_str.parse::<i32>().and_then(|first_num|{
        second_num_str.parse::<i32>().map(|second_num| first_num * second_num)
    })
}

/**
 * AliasedResit型を表示するための関数
 */
fn print(result: AliasedResult<i32>) {
    match result {
        Ok(n) => println!("n: {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

/**
 * エラーハンドリング
 */
fn main() {
    print(maltiply("10", "2"));  // Ok
    print(maltiply("t", "2"));  // Err
}