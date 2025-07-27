/**
 * ライフタイム省略規則
 * Rustコンパイラは、コードを読みやすくするために、特定のパターンに合致する関数のライフタイムアノテーションを自動的に推論する
 * これをライフタイム省略規則と呼ぶ
 *
 * 1. 各入力の参照パラメータは、それぞれ自身のライフパラメータを持つ
 * 例: fn foo(x: &str) は fn foo<'a>(x: &'a str) と推論される
 * 例: fn foo(x: &str, y: &str) は fn foo<'a, 'b>(x: &'a str, y: &'b str) と推論される
 *
 * 2. 入力パラメータが1つしかない場合、その入力パラメータのライフタイムがすべての出力ライフタイムに付与される
 * 例: fn foo(x: &str) -> &str は fn foo<'a>(x: &'a str) -> &'a str と推論される
 * 以下のケースはこのルールに該当するので、省略してもOKになっている
 *
 * 3. 複数の入力ライフタイムパラメータがある場合でも、その1つが&selfまたは&mut
 *    selfであるならば、selfのファイルタイムうがすべての出力ライフタイムに付与される
 * 例: impl<'a> MyStruct<'a> { fn foo(&self) -> &str } は fn foo(&'a self) -> &'a str と推論される
 * このルールはメソッドの場合に適用される
 *
 */
// fn first_word<'a>(s: &'a str) -> &'a str {
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn main() {
    let sentence = String::from("Hello world");
    let word = first_word(&sentence);
    println!("最初の単語: {}", word);
}
