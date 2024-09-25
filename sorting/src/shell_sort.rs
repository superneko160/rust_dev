/// シェルソート（破壊的）
/// 挿入ソートの改良版
///
/// シェルソートにおける「ギャップ」：比較する要素間の距離
/// 初期のギャップは配列の長さの半分から開始（list.len() / 2）
/// このギャップを使って、離れた要素同士を比較し並べ替える
/// ギャップが4の場合、インデックス0と4、1と5、2と6などの要素を比較する
/// ギャップを徐々に小さくしていき、最終的にギャップが1になったとき、通常の挿入ソートと同じ動作になる
///
/// # Arguments
///
/// * `list` - 並び替える要素が格納されたリスト
pub fn shell_sort<T: Ord + Copy>(list: &mut [T]) {
    // リストの半分の長さを取得
    let mut sublist_length = list.len() / 2;

    while sublist_length > 0 {
        for pos_start in 0..sublist_length {
            // 挿入ソートを実行
            insertion(list, pos_start, sublist_length);
        }

        // サブリストの長さを半分に
        sublist_length /= 2;
    }
}

/// 指定されたギャップを使用した挿入ソート
///
/// # Arguments
///
/// * `list` - 並び替える要素が格納されたリスト
/// * `start` - サブリストの開始位置
/// * `gap` - 要素間のギャップ
fn insertion<T: Ord + Copy>(list: &mut [T], start: usize, gap: usize) {
     // ギャップごとに要素をループ
     // step_by() 指定した間隔で取得した要素が格納されたイテレータを返す
    for i in ((start + gap)..list.len()).step_by(gap) {

        let current = list[i];  // 現在の要素
        let mut pos = i;        // 現在の位置

        // 適切な挿入位置を見つけるまでループ
        while pos >= gap && list[pos - gap] > current {
            list[pos] = list[pos - gap];  // 大きい要素を右にシフト
            pos -= gap;  // 位置をギャップ分左に移動
        }

        // 正しい位置に現在の要素を挿入
        list[pos] = current;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shell_sort_asc() {
        let mut list = [2, 7, 3, 5, 1, 24, 31, 100, 11];
        let result = [1, 2, 3, 5, 7, 11, 24, 31, 100];
        shell_sort(&mut list);
        assert_eq!(result, list);
    }
}
