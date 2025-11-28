/// 座標
/// PartialEqはテストで比較するために実装
#[derive(Debug, PartialEq)]
struct Point {
    x: f64,
    y: f64,
}

/// 2点間の距離を求める
fn distance(point1: &Point, point2: &Point) -> f64 {
    let dx = point2.x - point1.x;
    let dy = point2.y - point1.y;
    let squared_sum = (dx * dx) + (dy * dy);
    squared_sum.sqrt()
}

/// 中点（1:1に内分する座標）を求める
fn midpoint(point1: &Point, point2: &Point) -> Point {
    Point {
        x: (point1.x + point2.x) / 2.0,
        y: (point1.y + point2.y) / 2.0,
    }
}

/// 内分点（m:nに内分する座標）を求める
fn internal_division_point(point1: &Point, point2: &Point, m: f64, n: f64) -> Point {
    Point {
        x: (n * point1.x + m * point2.x) / (m + n),
        y: (n * point1.y + m * point2.y) / (m + n),
    }
}

/// 外分点（m:nに外分する座標）を求める
fn external_division_point(point1: &Point, point2: &Point, m: f64, n: f64) -> Option<Point> {
    if (m - n).abs() < f64::EPSILON { // 浮動小数点の比較では == での比較は避ける
        return None;  // m == n の場合は計算不可
    }
    Some(Point {
        x: (-n * point1.x + m * point2.x) / (m - n),
        y: (-n * point1.y + m * point2.y) / (m - n),
    })
}

fn main() {
    let p1 = Point {
        x: -3.0,
        y: 2.0,
    };
    let p2 = Point {
        x: 4.0,
        y: 5.0,
    };
    println!("{:.2}", distance(&p1, &p2));
    println!("{:?}", midpoint(&p1, &p2));
    println!("{:?}", internal_division_point(&p1, &p2, 2.0, 3.0));
    println!("{:?}", external_division_point(&p1, &p2, 2.0, 3.0).unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distance1() {
        let p1 = Point {
            x: 0.0,
            y: 2.0,
        };
        let p2 = Point {
            x: 3.0,
            y: 6.0,
        };
        let expect = 5.0;
        assert_eq!(distance(&p1, &p2), expect);
    }

    #[test]
    fn test_distance2() {
        let p1 = Point {
            x: -2.0,
            y: 1.0,
        };
        let p2 = Point {
            x: 2.0,
            y: 5.0,
        };
        let expect = 5.656854249492381;
        assert_eq!(distance(&p1, &p2), expect);
    }

    #[test]
    fn test_distance3() {
        let p1 = Point {
            x: -4.0,
            y: -1.0,
        };
        let p2 = Point {
            x: 1.0,
            y: -4.0,
        };
        let expect = 5.830951894845301; 
        assert_eq!(distance(&p1, &p2), expect);
    }

    #[test]
    fn test_midpoint1() {
        let p1 = Point {
            x: 1.0,
            y: 2.0,
        };
        let p2 = Point {
            x: 3.0,
            y: 4.0,
        };
        let expect = Point {
            x: 2.0,
            y: 3.0,
        };
        assert_eq!(midpoint(&p1, &p2), expect);
    }

    #[test]
    fn test_midpoint2() {
        let p1 = Point {
            x: 1.5,
            y: 2.3,
        };
        let p2 = Point {
            x: 3.7,
            y: 4.9,
        };
        let expect = Point {
            x: 2.6,
            y: 3.6,
        };
        assert_eq!(midpoint(&p1, &p2), expect);
    }

    #[test]
    fn test_internal_division_point1() {
        let p1 = Point {
            x: 10.0,
            y: 15.0,
        };
        let p2 = Point {
            x: 40.0,
            y: 45.0,
        };
        let m = 2.0;
        let n = 1.0;
        let expect = Point {
            x: 30.0,
            y: 35.0,
        };
        assert_eq!(internal_division_point(&p1, &p2, m, n), expect);
    }

    #[test]
    fn test_internal_division_point2() {
        let p1 = Point {
            x: 0.0,
            y: 5.0,
        };
        let p2 = Point {
            x: 50.0,
            y: 30.0,
        };
        let m = 3.0;
        let n = 2.0;
        let expect = Point {
            x: 30.0,
            y: 20.0,
        };
        assert_eq!(internal_division_point(&p1, &p2, m, n), expect);
    }

    #[test]
    fn test_external_division_point_normal() {
        let point1 = Point {
            x: 0.0,
            y: 0.0
        };
        let point2 = Point {
            x: 10.0,
            y: 10.0
        };
        // 正常系: m != n
        let m = 2.0;
        let n = 1.0;
        let result = external_division_point(&point1, &point2, m, n);
        assert!(result.is_some());
        let point = result.unwrap();
        let expect_point_x = 20.0;
        let expect_point_y = 20.0;
        assert_eq!(point.x, expect_point_x);
        assert_eq!(point.y, expect_point_y);
    }

    #[test]
    fn test_external_division_point_returns_none() {
        let point1 = Point {
            x: 5.0,
            y: 10.0
        };
        let point2 = Point {
            x: 15.0,
            y: 20.0
        };
        // 異常系: m = n 
        let m = 3.0;
        let n = 3.0;
        let result = external_division_point(&point1, &point2, m, n);
        assert!(result.is_none());
    }
}
