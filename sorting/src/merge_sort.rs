/// マージソート（破壊的）
/// @param &mut [T] list 並び替える要素が格納されたリスト（配列、ベクタ等）
pub fn merge_sort<T: Ord + Copy>(list: &mut [T]) {
    if list.len() > 1 {
        // 中央のインデックス取得
        let mid = list.len() / 2;
        // 左半分を再帰的にソート
        merge_sort(&mut list[..mid]);
        // 右半分を再帰的にソート
        merge_sort(&mut list[mid..]);
        // 半分に切ったリストをマージ
        merge(list, mid);
    }
}

/// マージ
/// @param &mut [T] list 並び替える要素が格納されたリスト（配列、ベクタ等）
/// @param usize mid  リスト中央のインデックス
fn merge<T: Ord + Copy>(list: &mut [T], mid: usize) {

    let left = list[..mid].to_vec();
    let right = list[mid..].to_vec();

    // マージ中にポジションを追跡するためのインデックス
    let mut left_index = 0;
    let mut right_index = 0;

    for v in list {
        if right_index == right.len() || (left_index < left.len() && left[left_index] < right[right_index]) {
            *v = left[left_index];
            left_index += 1;
        } else {
            *v = right[right_index];
            right_index += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_sort_asc() {
        let mut list = [2, 7, 3, 5, 1, 24, 31, 100, 7];
        let result = [1, 2, 3, 5, 7, 7, 24, 31, 100];
        merge_sort(&mut list);
        assert_eq!(result, list);
    }
}
