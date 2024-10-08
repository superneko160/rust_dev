/// 累積和リストを作成
///
/// # Arguments
///
/// * `list` - 対象リスト
///
/// # Returns
///
/// * 累積和リスト
pub fn create_cumulative_sum(list: &[i32]) -> Vec<i32> {
    let mut cum_sum = vec![0; list.len() + 1];
    for i in 1..=list.len() {
        cum_sum[i] = cum_sum[i - 1] + list[i - 1];
    }
    cum_sum
}

/// 累積和リストから指定された範囲の和を取得
///
/// # Arguments
///
/// * `cum_sum` - 累積和リスト
/// * `left` - 何番目の要素から
/// * `right` - 何番目の要素まで
///
/// # Returns
///
/// * `left`番目の要素から`right`番目の要素までの和（合計値）
pub fn range_cumulative_sum(cum_sum: &[i32], letf: usize, right: usize) -> i32 {
    cum_sum[right] - cum_sum[letf - 1]
}
