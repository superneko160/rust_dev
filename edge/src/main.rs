// ABC240 - A Edge Checker
// 10角形のa番目の点とb番目の点が直接で結ばれているか否か

fn solve(a: i8, b: i8) -> bool {
    let a1 = a % 10;
    let b1 = b % 10;

    if (b1 - a1).abs() == 1 || (b - a).abs() == 1 {
        return true;
    }

    false
}

fn main() {
    let a = 4;
    let b = 5;

    if solve(a, b) {
        println!("Yes");
    } else {
        println!("No");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert!(solve(4, 5));
        assert!(solve(1, 10));
        assert_eq!(solve(3, 5), false);
        assert_eq!(solve(7, 10), false);
    }
}
