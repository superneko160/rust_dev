fn main() {

    // let objective: Option<i32> = Some(1);
    let objective: Option<i32> = None;
    match objective {
        Some(x) if x % 2 == 0 => println!("{} is even", x),
        Some(x) => println!("{} is odd", x),
        None => println!("none!")
    }

    let result: Result<i32, String> = Ok(200);

    // ver match
    // match result {
    //     Ok(code) => println!("code: {}", code),
    //     Err(err) => println!("Err: {}", err),
    // }

    // ver if let
    // if let Ok(code) = result {
    //     println!("code: {}", code);
    // }

    // matchやif letを使ってネストが深くなる場合はunwrap_or()を利用
    // Okだった場合そのまま展開、Errだった場合引数に与えた値を返す
    // ver upwrap_or()
    // println!("code: {}", result.unwrap_or(-1));
    // let result: Result<i32, String> = Err("error".to_string());
    // println!("code: {}", result.unwrap_or(-1));

    // Okだった場合のみ、指定した関数を実行したい場合
    // ver and_then()
    let next_result = result.and_then(func);  // 実行される
    let result: Result<i32, String> = Err("error".to_string());
    let next_result = result.and_then(func);  // 実行されない
}

fn func(code: i32) -> Result<i32, String> {
    println!("code: {}", code);
    Ok(100)
}
