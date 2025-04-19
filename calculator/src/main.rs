use std::env;

// cargo run <operator> <number1> <number2>
// ex) cargo run add 1 2
// result) 3
fn main() {
    // コマンドライン引数取得
    let args: Vec<String> = env::args().collect();

    // as_str() &String -> &str に変換
    // parse() String -> i32 に変換
    let operator: &str = args[1].as_str();
    let num1: i32 = args[2].parse().expect("First argument must be a number");
    let num2: i32 = args[3].parse().expect("Second argument must be a number");

    let result = match operator {
        "add" => add(num1, num2),
        "sub" => sub(num1, num2),
        "mult" => mult(num1, num2),
        "div" => div(num1, num2),
        _ => panic!("Invalid operator"),
    };

    println!("{}", result);
}

fn add(num1: i32, num2: i32) -> i32 {
    return num1 + num2;
}

fn sub(num1: i32, num2: i32) -> i32 {
    return num1 - num2;
}

fn mult(num1: i32, num2: i32) -> i32 {
    return num1 * num2;
}

fn div(num1: i32, num2: i32) -> i32 {
    return num1 / num2;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3);
        assert_eq!(add(-1, 2), 1);
    }

    #[test]
    fn test_sub() {
        assert_eq!(sub(3, 2), 1);
        assert_eq!(sub(1, 2), -1);
    }

    #[test]
    fn test_mult() {
        assert_eq!(mult(3, 2), 6);
        assert_eq!(mult(-3, 2), -6);
    }

    #[test]
    fn test_div() {
        assert_eq!(div(4, 2), 2);
        assert_eq!(div(-4, 2), -2);
    }
}
