use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert("Alice", 92);
    scores.insert("Bob", 80);

    // entry():特定のキーのエントリ（キーと値のペア）を取得
    // キーが存在しない場合に値を挿入する
    scores.entry("Eve").or_insert(72);

    if let Some(score) = scores.get("Alice") {
        println!("Alice's score is {}", score);
    }

    // insert()で上書き可能
    scores.insert("Alice", 90);

    scores.remove("Bob");

    for (name, score) in &scores {
        println!("{}: {}", name, score);
    }
}
