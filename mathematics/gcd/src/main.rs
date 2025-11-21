/// 最大公約数をユークリッド互除法で求める
fn gcd(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        let tmp = b;
        b = a % b;
        a = tmp;
    }
    a
}

fn main() {
    println!("gcd(48, 18) = {}", gcd(48, 18));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd() {
        let expect = 6;
        assert_eq!(gcd(48, 18), expect);
    }
}
