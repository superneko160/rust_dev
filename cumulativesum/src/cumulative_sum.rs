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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_cumulative_sum() {
        // 通常のケース
        let list = vec![1, 2, 3, 4, 5];
        let expected = vec![0, 1, 3, 6, 10, 15];
        assert_eq!(create_cumulative_sum(&list), expected);

        // 空のリスト
        let empty_list: Vec<i32> = vec![];
        let expected_empty = vec![0];
        assert_eq!(create_cumulative_sum(&empty_list), expected_empty);

        // 負の数を含むリスト
        let negative_list = vec![-1, 2, -3, 4, -5];
        let expected_negative = vec![0, -1, 1, -2, 2, -3];
        assert_eq!(create_cumulative_sum(&negative_list), expected_negative);

        // 大きな数を含むリスト
        let large_numbers = vec![1_000_000, 2_000_000, 3_000_000];
        let expected_large = vec![0, 1_000_000, 3_000_000, 6_000_000];
        assert_eq!(create_cumulative_sum(&large_numbers), expected_large);
    }

    #[test]
    fn test_range_cumulative_sum() {
        let cum_sum = vec![0, 1, 3, 6, 10, 15];

        // 通常のケース
        assert_eq!(range_cumulative_sum(&cum_sum, 2, 4), 9); // 2 + 3 + 4 = 9
        assert_eq!(range_cumulative_sum(&cum_sum, 1, 5), 15); // 1 + 2 + 3 + 4 + 5 = 15

        // 単一要素の範囲
        assert_eq!(range_cumulative_sum(&cum_sum, 3, 3), 3);

        // 全範囲
        assert_eq!(range_cumulative_sum(&cum_sum, 1, 5), 15);

        // 負の数を含む累積和での範囲計算
        let negative_cum_sum = vec![0, -1, 1, -2, 2, -3];
        assert_eq!(range_cumulative_sum(&negative_cum_sum, 1, 5), -3); // -1 + 2 + (-3) + 4 + (-5) = -3
    }

    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn test_range_cumulative_sum_out_of_bounds() {
        let cum_sum = vec![0, 1, 3, 6, 10, 15];
        range_cumulative_sum(&cum_sum, 0, 6); // インデックスは0から始まるため、6は範囲外
    }
}
