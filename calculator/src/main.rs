use std::env;

// cargo run <operator> <number1> <number2> ...
// ex) cargo run add 1 2
// result) 3
fn main() -> Result<(), String> {
    // コマンドライン引数を取得
    let args: Vec<String> = env::args().collect();
    // コマンドライン引数の検証
    if args.len() < 3 {
        return Err("usage: cargo run <operator> <number1> <number2> ...".to_string());
    }

    let operator = parse_operator(&args[1])?;
    let numbers = parse_numbers(&args[2..])?;

    let result = calculate(operator, numbers)?;
    
    println!("結果: {}", result);
    Ok(())
}

// 演算子の解析
fn parse_operator(operator: &str) -> Result<&str, String> {
    match operator {
        "add" | "sub" | "mult" | "div" => Ok(operator),
        _ => Err(format!("無効な演算子です: {}", operator))
    }
}

// 数値の解析
fn parse_numbers(args: &[String]) -> Result<Vec<i32>, String> {
    args.iter()
        .map(|arg| arg.parse::<i32>()
            .map_err(|_| format!("無効な数値です: {}", arg)))
        .collect()
}

// 計算実行
fn calculate(operator: &str, numbers: Vec<i32>) -> Result<i32, String> {
    if numbers.is_empty() {
        return Err("数値が指定されていません".to_string());
    }

    let result = match operator {
        "add" => add(numbers),
        "sub" => sub(numbers),
        "mult" => mult(numbers),
        "div" => div(numbers),
        _ => return Err("無効な演算子です".to_string()),
    };

    Ok(result)
}

fn add(numbers: Vec<i32>) -> i32 {
    let mut result = numbers[0];
    for num in &numbers[1..] {
        result += num;
    }
    return result;
}

fn sub(numbers: Vec<i32>) -> i32 {
    let mut result = numbers[0];
    for num in &numbers[1..] {
        result -= num;
    }
    return result;
}

fn mult(numbers: Vec<i32>) -> i32 {
    let mut result = numbers[0];
    for num in &numbers[1..] {
        result *= num;
    }
    return result;
}

fn div(numbers: Vec<i32>) -> i32 {
    let mut result= numbers[0];
    for num in &numbers[1..] {
        result /= num;
    }
    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(vec![1, 2]), 3);
        assert_eq!(add(vec![-1, 2]), 1);
        assert_eq!(add(vec![1, 2, 3]), 6);
    }

    #[test]
    fn test_sub() {
        assert_eq!(sub(vec![2, 2]), 0);
        assert_eq!(sub(vec![1, 2]), -1);
        assert_eq!(sub(vec![1, 2, 3]), -4);
    }

    #[test]
    fn test_mult() {
        assert_eq!(mult(vec![3, 2]), 6);
        assert_eq!(mult(vec![-3, 2]), -6);
        assert_eq!(mult(vec![2, 2, 3]), 12);
    }

    #[test]
    fn test_div() {
        assert_eq!(div(vec![4, 2]), 2);
        assert_eq!(div(vec![-4, 2]), -2);
        assert_eq!(div(vec![12, 2, 2]), 3);
    }

    #[test]
    fn test_parse_operator() {
        assert_eq!(parse_operator("add"), Ok("add"));
        assert_eq!(parse_operator("sub"), Ok("sub")); 
        assert_eq!(parse_operator("mult"), Ok("mult"));
        assert_eq!(parse_operator("div"), Ok("div"));
        assert!(parse_operator("invalid").is_err());
    }

    #[test]
    fn test_parse_numbers() {
        let valid_nums = vec!["1".to_string(), "2".to_string(), "3".to_string()];
        assert_eq!(parse_numbers(&valid_nums), Ok(vec![1, 2, 3]));

        let negative_nums = vec!["-1".to_string(), "-2".to_string()];
        assert_eq!(parse_numbers(&negative_nums), Ok(vec![-1, -2]));

        let invalid_nums = vec!["1".to_string(), "abc".to_string()];
        assert!(parse_numbers(&invalid_nums).is_err());
    }
}
