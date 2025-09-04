use std::f64::consts::PI;

enum ShapeType {
    Rectangle { width: f64, height: f64 },
    Circle { radius: f64 },
}

// --- プロダクト ---
trait Shape {
    fn get_area(&self) -> f64;
}
// --- プロダクト ---

// --- 具象プロダクト ---
struct Rectangle {
    width: f64,
    height: f64,
}

impl Shape for Rectangle {
    fn get_area(&self) -> f64 {
        self.width * self.height
    }
}

struct Circle {
    radius: f64
}

impl Shape for Circle {
    fn get_area(&self) -> f64 {
        self.radius * self.radius * PI
    }
}
// --- 具象プロダクト ---

// --- クリエイタ ---
struct ShapeFactory;

impl ShapeFactory {
    fn new_shape(s: ShapeType) -> Box<dyn Shape> {
        match s {
            ShapeType::Circle { radius } => Box::new(
                Circle { radius }
            ),
            ShapeType::Rectangle { width, height } => Box::new(
                Rectangle { width, height }
            ),
        }
    }
}
// --- クリエイタ ---

fn main() {
    let circle = ShapeFactory::new_shape(
        ShapeType::Circle { radius: 5.0 }
    );
    println!("Circle Area: {}", circle.get_area());

    let rectangle = ShapeFactory::new_shape(
        ShapeType::Rectangle { width: 10.0, height: 5.0 }
    );
    println!("Rectangle Area: {}", rectangle.get_area());
}
