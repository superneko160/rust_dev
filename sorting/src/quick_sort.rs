/// クイックソート（破壊的）
///
/// # Arguments
///
/// * `list` - 並び替える要素が格納されたリスト
pub fn quick_sort<T: Ord>(list: &mut [T]) {
    let len = list.len();
    if len > 1 {
        qsort(list, 0, len - 1);
    }
}

/// クイックソートの再帰的実装
///
/// # Arguments
///
/// * `list` - 並び替える要素が格納されたリスト
/// * `low` - ソート範囲の開始インデックス
/// * `high` - ソート範囲の終了インデックス
fn qsort<T: Ord>(list: &mut [T], mut low: usize, mut high: usize) {
    while low < high {
        // リストをパーティション分割し、ピボットのインデックスを取得
        let pivot = partition(list, low, high);

        // 左側のサブリストが右側より小さい場合
        if pivot - low < high - pivot {
            if pivot > 0 {
                qsort(list, low, pivot - 1);
            }
            low = pivot + 1;  // 次のイテレーションで右側のサブリストをソート
        }
        // 左側のサブリストが右側より大きい場合
        else {
            qsort(list, pivot + 1, high);
            high = pivot - 1;  // 次のイテレーションで左側のサブリストをソート
        }
    }
}

/// リストをパーティション分割する
///
/// # Arguments
///
/// * `list` - パーティション分割する要素が格納されたリスト
/// * `low` - パーティション範囲の開始インデックス
/// * `high` - パーティション範囲の終了インデックス
///
/// # Returns
///
/// * ピボット要素の最終的な位置
fn partition<T: PartialOrd>(list: &mut [T], low: usize, high: usize) -> usize {
    let pivot = high;  // 最後の要素をピボットとして選択
    let mut i = low;  // 左側のポインタ
    let mut j = high - 1;  // 右側のポインタ（ピボットの1つ前）

    loop {
        // ピボットより大きい左側の要素を探す
        while list[i] < list[pivot] {
            i += 1;
        }
        // ピボットより小さい右側の要素を探す
        while j > 0 && list[j] > list[pivot] {
            j -= 1;
        }
        // ポインタが交差したらループを抜ける
        if j == 0 || i >= j {
            break;
        }
        // 要素が等しい場合、両方のポインタを動かす
        else if list[i] == list[j] {
            i += 1;
            j -= 1;
        } else {
            list.swap(i, j);
        }
    }
    // ピボットを適切な位置に移動
    list.swap(i, pivot);
    i
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quick_sort_asc() {
        let mut list = [2, 7, 3, 5, 1, 24, 31, 100, 7];
        let result = [1, 2, 3, 5, 7, 7, 24, 31, 100];
        quick_sort(&mut list);
        assert_eq!(result, list);
    }
}
