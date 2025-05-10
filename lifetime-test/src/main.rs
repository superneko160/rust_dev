/**
 * 複数の参照が絡み合う場合、コンパイラはそれらの参照が常に有効であると
 * 保証するためにライフタイムという仕組みを利用する
 * ライフタイムは、参照が指すデータの有効期間を表す
 */
fn main() {
    let string1 = String::from("abcdef");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }
}

/// 2つの文字スライス($str)を受け取り、長い方を返す
/// 
/// このとき返される参照は、入力されたどちらかの文字列スライスを参照するため
/// コンパイラは、返される参照のライフタイムが入力された参照のライフタイムよりも
/// 長くならないことを保証する必要がある
/// 
/// <'a>はライフタイム注釈と呼ばれ、「入力xとy、そして返り値は同じライフタイムaを持つという意味になる
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
