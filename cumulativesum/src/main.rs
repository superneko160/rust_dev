use rand::Rng;
use std::time::Instant;
use cumulativesum::cumulative_sum::create_cumulative_sum;
use cumulativesum::cumulative_sum::range_cumulative_sum;

/// 単純計算と累積和の処理時間を計測し比較
fn main() {
    let n = 1_000_000;
    let list: Vec<i32> = vec![1; n];

    // 累積和リストの用意
    let start = Instant::now();
    let cum_sum = create_cumulative_sum(&list);
    let precompute_time = start.elapsed();

    let mut rng = rand::thread_rng();
    let queries = 2000;                                    // 試行回数
    let mut simple_time = std::time::Duration::new(0, 0);  // 単純計算の処理時間保存用
    let mut cum_time = std::time::Duration::new(0, 0);     // 累積和の処理時間保存用

    for i in 0..queries {
        let left = rng.gen_range(1..900_000);
        let right = left + rng.gen_range(1..100_000);

        // 単純計算
        let start = Instant::now();
        let simple_sum = simple_range_sum(&list, left, right);
        simple_time += start.elapsed();

        // 累積和を利用した計算
        let start = Instant::now();
        let cum_sum = range_cumulative_sum(&cum_sum, left, right);
        cum_time += start.elapsed();

        // 結果の確認（最初の3クエリのみ表示）
        if i < 3 {
            // 単純計算と累積和の結果が合致しているかチェック
            println!("試行回数 {}: 範囲 [{}, {}]", i + 1, left, right);
            println!("単純計算: {} / 累積和: {}\n", simple_sum, cum_sum);
        }
    }

    // 計算結果
    println!("累積和リストの作成時間: {:?}", precompute_time);
    println!("単純計算の総計算時間: {:?}", simple_time);
    println!("累積和利用計算の総計算時間: {:?}", cum_time);
    println!("累積和リストの総実行時間: {:?}", precompute_time + cum_time);
}

/// リストの合計の和を取得（累積和未使用版）
///
/// # Arguments
///
/// * `list` - 対象リスト
/// * `left` - 何番目の要素から
/// * `right` - 何番目の要素まで
///
/// # Returns
///
/// * `left`番目の要素から`right`番目の要素までの和（合計値）
fn simple_range_sum(list: &[i32], left: usize, right: usize) -> i32 {
    list[left - 1..right].iter().sum()
}
