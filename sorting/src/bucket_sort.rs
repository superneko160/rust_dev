use crate::insertion_sort::insertion_sort;

/// バケットソート（バケツソート）
///
/// # Arguments
///
/// * `list` - 並び替える要素が格納されたリスト
/// 
/// # Returns
///
/// * ソートされた新しいリスト
pub fn bucket_sort(list: &[usize]) -> Vec<usize> {
    if list.is_empty() {
        return vec![];
    }

    let max = *list.iter().max().unwrap();
    let length = list.len();
    let mut buckets = vec![vec![]; length + 1];

    // 要素を適切なバケツに振り分け
    //「適切」の意味:
    // 小さい値は小さいインデックスのバケツに
    // 大きい値は大きいインデックスのバケツに
    // 値の大きさに比例してバケツが選ばれる
    // 上の方法により、値の範囲に応じて要素が分散され、その後の各バケツ内でのソートが効率的に行える
    for x in list {
        buckets[length * *x / max].push(*x);
    }

    // デバッグ: 各バケツの中身を表示
    // println!("Buckets:");
    // for (i, bucket) in buckets.iter().enumerate() {
    //     println!("Bucket {}: {:?}", i, bucket);
    // }

    // 各バケツ内の要素を挿入ソートで並び替え
    // note:挿入ソート以外のアルゴリズムでも問題ない
    for bucket in buckets.iter_mut() {
        insertion_sort(bucket, false);
    }

    // デバッグ: 各バケツの中身を表示
    // println!("Buckets(Sorted):");
    // for (i, bucket) in buckets.iter().enumerate() {
    //     println!("Bucket {}: {:?}", i, bucket);
    // }

    // ソートされたバケツを1つの結果リストにマージ
    let mut result = vec![];
    for bucket in buckets {
        for x in bucket {
            result.push(x);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bucket_sort_asc() {
        let mut list = [2, 7, 2, 5, 1, 11, 31, 100, 11];
        let result = vec![1, 2, 2, 5, 7, 11, 11, 31, 100];
        assert_eq!(result, bucket_sort(&mut list));
    }
}
