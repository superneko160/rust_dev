fn main() {
    // Java等ではヒープ領域に確保したメモリは、誰からも参照されなくなったあとにGCによって解放される
    // Rustではただひとつの変数がヒープ上のメモリの所有権を持ち、所有者がスコープから消えた時点でヒープ領域から解放される
    {
        let str = String::from("所有者がスコープから消えた時点でヒープ領域も解放される");
        println!("{}", str);
    }  // nameがスコープアウトしたこの時点で解放される

    // 関数に変数を渡すと、所有権が移動する
    {
        func1();

        fn func1() {
            let str = String::from("関数に変数を渡すと所有者が移動する");
            println!("{}", str);

            // ここで所有権がfunc2()のstrに移動する
            func2(str);
            
            // func2()終了時に解放済みの領域を参照することになるのでエラー
            // println!("{}", str);
        }

        fn func2(str: String) {
            println!("{}", str);
        }
    }

    // func2()から戻り値として所有権を返してもらうこともできる
    {
        func1();

        fn func1() {
            let mut str = String::from("戻り値として所有権を返すことも可能");
            println!("{}", str);

            // 所有権を渡した後、返却してもらう
            str = func2(str);
            println!("{}", str);
        }

        fn func2(str: String) -> String {
            println!("{}", str);
            str  // 所有権を返却
        }
    }

    // &で参照を渡し、所有権を渡さないまま関数を呼び出せる
    // これを借用と呼ぶ
    {
        func1();

        fn func1() {
            let str = String::from("参照のみを渡し、所有権を渡さないこともできる");
            println!("{}", str);

            func2(&str);  // 参照を渡す
            print!("{}", str);  // 所有権が残っているので参照可能
        }

        fn func2(str: &String) {
            println!("{}", str);
        }
    }

    // 関数内でほかの変数に渡しただけでも所有権の移動は発生する
    {
        func1();

        fn func1() {
            let s1 = String::from("関数内でほかの変数に渡しただけでも所有権の移動は発生する");
            {
                let s2 = s1;  // 所有権がs1からs2へ移動
                println!("{}", s2);
            }
            // println!("{}", s1);  // error
        }
    }
}
