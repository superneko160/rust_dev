use std::cmp::PartialEq;

/// 線形探索
/// 
/// # Arguments
///
/// * `item` - 探索対象
/// * `list` - 探索対象が格納されたリスト
/// 
/// # Returns
///
/// * 探索対象の格納されているリストのインデックス
pub fn linear_search<T: PartialEq>(item: &T, list: &[T]) -> Option<usize> {
    for (i, data) in list.iter().enumerate() {
        if item == data {
            return Some(i);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linear_search_string() {
        assert_eq!(Some(0), linear_search(&"dog", &["dog", "cat", "fox", "rabbit"]));
        assert_eq!(Some(2), linear_search(&"fox", &["dog", "cat", "fox", "rabbit"]));
    }

    #[test]
    fn test_linear_search_int() {
        assert_eq!(Some(0), linear_search(&1, &[1, 2, 3, 4]));
        assert_eq!(Some(2), linear_search(&3, &[1, 2, 3, 4]));
    }

    #[test]
    fn test_linear_search_not_found() {
        assert_eq!(None, linear_search(&5, &[1, 2, 3, 4]));
    }

    #[test]
    fn test_linear_search_empty() {
        assert_eq!(None, linear_search(&1, &[]));
    }
}
