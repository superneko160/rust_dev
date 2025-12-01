// PargialEqはテストのために追加
#[derive(Debug, PartialEq)]
struct Fraction {
    numerator: i32,
    denominator: i32,
}

impl Fraction {
    fn new(numerator: i32, denominator: i32) -> Self {
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

    fn multiply(&self, other: &Self) -> Self {
        let num = self.numerator * other.numerator;
        let den = self.denominator * other.denominator;
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

    let product = f2.multiply(&f3);
    println!("{:?}", product); // 1/6
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simplyfy() {
        let f = Fraction::new(3, 12); // 3/12
        let expect = Fraction::new(1, 4); // 1/4
        assert_eq!(f.simplify(), expect);
    }

    #[test]
    fn test_add() {
        let f1 = Fraction::new(3, 12); // 3/12
        let f2 = Fraction::new(2, 3); // 2/3
        let expect = Fraction::new(11, 12); // 11/12
        assert_eq!(f1.add(&f2), expect);
    }

    #[test]
    fn test_multiply() {
        let f1 = Fraction::new(3, 12); // 3/12
        let f2 = Fraction::new(2, 3); // 2/3
        let expect = Fraction::new(1, 6); // 1/6
        assert_eq!(f1.multiply(&f2), expect);
    }
}
