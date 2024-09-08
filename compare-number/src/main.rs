use std::cmp::PartialOrd;

/// リストから最大値を取得
/// 
/// ジェネリクスを利用しているのでi32、f64型でも利用できているのがわかる
/// 
/// PartialOrd（パーシャルオード）：2つの値を比較し、その大小関係を判定するためのトレイト
/// if number > largest の部分で比較を行っているが、ジェネリクスを利用しているので使う必要がある
/// このトレイトを実装すると <、>、<=、>= などの比較演算子が利用可能になる
/// 
/// Copy：このトレイトが実装されている型は、変数の代入や関数への引数渡しの際に、（ムーブではなく）値がコピーされる
/// let mut largest: T = list[0]; この行でlist[0]の値をlargestにコピーしているが、Copyトレイトが未実装の場合、この操作はムーブとなり、元の配列から値が移動してしまう
/// largest = number; や 返り値の largest も上と同様
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {

    let mut largest: T = list[0];

    for &number in list {
        if number > largest {
            largest = number;
        }
    }

    largest
}

fn main() {
    let numbers: Vec<i32> = vec![34, 50, 25, 100, 7];
    println!("The largets number is {}", largest(&numbers));

    let numbers: Vec<f64> = vec![100.2, 34.5, 6000.9, 89.1, 413.2];
    println!("The largets number is {}", largest(&numbers));
}
