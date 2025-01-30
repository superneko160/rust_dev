use std::collections::LinkedList;

fn main() {
    // 自作とは別に、標準ライブラリ（std）の連結リストを使ってみる
    let mut list: LinkedList<i8> = LinkedList::new();

    list.push_back(1);  // 末尾に要素を追加
    list.push_front(0); // 先頭に要素を追加
    list.push_back(2);  // 末尾に要素を追加
    println!("{:?}", list);

    let first = list.pop_front();  // 先頭から要素を削除
    let last = list.pop_back();  // 末尾から要素を削除

    println!("{:?}", first);
    println!("{:?}", last);
    
    for element in &list {
        println!("{}", element);
    }

    let mut names: LinkedList<String> = LinkedList::new();

    // 複数要素を一度に追加
    names.extend([
        String::from("Alice"),
        String::from("Bob"),
        String::from("Clara"),
    ]);

    println!("サイズ：{}", names.len());
    for name in &names {
        println!("{}", name);
    }

    /*
     # Vecとの比較
      - Vecと異なり、インデックスによる直接アクセス不可
      - メモリ使用量がVecより多くなる
      - 要素の挿入/削除が頻繁な場合はLinkedListが有利
      - 連続したメモリアクセスが重要な場合はVecが有利
     */
}
