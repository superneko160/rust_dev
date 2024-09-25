/// 挿入ソート（破壊的）
///
/// # Arguments
///
/// * `list` - 並び替える要素が格納されたリスト
/// * `reverse` - true:降順, false:昇順
pub fn insertion_sort<T: Ord + Copy>(list: &mut [T], reverse: bool) {
    for i in 1..list.len() {

        let mut target_index = i;  // 挿入する箇所
        let current = list[i];  // 挿入する要素

        while target_index > 0 && (current < list[target_index - 1]) ^ reverse {
            list[target_index] = list[target_index - 1];
            target_index -= 1;
        }

        list[target_index] = current;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insertion_sort_asc() {
        let mut list = [2, 7, 3, 5, 1, 24, 31, 100, 11];
        let result = [1, 2, 3, 5, 7, 11, 24, 31, 100];
        insertion_sort(&mut list, false);
        assert_eq!(result, list);
    }

    #[test]
    fn test_insertion_sort_desc() {
        let mut list = [2, 7, 3, 5, 1, 24, 31, 100, 11];
        let result = [100, 31, 24, 11, 7, 5, 3, 2, 1];
        insertion_sort(&mut list, true);
        assert_eq!(result, list);
    }
}
