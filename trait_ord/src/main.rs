/**
 * Rustの標準ライブラリには、数値型(i32, f64)や文字型(char)など
 * 多くの基本的な型が用意されており、それらはすでにpartialordトレイトを実装している
 * 以下のプログラムは、ジェネリック型とトレイト境界を使用して
 * これらの組み込み型に対して共通の比較処理を行う例を示している
 */

// a, b のうち大きいほうを返す
fn find_max<T: PartialOrd>(a: T, b: T) -> T {
    if a > b {
        return a;
    }
    b
}

fn main() {
    println!("Max of 10 and 20: {}", find_max(10, 20));  // i32
    println!("Max of 3.14 and 2.71: {}", find_max(3.14, 2.71));  // f64
    println!("Max of 'a' and 'z': {}", find_max('a', 'z'));  // char
}
