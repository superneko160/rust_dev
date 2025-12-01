enum QuadraticAnswer {
    TwoDistinctRealRoots(f64, f64),
    DoubleRoot(f64),
    TwoDistinctComplexRoots(f64, f64),
}

/// 二次方程式 ax^2 + bx + c = 0 の解を求める
fn solve_quadratic(a: f64, b: f64, c: f64) -> QuadraticAnswer {
    if a.abs() < f64::EPSILON {
        panic!("a = 0です!");
    }

    // 判別式: b^2 - 4ac
    let discriminant = b * b - 4.0 * a * c;

    if discriminant > 0.0 {
        // 2つの実数解
        let x1 = (-b + discriminant.sqrt()) / (2.0 * a); // 大きいほうの値
        let x2 = (-b - discriminant.sqrt()) / (2.0 * a); // 小さいほうの値
        QuadraticAnswer::TwoDistinctRealRoots(x1, x2)
    } else if discriminant == 0.0 {
        // 重解
        let x = -b / (2.0 * a);
        QuadraticAnswer::DoubleRoot(x)
    } else {
        // 複素数解
        let real = -b / (2.0 * a);
        let imag = (-discriminant).sqrt() / (2.0 * a);
        QuadraticAnswer::TwoDistinctComplexRoots(real, imag)
    }
}

/// 二次方程式の解を表示する
fn print_quadratic_answer(answer: QuadraticAnswer) {
    match answer {
        QuadraticAnswer::TwoDistinctRealRoots(x1, x2) => {
            println!("x = {}, {}", x1, x2);
        },
        QuadraticAnswer::DoubleRoot(x) => {
            println!("x = {}", x);
        },
        QuadraticAnswer::TwoDistinctComplexRoots(real, imag) => {
            println!("x = {}±{}i", real, imag);
        },
    }
}

fn main() {
    // x^2 - 5x + 6 = 0
    // x = 3, 2
    print_quadratic_answer(solve_quadratic(1.0, -5.0, 6.0));

    // x^2 - 4x + 4 = 0
    // x = 2
    print_quadratic_answer(solve_quadratic(1.0, -4.0, 4.0));

    // x^2 + 2x + 5 = 0
    // x = -1±2i 
    print_quadratic_answer(solve_quadratic(1.0, 2.0, 5.0));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_distinct_real_roots() {
        // x^2 - 5x + 6 = 0
        // x = 2, 3
        match solve_quadratic(1.0, -5.0, 6.0) {
            QuadraticAnswer::TwoDistinctRealRoots(x1, x2) => {
                let expect_x1 = 3.0;
                let expect_x2 = 2.0;
                // 浮動小数点は計算誤差があるためassert_eq!ではなく
                // assert!と許容誤差を使う
                assert!((expect_x1 - x1).abs() < 1e-10);
                assert!((expect_x2 - x2).abs() < 1e-10);
            },
            _ => panic!("予期せぬ結果"),
        }
    }

    #[test]
    fn test_double_root() {
        // x^2 - 4x + 4 = 0
        // x = 2
        match solve_quadratic(1.0, -4.0, 4.0) {
            QuadraticAnswer::DoubleRoot(x) => {
                let expect_x = 2.0;
                assert!((expect_x - x).abs() < 1e-10);
            },
            _ => panic!("予期せぬ結果"),
        }
    }

    #[test]
    fn test_two_distinct_complex_roots() {
        // x^2 + 2x + 5 = 0
        // x = -1±2i 
        match solve_quadratic(1.0, 2.0, 5.0) {
            QuadraticAnswer::TwoDistinctComplexRoots(real, imag) => {
                let expect_real = -1.0;
                let expect_imag = 2.0;
                assert!((expect_real - real).abs() < 1e-10);
                assert!((expect_imag - imag).abs() < 1e-10);
            },
            _ => panic!("予期せぬ結果"),
        }
    }

    #[test]
    #[should_panic(expected = "a = 0です!")]
    fn test_linear_function() {
        solve_quadratic(0.0, 2.0, 5.0);
    }
}
