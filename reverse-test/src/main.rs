use std::cmp::Reverse;

// 成績を表す構造体
struct Grade {
    name: String,
    score: i32,
    class: String,
}

fn main() {
    let mut numbers = vec![1, 5, 2, 8, 3];

    // 昇順ソート
    numbers.sort();
    println!("昇順ソート:{:?}", numbers);

    // Reverseを利用した降順ソート
    numbers.sort_by_key(|&x| Reverse(x));
    println!("降順ソート:{:?}", numbers);

    // より複雑なデータ構造で降順ソート
    let mut pairs = vec![(1, "one"), (2, "two"), (3, "three")];
    pairs.sort_by_key(|&(num, _)| Reverse(num));
    println!("ペアの降順ソート:{:?}", pairs);

    // 成績の定義
    let mut grades = vec![
        Grade { name: String::from("Alice"), score:85, class: String::from("A") },
        Grade { name: String::from("Bob"), score:92, class: String::from("B") },
        Grade { name: String::from("Charlie"), score:78, class: String::from("A") },
    ];

    // スコアで降順、クラスで昇順にソート
    grades.sort_by_key(|g| (Reverse(g.score), g.class.clone()));

    // 結果の表示
    for grade in grades {
        println!("{}: {} (Class {})", grade.name, grade.score, grade.class);
    }
}
