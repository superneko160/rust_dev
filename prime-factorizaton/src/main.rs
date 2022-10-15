use std::env;

/**
 * コマンドライン引数で渡された正の整数を最小の素数に分解
 * ex) cargo run 100 -> [2, 2, 5, 5]
 */
fn main() {
    // コマンドライン引数を取得
    let args: Vec<String> = env::args().collect();
    // 数値に変換
    let n: u64 = match args[1].trim().parse() {
        Ok(n) => n,
        Err(_e) => 1,
    };
    // 素因数分解
    if n > 1 {
        println!("{:?}", prime_factorize(n));
    }
    println!("2以上の正の整数を入力してください");
}

fn prime_factorize(n: u64) -> Vec<u64> {
    let mut result = n;
    for i in 2..((n as f64).sqrt() as u64)+1 {
        if n % i == 0 {
            result = i;
            break;
        }
    }
    if result == n {
        return vec![n];
    }
    else {
        let mut v1 = vec![result];
        let mut v2 = prime_factorize(n / result);
        v1.append(&mut v2);
        return v1;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prime_factorize() {
        let result: Vec<u64> = vec![2, 2, 5, 5];
        assert_eq!(result, prime_factorize(100));
    }
}