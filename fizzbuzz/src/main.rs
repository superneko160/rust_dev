fn main() {
    (0..=100).for_each(|num| println!("{}", fizzbuzz(num)));
}

fn fizzbuzz(num: i32) -> String {
    if num == 0 {
        return num.to_string();
    }
    match (num % 3, num % 5) {
        (0, 0) => "FizzBuzz".to_string(),
        (0, _) => "Fizz".to_string(),
        (_, 0) => "Buzz".to_string(),
            _ => num.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fizzbuzz() {
        assert_eq!(fizzbuzz(1), "1".to_string());
        assert_eq!(fizzbuzz(3), "Fizz".to_string());
        assert_eq!(fizzbuzz(5), "Buzz".to_string());
        assert_eq!(fizzbuzz(15), "FizzBuzz".to_string());
        assert_eq!(fizzbuzz(-1), "-1".to_string());
        assert_eq!(fizzbuzz(-3), "Fizz".to_string());
        assert_eq!(fizzbuzz(-5), "Buzz".to_string());
        assert_eq!(fizzbuzz(-15), "FizzBuzz".to_string());
    }
}