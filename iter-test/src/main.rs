/**
 * https://doc.rust-lang.org/std/iter/trait.Iterator.html
 * イテレータ：連続したオブジェクトを順番に扱うための機能を提供するオブジェクト
 * （Iteratorトレイトのインスタンス）
 * アダプタ：イテレータからイテレータを作成するメソッド
 * イテレータのチェーン：複数のアダプタを連続して適用し、データ処理のフローを表現力豊かに記述できる書き方
 * これにより、複雑なデータ操作を、中間のコレクションを生成せずに、シンプルに記述可能
 * （中間コレクションが生成されない -> メモリ使用量を削減できる）
 */
fn main() {
    // 基本
    {
        let vec = vec![1, 2, 3, 4, 5];

        // vecのイテレータ取得
        let mut iter = vec.iter();

        // next()で要素を1つずつ取得
        while let Some(value) = iter.next() {
            print!("{} ", value);
        }
        println!();
    }

    println!("===========");

    // イテレータのアダプタ
    {
        // map、zip、filter、find、take、chain、enumerate、fold、collectなどが利用頻度高め

        // find() 一致した要素をSomeで返す
        let vec1 = vec![1, 2, 3, 4, 5];
        let mut iter1 = vec1.iter();
        println!("iter.find {:?}", iter1.find(|&&x| x == 4));

        // take() 与えられた個数(usize)ぶんの要素を返し、それ以降の要素を無視するような、新たなイテレータを返す
        let vec2 = vec![1, 2, 3, 4, 5];
        let iter2 = vec2.iter();
        let mut result = iter2.take(2);
        while let Some(value) = result.next() {
            print!("{} ", value);
        }
        println!();
    }

    println!("===========");

    // アダプタを組み合わせたイテレータチェーン
    {
        // ベクタから偶数の2乗を取得
        let vec = vec![1, 2, 3, 4, 5, 6];

        let squares_of_even: Vec<i32> = vec.iter()  // イテレータ取得
            .filter(|&&x| x % 2 == 0)  // 偶数をフィルタリング
            .map(|x| x * x)  // 各要素を2乗
            .collect();  // 結果をベクタに集約

        println!("{:?}", squares_of_even);

        // 2つイテレータの各要素を+1したイテレータを作成
        let vec1 = vec![1, 2, 3];
        let vec2 = vec![11, 22, 33];

        let chained_vec: Vec<i32> = vec1.iter()  // イテレータ取得
            .chain(vec2.iter())  // イテレータの連結
            .map(|x| x + 1)  // 各要素を+1
            .collect();  //   // 結果をベクタに集約

        println!("{:?}", chained_vec);
    }

    println!("===========");

    // イテレータのチェーンと所有権
    // イテレータがベクタを借用しているあいだは、変更が許可されない
    // イテレータの使用が終了すると、ベクタの変更が可能となる
    {
        let mut vec: Vec<u8> = vec![1, 2, 3, 4, 5];
        let numbers = vec.iter()
            .filter(|&&n| n % 2 == 0)
            .map(|n| n  * n);

        // cannot borrow `vec` as mutable because it is also borrowed as immutable
        // mutable borrow occurs hererustcClick for full compiler diagnostic
        // vec.push(6);  // vec は借用中なので変更できないとエラーが出る

        for number in numbers {
            print!("{} ", number);
        }
        println!();

        // 借用が終わったら vec は変更可能
        vec.push(6);
    }

    println!("===========");

    // デバッグ
    // inspect() チェーン内の各ステップでデータの状態を確認できる
    {
        let vec: Vec<u8> = vec![1, 2, 3, 4, 5];
        let result: Vec<u8> = vec.iter()
            .inspect(|n| println!("Original: {}", n))
            .filter(|&&n| n % 2 == 0)
            .inspect(|n| println!("Filtered: {}", n))
            .map(|n| n * n)
            .inspect(|n| println!("Mapped: {}", n))
            .collect();

        println!("{:?}", result);
    }
}
