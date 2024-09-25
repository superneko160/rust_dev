/// 選択ソート（破壊的）
///
/// # Arguments
///
/// * `list` - 並び替える要素が格納されたリスト
/// * `reverse` - true:降順, false:昇順
pub fn selection_sort<T: Ord>(list: &mut [T], reverse: bool) {

    let length = list.len();

    for left in 0..length {

        let mut maxmin = left;

        for right in (left + 1)..length {
            if (list[right] < list[maxmin]) ^ reverse {
                maxmin = right;
            }
        }

        list.swap(maxmin, left);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_selection_sort_asc() {
        let mut list = [2, 7, 3, 5, 1, 24, 31, 100, 11];
        let result = [1, 2, 3, 5, 7, 11, 24, 31, 100];
        selection_sort(&mut list, false);
        assert_eq!(result, list);
    }

    #[test]
    fn test_selection_sort_desc() {
        let mut list = [2, 7, 3, 5, 1, 24, 31, 100, 11];
        let result = [100, 31, 24, 11, 7, 5, 3, 2, 1];
        selection_sort(&mut list, true);
        assert_eq!(result, list);
    }
}
