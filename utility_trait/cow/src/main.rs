use std::borrow::Cow;

// Cowの基本
fn demonstrate_basic_cow() {
    // Cowは借用(Brrowed) or 所有(Owned)のどちらかを保持
    let borrowed_cow: Cow<str> = Cow::Borrowed("Hello, World");
    let owned_cow: Cow<str> = Cow::Owned(String::from("Hello, Rust"));

    println!("借用版: {}", borrowed_cow);
    println!("所有版: {}", owned_cow);

    // 借用しているか確認
    println!("borrowed_cow is borrowed: {}", matches!(borrowed_cow, Cow::Borrowed(_)));
    println!("owned_cow is borrowed: {}", matches!(owned_cow, Cow::Borrowed(_)));
    println!("-------");
}

// -- Cowが解決する問題 --
// 問題のあるAPIの例
fn process_string_bad(input: &str) -> String {
    if input.contains("dog") {
        input.replace("dog", "cat") // 新しいString作成
    } else {
        input.to_string() // ここで不要なクローン発生している
    }
}

// Cowを利用した改良版APIの例
fn process_string_good(input: &str) -> Cow<str> {
    if input.contains("dog") {
        Cow::Owned(input.replace("dog", "cat")) // 必要なときだけクローン
    } else {
        Cow::Borrowed(input) // 借用のまま返す
    }
}

fn demonstrate_cow_efficiency() {
    let dog_input = "I like dog.";
    let cat_input = "I like cat.";

    // 従来の方法（常にクローン）
    let result1 = process_string_bad(dog_input);
    let result2 = process_string_bad(cat_input);

    println!("従来版 - dog: {}", result1);
    println!("従来版 - cat: {}", result2);

    // Cow版（必要なときだけクローン）
    let cow_result1 = process_string_good(dog_input);
    let cow_result2 = process_string_good(cat_input);

    println!("Cow版 - dog: {}(borrowed: {})",
        cow_result1, matches!(cow_result1, Cow::Borrowed(_)));
    println!("Cow版 - cat: {}(borrowed: {})",
        cow_result2, matches!(cow_result2, Cow::Borrowed(_)));
}

fn main() {
    demonstrate_basic_cow();
    demonstrate_cow_efficiency();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cow_efficiency() {
        let input = "no changes needed";
        let result = process_string_good(input);

        // 変更不要な場合は借用のままであることをテスト
        assert!(matches!(result, Cow::Borrowed(_)));
        assert_eq!(result, input);
    }

    #[test]
    fn test_cow_modification() {
        let input = "This is dog!";
        let result = process_string_good(input);

        // 変更が必要な場合は所有版になることをテスト
        assert!(matches!(result, Cow::Owned(_)));
        assert_eq!(result, "This is cat!");
    }
}
