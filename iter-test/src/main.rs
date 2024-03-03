/**
 * イテレータは連続したオブジェクトを順番に扱うための機能を提供するオブジェクト
 * イテレータはIteratorトレイトのインスタンス
 * アダプタ：イテレータからイテレータを作成するメソッド
 */

fn main() {
    iter_zip(&[1, 2, 3], &[4, 5, 6]);
    iter_map(&[1, 2, 3]);
    iter_filter(&[0i32, 1, 2]);
    iter_fold(&[1, 2, 3]);
    iter_collect(&[1, 2, 3]);
    iter_enumerate(&['a', 'b', 'c']);
}

/**
 * zipメソッド
 * イテレータ同士の合成
 * 要素はタプルになる
 */
fn iter_zip(list1: &[i32], list2: &[i32]) {
    let iter = list1.iter().zip(list2.iter());
    iter.for_each(|(x, y)| println!("{} {}", x, y));
}

/**
 * mapメソッド
 * 各要素に関数を適用
 */
fn iter_map(list: &[i32]) {
    let iter = list.iter().map(|x| 2 * x);
    iter.for_each(|x| println!("{}", x));
}

/**
 * filterメソッド
 * 各要素に関数を適用し、trueを返した要素だけを抽出
 */
fn iter_filter(list: &[i32]) {
    let iter = list.iter().filter(|x| x.is_positive());
    iter.for_each(|x| println!("{}", x));
}

/**
 * foldメソッド
 * 各要素に関数を適用し、状態を更新し、その状態を返す
 */
fn iter_fold(list: &[i32]) {
    let sum = list.iter().fold(0, |acc, x| acc + x);
    println!("{}", sum);
}

/**
 * collectメソッド
* イテレータの全要素をコレクションに変換
 */
fn iter_collect(list: &[i32]) {
    let doubled: Vec<i32> = list.iter().map(|&x| x * 2).collect();
    doubled.iter().for_each(|x| println!("{}", x));
}

/**
 * enumerateメソッド
 * インデックスと各要素のペアをタプルに
 */
fn iter_enumerate<T: std::fmt::Display>(list: &[T]) {
    list.iter().enumerate().for_each(|(x, y)|println!("{} {}", x, y));
}