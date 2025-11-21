/// 順列
/// nPr = n (n-1)(n-2)...(n-r+1)
/// ex) P(5, 3) = 5 * 4 * 3 = 60
fn permutation(n: i32, r: i32) -> i32 {
    let mut result = 1;
    for i in (n - r + 1..=n).rev() {
        result *= i;
    }
    result
}

/// 階乗
/// n! = nPn 
/// ex) F(3) = 3 * 2 * 1 = 6
fn factorial(n: i32) -> i32 {
    let mut result = 1;
    // 1は計算しなくてもいいので2スタート
    for i in 2..=n {
        result *= i
    }
    result
}

/// 組み合わせ
/// nCr = nPr / r!
/// ex) C(4, 3) = 4P3 / 3! = 24 / 6 = 4 
fn combination(n: i32, r: i32) -> i32 {
    permutation(n, r) / factorial(r)
}

fn main() {
    let p = permutation(5, 3);
    println!("5P3 = {}", p);
    let f = factorial(5);
    println!("5! = {}", f);
    let c = combination(4, 3);
    println!("4C3 = {}", c);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_permutation() {
        // 4P3
        // 4 * 3 * 2 = 24
        let expect = 24;
        assert_eq!(permutation(4, 3), expect);
    }

    #[test]
    fn test_factorial() {
        // 4!
        // 4 * 3 * 2 * 1 = 24
        let expect = 24;
        assert_eq!(factorial(4), expect);
    }

    #[test]
    fn test_combination() {
        // 8C4 
        // 8 * 7 * 6 * 5 / 4 * 3 * 2 * 1 = 1680 / 24 = 70
        let expect = 70;
        assert_eq!(combination(8, 4), expect);
    }
}
