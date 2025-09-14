struct Point { x: f64, y: f64 }

impl From<f64> for Point {
    fn from(input: f64) -> Self {
        Point { x: input, y: input }
    }
}

fn main() {
    let p1 = Point::from(1.0);
    let p2: Point = (1.0).into();
    println!("p1: {}, {}", p1.x, p1.y);
    println!("p2: {}, {}", p2.x, p2.y);
}
