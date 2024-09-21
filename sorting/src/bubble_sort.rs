/// バブルソート（破壊的）
/// @param &mut [T] list 並び替える要素が格納されたリスト（配列、ベクタ等）
/// @param bool reverse true:降順, false:昇順
pub fn bubble_sort<T: Ord>(list: &mut [T], reverse: bool) {

    if list.is_empty() {
        return;
    }

    let mut sorted = false;
    let mut n = list.len();

    while !sorted {
        sorted = true;
        for i in 0..n - 1 {
            // ^ XOR（排他的論理和）
            // どちらかがtrueの場合、値を交換
            if (list[i] > list[i + 1]) ^ reverse {
                list.swap(i, i + 1);
                sorted = false;
            }
        }
        n -= 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bubble_sort_asc() {
        let mut list = [2, 7, 3, 5, 1, 24, 31, 100, 11];
        let result = [1, 2, 3, 5, 7, 11, 24, 31, 100];
        bubble_sort(&mut list, false);
        assert_eq!(result, list);
    }

    #[test]
    fn test_bubble_sort_desc() {
        let mut list = [2, 7, 3, 5, 1, 24, 31, 100, 11];
        let result = [100, 31, 24, 11, 7, 5, 3, 2, 1];
        bubble_sort(&mut list, true);
        assert_eq!(result, list);
    }
}
