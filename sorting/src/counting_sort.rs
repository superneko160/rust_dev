/// カウンティングソート（破壊的）
/// 同じ値が複数ある場合に、計算量が少なくなり、処理速度が向上する可能性有
///
/// # Arguments
///
/// * `list` - 並び替える要素が格納されたリスト
/// * `maxval` - 入力がとり得る最大値（[2, 15, 7]の場合15）
pub fn counting_sort(list: &mut [usize], maxval: usize) {

    // カウント用ベクタ
    let mut counter: Vec<usize> = vec![0; maxval + 1];

    // 各要素の出現回数をカウント
    for &data in list.iter() {
        counter[data as usize] += 1;
    }

    // カウント用ベクタを走査し、ソートしたリストに再構築
    let mut i = 0;
    for (data, &number) in counter.iter().enumerate() {
        // 各値について、出現回数分だけリストに追加
        for _ in 0..number {
            list[i] = data as usize;
            i += 1;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_counting_sort_asc() {
        let mut list = [2, 7, 2, 5, 1, 11, 31, 100, 11];
        let result = [1, 2, 2, 5, 7, 11, 11, 31, 100];
        counting_sort(&mut list, 100);
        assert_eq!(result, list);
    }
}
