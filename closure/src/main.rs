fn main() {
    {
        // クロージャは他言語でいうところの無名関数やラムダ式に近い
        let square = |x: i32| {
            x * x
        };
        println!("{}", square(3));  // 9
    }

    {
        // moveはクロージャ下で参照するクロージャ外変数が存在する場合
        // その所有権をクロージャに移す
        let msg = String::from("Hello");
        let func = move || {
            println!("{}", msg);
        };
        func();  // Hello
        // println!("{}", msg);  // 所有権を移動したあとなのでエラー

        // PHPでいうと、useを使って関数外の変数を利用するのに近い
        // ただし、PHPは変数のコピーを作成するのに対し
        // Rustは所有権を移動するので、もとのスコープで使えなくなる
        // $msg = "Hello";
        // $func = function() use ($msg) {
        //     echo $msg;
        // };
        // $func();
    }

    // 高階関数の例
    {
        // クロージャを定義
        let add_one = |x| x + 1;
        let multiply_by_2 = |x| x * 2;

        // クロージャを高階関数に渡す
        let result1 = apply_function(add_one, 5);  // 6
        let result2 = apply_function(multiply_by_2, 5);  // 10
        println!("{} {}", result1, result2);
        
        fn apply_function<F>(f: F, value: i32) -> i32 where F: Fn(i32) -> i32 {
            f(value)
        }
    }
}
