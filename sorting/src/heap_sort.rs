use std::collections::BinaryHeap;

/// ヒープソート
/// 
/// 全ての要素のクローンを作成しているためメモリ使用量が大きい
///メモリ効率とパフォーマンスを重視する場合は
/// BinaryHeapを使用せずに配列を直接操作するカスタムのヒープソート実装する
/// 
/// # Arguments
///
/// * `list` - ソートする配列
/// * `reverse` - true:降順, false:昇順
///
/// # Returns
///
/// * ソートされた新しいリスト
pub fn heap_sort<T: Ord>(list: &[T], reverse: bool) -> Vec<T>
where
    T: Clone,
{
    // BinaryHeapを作成し、すべての要素を追加（構築段階ですでにソートされている）
    let mut heap: BinaryHeap<_> = list.iter().cloned().collect();

    // もとのリストの同じ要素数のリストを用意
    let mut result = Vec::with_capacity(list.len());

    // ヒープから要素を順に取り出し、新しいリストに追加
    while let Some(item) = heap.pop() {
        result.push(item);
    }

    // BinaryHeapはmax-heapなので、昇順の場合は結果を反転させる
    if !reverse {
        result.reverse();
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_heap_sort_asc() {
        let mut list = [2, 7, 2, 5, 1, 11, 31, 100, 11];
        let result = vec![1, 2, 2, 5, 7, 11, 11, 31, 100];
        assert_eq!(result, heap_sort(&mut list, false));
    }

    #[test]
    fn test_heap_sort_desc() {
        let mut list = [2, 7, 2, 5, 1, 11, 31, 100, 11];
        let result = vec![100, 31, 11, 11, 7, 5, 2, 2, 1];
        assert_eq!(result, heap_sort(&mut list, true));
    }
}
