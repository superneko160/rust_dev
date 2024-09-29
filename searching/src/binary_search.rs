use std::cmp::Ordering;

/// 二分探索
/// 
/// # Arguments
///
/// * `item` - 探索対象
/// * `list` - 探索対象が格納されたリスト（ソート済み）
/// 
/// # Returns
///
/// * 探索対象の格納されているリストのインデックス
pub fn binary_search<T: Ord>(item: &T, list: &[T]) -> Option<usize> {

    let mut low = 0;
    let mut high = list.len();

    while low < high {
        // オーバーフローを防ぐため、(low + high) / 2ではなくlow + (high - low) / 2を使用
        let mid = low + (high - low) / 2;

        // 検索対象と中央要素を比較
        match item.cmp(&list[mid]) {
            Ordering::Equal => return Some(mid),  // 対象発見
            Ordering::Less => high = mid,  // 中央より左に対象がある場合
            Ordering::Greater => low = mid + 1,  // 中央より右に対象がある場合
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_search_int_list_length_even() {
        assert_eq!(Some(0), binary_search(&1, &[1, 2, 3, 4]));
        assert_eq!(Some(2), binary_search(&3, &[1, 2, 3, 4]));
    }

    #[test]
    fn test_binary_search_int_list_length_odd() {
        assert_eq!(Some(2), binary_search(&11, &[5, 6, 11, 13, 15]));
        assert_eq!(Some(3), binary_search(&13, &[5, 6, 11, 13, 15]));
    }

    #[test]
    fn test_binary_search_not_found() {
        assert_eq!(None, binary_search(&5, &[1, 2, 3, 4]));
    }

    #[test]
    fn test_binary_search_empty() {
        assert_eq!(None, binary_search(&1, &[]));
    }
}
