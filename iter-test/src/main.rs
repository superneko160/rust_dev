/**
 * イテレータは連続したオブジェクトを順番に扱うための機能を提供するオブジェクト
 * イテレータはIteratorトレイトのインスタンス
 * アダプタ：イテレータからイテレータを作成するメソッド
 */

fn main() {
    iter_zip(&[1,2,3], &[4,5,6]).for_each(|(x,y)|println!("{} {}", x, y));
    iter_map(&[1,2,3]).for_each(|x| println!("{}", x));
    iter_filter(&[0i32, 1, 2]).for_each(|x| println!("{}", x));
    println!("{}", iter_fold(&[1, 2, 3]));
    iter_collect(&[1,2,3]).iter().for_each(|x| println!("{}", x));
    iter_enumerate(&['a', 'b', 'c']).for_each(|(x, y)| println!("{} {:?}", x, y));
}

/// イテレータ同士の合成
/// # Arguments
/// * `list1` - 入力スライス
/// * `list2` - 入力スライス
/// # Returns
/// イテレータ（数値、数値）
fn iter_zip<'a>(list1: &'a [i32], list2: &'a [i32]) -> impl Iterator<Item = (&'a i32, &'a i32)> {
    list1.iter().zip(list2.iter())
}

/// 各要素に関数を適用したイテレータを作成
/// # Arguments
/// * `list` - 入力スライス
/// # Returns
/// イテレータ 数値
fn iter_map<'a>(list: &'a [i32]) -> impl Iterator<Item = i32> + 'a {
    list.iter().map(|x| 2 * x)
}

/// 各要素に関数を適用し、trueを返した要素だけを抽出したイテレータを作成
/// # Arguments
/// * `list` - 入力スライス
/// # Returns
/// イテレータ 数値
fn iter_filter<'a>(list: &'a [i32]) -> impl Iterator<Item = &i32> {
    list.iter().filter(|x| x.is_positive())
}

/// 各要素に関数を適用し、状態を更新し、その状態を返す
/// # Arguments
/// * `list` - 入力スライス
/// # Returns
/// 数値
fn iter_fold(list: &[i32]) -> i32 {
    list.iter().fold(0, |acc, x| acc + x)
}

/// イテレータの全要素をコレクションに変換
/// # Arguments
/// * `list` - 入力スライス
/// # Returns
/// コレクション
fn iter_collect(list: &[i32]) -> Vec<i32> {
    list.iter().map(|&x| x * 2).collect()
}

/// インデックスと要素の組み合わせを返すイテレータを生成
/// # Arguments
/// * `list` - 入力スライス
/// # Returns
/// イテレータ (usize, &T) 左側の値がインデックス、右側の値が要素
fn iter_enumerate<T>(list: &[T]) -> impl Iterator<Item = (usize, &T)> {
    list.iter().enumerate()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iter_zip() {
        let list1 = vec![1, 2, 3];
        let list2 = vec![4, 5, 6];
        let result: Vec<_> = iter_zip(&list1, &list2).collect();
        assert_eq!(result, vec![(&1, &4), (&2, &5), (&3, &6)]);
    }

    #[test]
    fn test_iter_map() {
        let data = [1, 2, 3, 4, 5];
        let result: Vec<_> = iter_map(&data).collect();
        assert_eq!(result, vec![2, 4, 6, 8, 10]);
    }

    #[test]
    fn test_iter_filter() {
        let data = vec![-1, 2, -3, 4, -5];
        // 正の整数のみを抽出
        let result: Vec<i32> = iter_filter(&data).copied().collect();
        assert_eq!(result, vec![2, 4]);
    }

    #[test]
    fn test_iter_fold() {
        let data = vec![1, 2, -3];
        let result: i32 = iter_fold(&data);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_iter_collect() {
        let data = [1, 2, 3, 4, 5];
        assert_eq!(iter_collect(&data), vec![2, 4, 6, 8, 10]);
    }
    
    #[test]
    fn test_iter_enumerate() {
        let list_int = vec![1, 2, 3, 4, 5];
        let list_str = vec!["apple", "banana", "cherry"];
        // 整数のスライスをテスト
        let result_int: Vec<(usize, &i32)> = iter_enumerate(&list_int).collect();
        assert_eq!(result_int, vec![(0, &1), (1, &2), (2, &3), (3, &4), (4, &5)]);
        // 文字列のスライスをテスト
        let result_str: Vec<(usize, &&str)> = iter_enumerate(&list_str).collect();
        assert_eq!(result_str, vec![(0, &"apple"), (1, &"banana"), (2, &"cherry")]);
    }

}