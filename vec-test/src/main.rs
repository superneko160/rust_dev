fn main() {
    {
        let vec = vec![1, 2, 3, 4, 5];
        println!("vec[index] {}", vec[2]);  // インデックスでアクセス（パニックの可能性あり）
        println!("vec.get(index) {:?}", vec.get(1));  // 安全なインデックスアクセス（Option）
        println!("vec.len() {}", vec.len());  // サイズを取得
        println!("vec.capacity() {}", vec.capacity());  // 容量を取得
        println!("vec.is_empty() {}", vec.is_empty());  // 空かどうかを確認
        println!("vec.contains(index) {}", vec.contains(&4));  // 要素の存在確認
        println!("vec slice: {:?}", &vec[1..4]);  // 範囲でスライス
        println!("vec.split_at(mid) {:?}", vec.split_at(2)); // index0~mid値までのリストとそれ以外のリストに分割

        //  要素の位置を検索
        let pos = vec.iter().position(|&value| value == 3);
        println!("{:?}", pos);
    }

    println!("================");

    {
        // 指定要素数ぶん特定の値で初期化
        let mut vec = vec![0; 5];
        for val in &vec {
            print!("{} ", val);
        }
        println!();

        vec.push(1);  // 末尾に追加
        vec.insert(1, 2);  // 指定位置に挿入
        vec.extend([3, 4, 5]);  // 複数要素を末尾に追加
        for val in &vec {
            print!("{} ", val);
        }
        println!();

        vec.swap(2, 7);  // 指定したインデックスの値の交換
        for val in &vec {
            print!("{} ", val);
        }
        println!();

        vec.sort();
        for val in &vec {
            print!("{} ", val);
        }
        println!();
    }
}
