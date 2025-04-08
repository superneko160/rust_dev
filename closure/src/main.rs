fn main() {
    {
        // クロージャは多言語でいうところの無名関数やラムダ式に近い
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
}
