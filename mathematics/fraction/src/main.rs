// ParialEqはテストのために追加
#[derive(Debug, PartialEq)]
struct Fraction {
    numerator: i32,
    denominator: i32,
}

impl Fraction {
    fn new(numerator: i32, denominator: i32) -> Self {
        if denominator == 0 {
            panic!("分母は0にできません");
        }

        // 符号を分子に統一
        // -1/2 と 1/-2 が違うものとして扱われないように表現
        let (numerator, denominator) = if denominator < 0 {
            (-numerator, -denominator)
        } else {
            (numerator, denominator)
        };

        Self {
            numerator,
            denominator,
        }
    }

    fn simplify(&self) -> Self {
        let gcd = gcd(self.numerator.abs(), self.denominator.abs());
        Fraction::new(self.numerator / gcd, self.denominator / gcd)
    }

    fn add(&self, other: &Self) -> Self {
        let num = self.numerator * other.denominator + other.numerator * self.denominator;
        let den = self.denominator * other.denominator;
        Fraction::new(num, den).simplify()
    }

    fn sub(&self, other: &Self) -> Self {
        let num = self.numerator * other.denominator - other.numerator * self.denominator;
        let den = self.denominator * other.denominator;
        Fraction::new(num, den).simplify()
    }

    fn mult(&self, other: &Self) -> Self {
        let num = self.numerator * other.numerator;
        let den = self.denominator * other.denominator;
        Fraction::new(num, den).simplify()
    }

    fn div(&self, other: &Self) -> Self {
        if other.numerator == 0 {
            panic!("0で割ることはできません");
        }
        let num = self.numerator * other.denominator;
        let den = self.denominator * other.numerator;
        Fraction::new(num, den).simplify()
    }
}

/// 最大公約数を求める
fn gcd(a: i32, b: i32) -> i32 {
    let mut a = a;
    let mut b = b;

    while b != 0 {
        let tmp = b;
        b = a % b;
        a = tmp;
    }
    a
}

fn main() {
    let f1 = Fraction::new(3, 12); // 3/12
    let f2 = Fraction::new(1, 2); // 1/2
    let f3 = Fraction::new(1, 3); // 1/3
 
    let f = f1.simplify();
    println!("{:?}", f); // 1/4

    let sum = f2.add(&f3);
    println!("{:?}", sum); // 5/6

    let diff = f2.sub(&f3);
    println!("{:?}", diff); // 1/6

    let product = f2.mult(&f3);
    println!("{:?}", product); // 1/6

    let quotient = f2.div(&f3);
    println!("{:?}", quotient); // 3/2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "分母は0にできません")]
    fn test_new_with_zero_denominator_panics() {
        Fraction::new(1, 0);
    }

    #[test]
    fn test_new_with_negative_denominator_normalizes_sign() {
        let f = Fraction::new(1, -2); // 1/-2
        let expect = Fraction::new(-1, 2); // -1/2
        assert_eq!(f, expect);
    }

    #[test]
    fn test_new_with_both_negative_makes_positive() {
        let f = Fraction::new(-1, -2); // -1/-2
        let expect = Fraction::new(1, 2); // 1/2
        assert_eq!(f, expect);
    }

    #[test]
    fn test_simplify() {
        let f1 = Fraction::new(3, 12); // 3/12
        let expect = Fraction::new(1, 4); // 1/4
        assert_eq!(f1.simplify(), expect);
    }

    #[test]
    fn test_add_positive_fractions() {
        let f1 = Fraction::new(3, 12); // 3/12
        let f2 = Fraction::new(2, 3); // 2/3
        let expect = Fraction::new(11, 12); // 11/12
        assert_eq!(f1.add(&f2), expect);
    }

    #[test]
    fn test_add_negative_fractions() {
        let f1 = Fraction::new(-1, 2); // -1/2
        let f2 = Fraction::new(-1, 3); // -1/3
        let expect = Fraction::new(-5, 6); // -5/6
        assert_eq!(f1.add(&f2), expect);
    }

    #[test]
    fn test_mult_positive_fractions() {
        let f1 = Fraction::new(3, 12); // 3/12
        let f2 = Fraction::new(2, 3); // 2/3
        let expect = Fraction::new(1, 6); // 1/6
        assert_eq!(f1.mult(&f2), expect);
    }

    #[test]
    fn test_mult_negative_fractions() {
        let f1 = Fraction::new(-1, 2); // -1/2
        let f2 = Fraction::new(-1, 3); // -1/3
        let expect = Fraction::new(1, 6); // 1/6
        assert_eq!(f1.mult(&f2), expect);
    }

    #[test]
    fn test_mult_by_zero() {
        let f1 = Fraction::new(3, 4); // 3/4
        let f2 = Fraction::new(0, 1); // 0/1 (0)
        let expect = Fraction::new(0, 1); // 0/1 (0)
        assert_eq!(f1.mult(&f2), expect);
    }

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(12, 8), 4);
        assert_eq!(gcd(1, 1), 1);
        assert_eq!(gcd(7, 3), 1);
    }

    #[test]
    fn test_sub_results_in_negative() {
        let f1 = Fraction::new(3, 12); // 3/12
        let f2 = Fraction::new(2, 3); // 2/3
        let expect = Fraction::new(-5, 12); // -5/12
        assert_eq!(f1.sub(&f2), expect);
    }

    #[test]
    fn test_sub_with_negative_denominator_normalizes_to_negative_numerator() {
        let f1 = Fraction::new(3, 12); // 3/12
        let f2 = Fraction::new(2, 3); // 2/3
        let expect = Fraction::new(5, -12); // 5/-12
        assert_eq!(f1.sub(&f2), expect);
    }

    #[test]
    fn test_sub_integer_result() {
        let f1 = Fraction::new(6, 3); // 6/3
        let f2 = Fraction::new(3, 3); // 2/3
        let expect = Fraction::new(1, 1); // 1/1 (1)
        assert_eq!(f1.sub(&f2), expect);
    }

    #[test]
    fn test_div_positive_fractions() {
        let f1 = Fraction::new(3, 12); // 3/12
        let f2 = Fraction::new(2, 3); // 2/3
        let expect = Fraction::new(3, 8); // 3/8
        assert_eq!(f1.div(&f2), expect);
    }

    #[test]
    fn test_div_results_in_integer() {
        let f1 = Fraction::new(4, 2); // 2
        let f2 = Fraction::new(1, 2); // 1/2
        let expect = Fraction::new(4, 1); // 4
        assert_eq!(f1.div(&f2), expect);
    }

    #[test]
    fn test_div_zero_numerator_by_fraction() {
        let f1 = Fraction::new(0, 1); // 0/1 (0)
        let f2 = Fraction::new(2, 3); // 2/3
        let expect = Fraction::new(0, 1); // 0 
        assert_eq!(f1.div(&f2), expect);
    }

    #[test]
    #[should_panic(expected = "0で割ることはできません")]
    fn test_div_by_zero_panics() {
        let f1 = Fraction::new(2, 3); // 2/3
        let f2 = Fraction::new(0, 1); // 0/1 (0)
        f1.div(&f2);
    }
}